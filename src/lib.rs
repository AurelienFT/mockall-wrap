use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemTrait, TraitItem};

fn generated_wrap(input: &proc_macro2::TokenStream, trait_has_mut: bool) -> TokenStream {
    let trait_block = syn::parse::<ItemTrait>(input.clone().into()).unwrap();
    let trait_name = trait_block.ident;
    let mock_name = syn::Ident::new(&format!("Mock{}", trait_name), trait_name.span());
    let struct_name = syn::Ident::new(&format!("Mock{}Wrapper", trait_name), trait_name.span());
    let inner_type = match trait_has_mut {
        true => quote! { Arc<RwLock<#mock_name>> },
        false => quote! { Arc<#mock_name> },
    };
    let init_code = match trait_has_mut {
        true => quote! { Arc::new(RwLock::new(#mock_name::new())) },
        false => quote! { Arc::new(#mock_name::new()) },
    };
    let set_expectations = match trait_has_mut {
        true => quote! {
            pub fn set_expectations<F: FnOnce(&mut #mock_name)>(&mut self, g: F) {
                g(&mut Arc::get_mut(&mut self.inner).unwrap().write().unwrap())
            }
        },
        false => quote! {
            pub fn set_expectations<F: FnOnce(&mut #mock_name)>(&mut self, g: F) {
                g(&mut Arc::get_mut(&mut self.inner).unwrap())
            }
        },
    };
    let mut mock_methods = Vec::new();
    for method in trait_block.items {
        if let TraitItem::Fn(function) = method {
            let signature = function.sig.clone();
            let name = signature.ident.clone();
            let param_names = signature.inputs.iter().map(|param| match param {
                syn::FnArg::Typed(pat_type) => {
                    let pat = pat_type.pat.clone();
                    Some(quote! { #pat })
                }
                _ => None,
            });
            let param_names = param_names.flatten().collect::<Vec<TokenStream>>();
            let self_type = function
                .sig
                .inputs
                .first()
                .and_then(|first_param| match first_param {
                    syn::FnArg::Receiver(re) => Some(re.mutability.is_some()),
                    _ => None,
                });
            let function_code = if function.sig.ident == "clone_box" {
                quote! {Box::new(self.clone())}
            } else {
                match (self_type, trait_has_mut) {
                    (None, _) => {
                        quote! {
                            #name(#(#param_names),*)
                        }
                    }
                    (Some(true), true) => {
                        quote! {
                            self.inner.write().unwrap().#name(#(#param_names),*)
                        }
                    }
                    (Some(false), true) => {
                        quote! {
                            self.inner.read().unwrap().#name(#(#param_names),*)
                        }
                    }
                    (Some(_), false) => quote! {
                        self.inner.#name(#(#param_names),*)
                    },
                }
            };
            mock_methods.push(quote! {
                #signature {
                    #function_code
                }
            });
        }
    }
    quote! {
            pub struct #struct_name {
                inner: #inner_type,
            }

            impl Clone for #struct_name {
                fn clone(&self) -> Self {
                    Self {
                        inner: self.inner.clone(),
                    }
                }
            }

            impl #struct_name {
                pub fn new() -> Self {
                    Self {
                        inner: #init_code,
                    }
                }

                #set_expectations
            }

            impl #trait_name for #struct_name {
                #(#mock_methods)*
            }
    }
}

fn check_trait_has_mut(input: &proc_macro2::TokenStream) -> bool {
    let trait_block = syn::parse::<ItemTrait>(input.clone().into()).unwrap();
    for method in trait_block.items {
        if let TraitItem::Fn(function) = method {
            let self_type = function
                .sig
                .inputs
                .first()
                .and_then(|first_param| match first_param {
                    syn::FnArg::Receiver(re) => Some(re.mutability.is_some()),
                    _ => None,
                });
            if let Some(true) = self_type {
                return true;
            }
        }
    }
    false
}

#[proc_macro_attribute]
pub fn wrap(
    attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attrs: proc_macro2::TokenStream = attrs.into();
    let input: proc_macro2::TokenStream = input.into();
    let trait_has_mut: bool = check_trait_has_mut(&input);
    let generated_wrap = generated_wrap(&input, trait_has_mut);
    let output = quote::quote! {
        #attrs
        #input
        #generated_wrap
    };
    output.into()
}
