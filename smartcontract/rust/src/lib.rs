mod constants;

use wasmlib::*;
use iota_sc_utils::*;

#[no_mangle]
fn on_load() {
    let _exports = ScExports::new();
    _exports.add_func(constants::MY_DUMMY_FN, my_dummy_fn)
}

fn my_dummy_fn(ctx : &ScFuncContext) {
    let hname_external_contract : ScHname = iota_sc_hname_generator::generate_schname!("external_contract");
    let hname_implements : ScHname = iota_sc_hname_generator::generate_schname!("implements");
    let hname_erc165 : ScHname = iota_sc_hname_generator::generate_schname!("tip_100");
    let input_params = params::new();
    params::add_hname("hname_tip_100", hname_erc165, &input_params);

    let result = ctx.call(hname_external_contract, hname_implements, Some(input_params), None);
    let implements = results::get_bool("implements", result);
    if implements {

    }

    let _supported = ctx.supports(0x1);
}

trait Selector {
    const HASH_ERC165 : u32 = iota_sc_hname_generator::calculate_hash!("tip_100");

    fn supports(self, hname_interface : u32) -> bool;
}

impl Selector for &ScFuncContext {
    fn supports(self, hname_interface : u32) -> bool {
        hname_interface == Self::HASH_ERC165
    }
}