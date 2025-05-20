use proc_macro::{TokenStream, TokenTree};
use quote::ToTokens;
use syn;

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr is {:?}", attr);
    println!("func is {:?}", item);
    for token in attr.into_iter() {
        println!("token is {:?}", token);
        match token {
            TokenTree::Ident(_) => println!("method is {}", token.span().source_text().unwrap()),
            TokenTree::Literal(_) => println!("path is {}", token.span().source_text().unwrap()),
            _ => println!("others is {}", token.span().source_text().unwrap())
        }
    }
    
    return item;
}