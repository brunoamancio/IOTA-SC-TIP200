mod constants;

use iota_sc_utils::wasmlib::ScExports;

#[no_mangle]
fn on_load() {
    let _exports = ScExports::new();
    
}