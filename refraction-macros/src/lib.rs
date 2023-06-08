use proc_macro::TokenStream;

#[proc_macro]
pub fn assign_some(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let args : Vec<&str> = input.split(",").collect();
    format!("if let Some(k) = {}.get({}) {{ {} = k.to_string() ; }} ", args[0], args[1], args[2]).parse().unwrap()
}
