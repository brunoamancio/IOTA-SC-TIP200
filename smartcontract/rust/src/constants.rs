#![allow(dead_code)]

use iota_sc_hname_generator::generate_schname;
use wasmlib::ScHname;

pub const SC_NAME : &str  = "erc_721";
pub const HNAME_SC_NAME : ScHname  = generate_schname!("erc_721");
