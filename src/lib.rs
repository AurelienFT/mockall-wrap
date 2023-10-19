#[proc_macro_attribute]
pub fn wrap(attrs: proc_macro::TokenStream, input: proc_macro::TokenStream)
    -> proc_macro::TokenStream
{
    let attrs: proc_macro2::TokenStream = attrs.into();
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #attrs
        #input
    };
    output.into()
}