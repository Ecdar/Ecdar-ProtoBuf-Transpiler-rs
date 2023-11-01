use std::fs;

use proc_macro::{TokenStream};


#[proc_macro]
pub fn get_services(_: TokenStream) -> TokenStream{
    let out_dir = env!("OUT_DIR");
    let services_file = format!("{out_dir}/services.rs");
    let services = fs::read_to_string(&services_file).unwrap();
    services.parse().unwrap()
}

