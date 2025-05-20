use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn;

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    println!("ast is {:?}", ast.to_token_stream());
    let name = &ast.ident;
    println!("name is {}", name);
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    println!("gen into is {:}", gen);
    gen.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}