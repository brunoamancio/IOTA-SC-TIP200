use wasmlib::*;

macro_rules! add_impl_pub_setter_fns {
    ($set_func:ident, $get_func:ident, $param_type:ty) => {
        /// Tries to set result. Does nothing if it can't find it.
        pub fn $set_func<TContext : ScBaseContext>(param_name : &str, param_value : $param_type, ctx : &TContext) {
            ctx.results().$get_func(param_name).set_value(param_value);
        }
    };
}

// ---------------------------    Setter functions    -------------------------------------

// Primitive types
add_impl_pub_setter_fns!(set_string, get_string, &str);
add_impl_pub_setter_fns!(set_int64, get_int64, i64);
add_impl_pub_setter_fns!(set_bytes, get_bytes, &[u8]);
/// Tries to set result. Does nothing if it can't find it.
pub fn set_bool<TContext : ScBaseContext>(param_name : &str, param_value : bool, ctx: &TContext) {
    let bool_value = to_u8(param_value);
    ctx.results().get_bytes(param_name).set_value(&bool_value);
}
/// converts a boolean value into a vector of u8
fn to_u8(param_value : bool) -> Vec<u8> {
    let mut value : Vec<u8> = Vec::new();
    if param_value {
        value.push(1)
    } else {
        value.push(0)
    }
    value
}


// ISCP Types
add_impl_pub_setter_fns!(set_agent_id, get_agent_id, &ScAgentId);
add_impl_pub_setter_fns!(set_address, get_address, &ScAddress);
add_impl_pub_setter_fns!(set_request_id, get_request_id, &ScRequestId);
add_impl_pub_setter_fns!(set_hname, get_hname, ScHname);
add_impl_pub_setter_fns!(set_hash, get_hash, &ScHash);
add_impl_pub_setter_fns!(set_contract_id, get_contract_id, &ScContractId);
add_impl_pub_setter_fns!(set_color, get_color, &ScColor);
add_impl_pub_setter_fns!(set_chain_id, get_chain_id, &ScChainId);

// ---------------------------    Getter functions    -------------------------------------

macro_rules! add_impl_pub_getter_fns {
    ($get_func:ident, $return_type:ty) => {
        pub fn $get_func(param_name : &str, immutablemap : ScImmutableMap) -> $return_type {
            immutablemap.$get_func(param_name).value()
        }
    }
}

// Primitive types
add_impl_pub_getter_fns!(get_string, String);
add_impl_pub_getter_fns!(get_int64, i64);
add_impl_pub_getter_fns!(get_bytes, Vec<u8>);

pub fn get_bool(param_name : &str, immutablemap : ScImmutableMap) -> bool {
    let param_value = immutablemap.get_bytes(param_name).value();
    let bool_value = to_bool(param_value);
    bool_value
}

fn to_bool(bytes_vector : Vec<u8>) -> bool {
    match bytes_vector.get(0) {
        Some(byte) => *byte == 1,
        None => panic!("Could not convert byte to bool")
    }
}

// ISCP Types
add_impl_pub_getter_fns!(get_agent_id, ScAgentId);
add_impl_pub_getter_fns!(get_address, ScAddress);
add_impl_pub_getter_fns!(get_request_id, ScRequestId);
add_impl_pub_getter_fns!(get_hname, ScHname);
add_impl_pub_getter_fns!(get_hash, ScHash);
add_impl_pub_getter_fns!(get_contract_id, ScContractId);
add_impl_pub_getter_fns!(get_color, ScColor);
add_impl_pub_getter_fns!(get_chain_id, ScChainId);