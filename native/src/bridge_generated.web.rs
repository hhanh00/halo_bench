use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_test_from_seed(port_: MessagePort, seed: u64) {
    wire_test_from_seed_impl(port_, seed)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: impl Wire2Api for JsValue

impl Wire2Api<u64> for JsValue {
    fn wire2api(self) -> u64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
