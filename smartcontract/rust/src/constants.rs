#![allow(dead_code)]
use iota_sc_utils::wasmlib::ScHname;
use iota_sc_utils::generator::generate_schname;

pub const SC_NAME : &str  = "erc_721";
pub const HNAME_SC_NAME : ScHname  = generate_schname!("erc_721");
