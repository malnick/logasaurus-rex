#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_yaml;
#[macro_use]
extern crate serde_json;

pub mod config;
pub mod query;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {}
}
