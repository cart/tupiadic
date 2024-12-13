use proc_macro::{Span, TokenStream};
use proc_macro2::Literal;
use quote::{format_ident, quote};

#[proc_macro]
pub fn tupiadic_impl(input: TokenStream) -> TokenStream {
    let Some(proc_macro::TokenTree::Literal(literal)) = input.into_iter().next() else {
        panic!("Expected a single integer input");
    };
    let count = literal
        .to_string()
        .parse::<usize>()
        .expect("Expected a single integer input");

    let mut impls = TokenStream::new();
    let mut total_impls = 0;
    for i in 1..=count {
        tupiadic_count(&mut impls, i, &mut total_impls);
    }

    println!("TOTAL IMPLS: {total_impls}");
    impls
}

/// Implements Variadic for a given tuple count
fn tupiadic_count(impls: &mut TokenStream, count: usize, total_impls: &mut usize) {
    let generics = (1..=count).map(|i| format_ident!("T{}", i));
    let generics2 = generics.clone();
    let generics3 = generics.clone();
    let generics4 = generics.clone();

    // TupleSlice impls
    if count >= 2 {
        // TupleSlice indices start from the right of a tuple. When we reach the count impl, it resolves in the same way as
        // a (T1, T2) tuple (the "leaf", which returns normal left index 0 right index 1 values). 
        for i in 0..(count-2) {
            *total_impls += 1;
            let generics = generics.clone();
            let generics2 = generics.clone();
            let generics3 = generics.clone();
            let generics4 = generics.clone();
            let generics5 = generics.clone();
            let index = Literal::usize_unsuffixed(i);
            let index_plus_1 = Literal::usize_unsuffixed(i+1);
            let right_index = Literal::usize_unsuffixed(count - i - 1);
            let right_generic = format_ident!("T{}", count - i);
            
            impls.extend(TokenStream::from(quote! {
                impl<#(#generics,)*> Variadic for TupleSlice<(#(#generics2,)*), #index> {
                    type Left = TupleSlice<(#(#generics3,)*), #index_plus_1>;
                    type Right = #right_generic;
                
                    fn left(&self) -> &Self::Left {
                        unsafe {
                            &*(self as *const TupleSlice<(#(#generics4,)*), #index> as *const TupleSlice<(#(#generics5,)*), #index_plus_1>)
                        }
                    }
                
                    fn right(&self) -> &Self::Right {
                        &self.0 .#right_index
                    }
                }
            }));
        }

        let generics = generics.clone();
        let generics2 = generics.clone();
        let right_index = Literal::usize_unsuffixed(count - 2);
        *total_impls += 1;
        impls.extend(TokenStream::from(quote! {
            impl<#(#generics,)*> Variadic for TupleSlice<(#(#generics2,)*), #right_index> {
                type Left = T1;
                type Right = T2;
            
                fn left(&self) -> &Self::Left {
                    &self.0 .0
                }
            
                fn right(&self) -> &Self::Right {
                    &self.0 .1
                }
            }
        }));
    }

    // Tuple Impls
    // Only implement Variadic for actual tuples >= 3. Lower impls are manually implemented for macro clarity + compile times.
    if count >= 3 {
        let last = format_ident!("T{}", count);
        let last_index = Literal::usize_unsuffixed(count - 1);

        // SAFETY: the cast from the tuple pointer to the TupleSlice pointer is safe because TupleSlice has the same
        // memory layout (just wraps a tuple of the same type, and it has #[repr(transparent)] ).
        *total_impls += 1;
        impls.extend(TokenStream::from(quote! {
            impl <#(#generics,)*> Variadic for (#(#generics2,)*) {
                type Left = TupleSlice<(#(#generics3,)*), 1>;
                type Right = #last;

                fn left(&self) -> &Self::Left {
                    unsafe { &*(self as *const Self as *const TupleSlice<(#(#generics4,)*), 1>) }
                }

                fn right(&self) -> &Self::Right {
                    &self.#last_index
                }
            }
        }))
    }
}
