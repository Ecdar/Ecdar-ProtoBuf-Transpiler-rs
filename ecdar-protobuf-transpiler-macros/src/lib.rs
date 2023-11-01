use std::fs;

use proc_macro::{token_stream, TokenStream};


#[proc_macro]
pub fn get_services(_: TokenStream) -> TokenStream{
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let services_file = format!("{out_dir}/services.txt");
    let services = fs::read_to_string(&services_file).unwrap();


    services.parse().unwrap()
}

