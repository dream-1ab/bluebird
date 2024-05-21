
use std::{fs::File, io::Read};

use proc_macro::TokenStream;

#[proc_macro]
pub fn hello_my_macro(input: TokenStream) -> TokenStream {
    let count = input.to_string().parse::<u16>().unwrap();
    let source_code = (0..count).into_iter().map(|number| format!("
    pub fn add_{}(a: u8, b: u8) -> u8 {{
        a + b
    }}
    ", number)).reduce(|previous, next| format!("{}\n{}", previous, next)).unwrap();

    source_code.parse().unwrap()
}

#[proc_macro]
pub fn my_embed_source_code(input: TokenStream) -> TokenStream {
    let mut file = File::open(input.to_string().replace("\"", "")).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer.parse().unwrap()
}

