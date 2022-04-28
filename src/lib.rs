#![feature(mixed_integer_ops)]

use hmac::{Hmac, digest::InvalidLength};
use sha2::Sha256;

#[macro_use]
extern crate lazy_static;
extern crate proc_macros;
#[macro_use]
extern crate logging;

pub mod config;

// Generally useful type aliases
pub type HmacSha256 = Hmac<Sha256>;