use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
  let fun = parse_macro_input!(item as ItemFn);
  let mut cmd_fun = fun.clone();
  cmd_fun.sig.ident = format_ident!("{}", attr.to_string());


  quote!{
    #[allow(dead_code)]
    #fun

    #[tauri::command]
    #[allow(non_snake_case)]
    #cmd_fun
  }.into()
}
