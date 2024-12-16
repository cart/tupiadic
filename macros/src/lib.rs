use proc_macro::TokenStream;
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
    for i in 2..=count {
        tupiadic_count(&mut impls, i);
    }
    impls
}

/// Implements Variadic for a given tuple count
fn tupiadic_count(impls: &mut TokenStream, count: usize) {
    let generics = (1..=count).map(|i| format_ident!("T{}", i));
    let generics1 = generics.clone();
    let generics2 = generics.clone();
    let generics3 = generics.clone();
    let generics4 = generics.clone();
    let generics5 = generics.clone();
    let indices = (3..=count).map(|i| Literal::usize_unsuffixed(i - 1));
    let indices1 = indices.clone();
    let indices2 = indices.clone();

    impls.extend(TokenStream::from(quote! {
        impl<M: VariadicMarker, #(#generics: VariadicMut<M>,)*> VariadicMut<M> for (#(#generics1,)*) {
            fn visit(&mut self, input: &mut M::Input) -> M::Output {
                let o = M::fold(self.0.visit(input), self.1.visit(input));
                #(let o = M::fold(o, self.#indices.visit(input));)*
                o
            }
        }
        
        impl<M: VariadicMarker, #(#generics2: VariadicRef<M>,)*> VariadicRef<M> for (#(#generics3,)*) {
            fn visit(&self, input: &mut M::Input) -> M::Output {
                let o = M::fold(self.0.visit(input), self.1.visit(input));
                #(let o = M::fold(o, self.#indices1.visit(input));)*
                o
            }
        }
        
        impl<M: VariadicMarker, #(#generics4: VariadicOwned<M>,)*> VariadicOwned<M> for (#(#generics5,)*) {
            fn visit(self, input: &mut M::Input) -> M::Output {
                let o = M::fold(self.0.visit(input), self.1.visit(input));
                #(let o = M::fold(o, self.#indices2.visit(input));)*
                o
            }
        }
    }));
}
