use anyhow::anyhow;
use incrementalmerkletree::Hashable;
use lazy_static::lazy_static;
use orchard::builder::Builder;
use orchard::bundle::Flags;
use orchard::circuit::{ProvingKey, VerifyingKey};
use orchard::keys::{FullViewingKey, Scope, SpendAuthorizingKey, SpendingKey};
use orchard::tree::MerkleHashOrchard;
use orchard::value::NoteValue;
use orchard::Bundle;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaChaRng;

lazy_static! {
    static ref PK: ProvingKey = ProvingKey::build();
    static ref VK: VerifyingKey = VerifyingKey::build();
}

pub fn test_from_seed(seed: u64) -> anyhow::Result<()> {
    let mut seed_bytes = [0u8; 32];
    seed_bytes[0..8].copy_from_slice(&seed.to_be_bytes());
    let mut rng = ChaChaRng::from_seed(seed_bytes);

    let sk = SpendingKey::from_bytes([0; 32]).unwrap();
    let fvk = FullViewingKey::from(&sk);
    let recipient = fvk.address_at(0u32, Scope::External);

    let anchor = MerkleHashOrchard::empty_root(32.into()).into();
    let mut builder = Builder::new(Flags::from_parts(false, true), anchor);
    builder
        .add_recipient(None, recipient, NoteValue::from_raw(1000), None)
        .map_err(|e| anyhow!(e))?;
    let unauthorized = builder.build(&mut rng).unwrap();
    let sighash = unauthorized.commitment().into();
    let proven = unauthorized.create_proof(&PK, &mut rng).unwrap();
    let authorized: Bundle<_, i64> = proven
        .apply_signatures(&mut rng, sighash, &[SpendAuthorizingKey::from(&sk)])
        .map_err(|_| anyhow!("Error in apply_signatures"))?;

    authorized.verify_proof(&VK)?;
    println!("verified {}", seed);
    Ok(())
}
