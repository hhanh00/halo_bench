use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_test_from_seed(port_: MessagePort, seed: u32) {
    wire_test_from_seed_impl(port_, seed)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: impl Wire2Api for JsValue

impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
