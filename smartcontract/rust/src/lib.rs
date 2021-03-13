use wasmlib::*;
//use iota_sc_utils::*;

mod constants;

#[no_mangle]
fn on_load() {
    let _exports = ScExports::new();
}