use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hallo_macro_derive(input: TokenStream) -> TokenStream{
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    //build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name{
            fn hello_macro(){
                println!("Hello, Macor! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}
