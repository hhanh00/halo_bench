use anyhow::anyhow;
use bip39::{Language, Mnemonic, Seed};
use orchard::builder::Builder as OrchardBuilder;
use orchard::bundle::Flags;
use orchard::circuit::ProvingKey;
use orchard::keys::{FullViewingKey, Scope, SpendAuthorizingKey, SpendingKey};
use orchard::note::Nullifier;
use orchard::tree::MerkleHashOrchard;
use orchard::value::NoteValue;
use orchard::{Address, Anchor, Bundle};
use std::slice;
use zcash_primitives::consensus::{BlockHeight, BranchId, MainNetwork};
use zcash_primitives::transaction::components::Amount;
use zcash_primitives::transaction::sighash::{signature_hash, SignableInput};
use zcash_primitives::transaction::txid::TxIdDigester;
use zcash_primitives::transaction::{Transaction, TransactionData, TxVersion};

use crate::tx_types::Tx;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaChaRng;

pub fn build_tx(tx: &Tx, seed_phrase: &str, rng_seed: u32, pk: &ProvingKey) -> Vec<u8> {
    let network = &MainNetwork;
    let mnemonic = Mnemonic::from_phrase(seed_phrase, Language::English).unwrap();
    let seed = Seed::new(&mnemonic, "");
    let sk = SpendingKey::from_zip32_seed(seed.as_bytes(), 133, 0).unwrap();

    let fvk = FullViewingKey::from(&sk);
    let ovk = fvk.clone().to_ovk(Scope::External);

    let mut seed = [0u8; 32];
    seed[0..4].copy_from_slice(&rng_seed.to_be_bytes());
    let mut rng = ChaChaRng::from_seed(seed);

    let anchor: Anchor = MerkleHashOrchard::from_bytes(&tx.orchard_anchor)
        .unwrap()
        .into();
    let mut orchard_builder = OrchardBuilder::new(Flags::from_parts(true, true), anchor);

    for spend in tx.spends.iter() {
        let diversifier = orchard::keys::Diversifier::from_bytes(spend.diversifier);
        let address = fvk.address(diversifier, Scope::External);
        let value = NoteValue::from_raw(spend.amount);
        let rho = Nullifier::from_bytes(&spend.rho).unwrap();
        let rseed = orchard::note::RandomSeed::from_bytes(spend.rseed, &rho).unwrap();
        let note = orchard::Note::from_parts(address, value, rho, rseed).unwrap();
        let auth_path: Vec<_> = spend
            .witness
            .iter()
            .map(|h| MerkleHashOrchard::from_bytes(&h).unwrap())
            .collect();
        let position = spend.position;
        let merkle_path =
            orchard::tree::MerklePath::from_parts(position, auth_path.try_into().unwrap());
        orchard_builder
            .add_spend(fvk.clone(), note, merkle_path)
            .map_err(|e| anyhow!(e.to_string()))
            .unwrap();
    }

    for output in tx.outputs.iter() {
        let mut address = [0u8; 43];
        address.copy_from_slice(&output.orchard);
        let orchard_address = Address::from_raw_address_bytes(&address).unwrap();
        let mut memo = [0u8; 512];
        memo[0..output.memo.len()].copy_from_slice(&output.memo);
        orchard_builder
            .add_recipient(
                Some(ovk.clone()),
                orchard_address,
                NoteValue::from_raw(output.amount),
                Some(memo),
            )
            .map_err(|_| anyhow!("Orchard::add_recipient"))
            .unwrap();
    }
    let orchard_bundle: Bundle<_, Amount> = orchard_builder.build(&mut rng).unwrap();

    let consensus_branch_id =
        BranchId::for_height(network, BlockHeight::from_u32(tx.anchor_height));
    let version = TxVersion::suggested_for_branch(consensus_branch_id);

    let unauthed_tx: TransactionData<zcash_primitives::transaction::Unauthorized> =
        TransactionData::from_parts(
            version,
            consensus_branch_id,
            0,
            BlockHeight::from_u32(tx.expiry_height),
            None,
            None,
            None,
            Some(orchard_bundle),
        );
    let txid_parts = unauthed_tx.digest(TxIdDigester);
    let sig_hash = signature_hash(&unauthed_tx, &SignableInput::Shielded, &txid_parts);
    let sig_hash: [u8; 32] = sig_hash.as_ref().clone();

    let orchard_bundle = unauthed_tx.orchard_bundle().map(|ob| {
        let proven = ob.clone().create_proof(&pk, &mut rng).unwrap();
        proven
            .apply_signatures(
                &mut rng,
                sig_hash,
                slice::from_ref(&SpendAuthorizingKey::from(&sk)),
            )
            .unwrap()
    });

    let tx_data: TransactionData<zcash_primitives::transaction::Authorized> =
        TransactionData::from_parts(
            version,
            consensus_branch_id,
            0,
            BlockHeight::from_u32(tx.expiry_height),
            None,
            None,
            None,
            orchard_bundle,
        );
    let tx = Transaction::from_data(tx_data).unwrap();
    let mut tx_bytes = vec![];
    tx.write(&mut tx_bytes).unwrap();
    tx_bytes
}
