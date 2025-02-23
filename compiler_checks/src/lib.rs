extern crate impls;

use const_format::formatcp;
use impls::impls;
use proc_macro::TokenStream;
use quote::quote;
use static_assertions::assert_impl_all;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, ItemTrait, Token, TraitBound, Type};

struct TypeTraitInput {
    ty: Ident,
    _comma: Token![,],
    trait_bound: TraitBound,
}

macro_rules! conditionally_expand {
    {
        true,
        $fragment:item
    } => {
        $fragment
    };
    {
        false,
        $fragment:item
    } => {
    };
}

macro_rules! fail_on {
    ($l:tt) => {
        conditionally_expand!($l, compile_error!("Error: failed to compile"))
    };
}

macro_rules! check_trait_impl {
    ($ty:ty, $thetrait:path) => {
         {
    let output = impls::impls!($ty: $thetrait);
            if !output {
                conditionally_expand!(true, quote!{compile_error!("Error: failed to compile")});
            }
        };
    };
}

impl Parse for TypeTraitInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(TypeTraitInput {
            ty: input.parse()?,
            _comma: input.parse()?,
            trait_bound: input.parse()?,
        })
    }
}

macro_rules! assert_type_from_type {
    ($x:ty, $y:ty) => {
        const fn check() -> () {
    const DOES_IMPL: bool = impls::impls!($x: From<$y>);

            if !DOES_IMPL {
                const MESSAGE: &str = formatcp!(
                    "`{}` does not implement `From<{}>`.\n Consider adding #[extendr] to `{}`",
                    stringify!($x),
                    stringify!($y),
                    stringify!($y)
                );
                panic!("{}", MESSAGE);
            }
        };
        const _: () = check();
    };
}

macro_rules! assert_type_tryfrom_type {
    ($x:ty, $y:ty) => {
        const fn check() -> () {
    const DOES_IMPL: bool = impls::impls!($x: TryFrom<$y>);

            if !DOES_IMPL {
                const MESSAGE: &str = formatcp!(
                    "`{}` does not implement `TryFrom<{}>`.\n Consider adding #[extendr] to `{}`",
                    stringify!($x),
                    stringify!($y),
                    stringify!($y)
                );
                panic!("{}", MESSAGE);
            }
        };
        const _: () = check();
    };
}

fn test_this() {
    struct Foo;
    struct Bar;
    impl From<Bar> for Foo {
        fn from(_value: Bar) -> Self {
            Self
        }
    }
    assert_type_from_type!(Foo, Bar);
}

// #[proc_macro]
// pub fn check_impl(input: TokenStream) -> TokenStream {
//     let input_copy = input.clone();
//     let TypeTraitInput {
//         ty,
//         _comma,
//         trait_bound,
//     } = parse_macro_input!(input as TypeTraitInput);

//     let trait_path = &trait_bound.path;
//     let implements_trait = {
//                   /// Fallback trait with `False` for `IMPLS` if the type does not
//                   /// implement the given trait.
//                   trait DoesNotImpl {
//                       const IMPLS: bool = false;
//                   }
//                   impl<T: ?Sized> DoesNotImpl for T {}
//                   /// Concrete type with `True` for `IMPLS` if the type implements the
//                   /// given trait. Otherwise, it falls back to `DoesNotImpl`.
//                   struct Wrapper<T: ?Sized>(::impls::_core::marker::PhantomData<T>);
//                   #[allow(dead_code)]
//                   impl<T: ?Sized + #trait_bound> Wrapper<T> {
//                       const IMPLS: bool = true;
//                   }
//                   <Wrapper<String>>::IMPLS
//     }
//     let implements_trait = quote!{ impls::impls!(#input) };
//     let implements_trait = implements_trait
//     if !implements_trait {
//         return quote! {
//             compile_error!(concat!(
//                 "Type ", stringify!(#ty), " does not implement trait ", stringify!(#trait_path)
//             ));
//         }
//         .into();
//     }

//     quote! {}.into()
// }

// use impls::impls;
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse::Parse, parse_macro_input, parse_quote, ExprPath, Ident, Item, Token, Type};

// /// Parses input as `check_trait_impl!(Type, Trait);`
// struct TraitCheckInput {
//     type_ident: syn::Type,
//     _comma: Token![,],
//     trait_ident: Ident,
// }

// #[derive(Debug)]
// struct MissingTrait;

// impl std::fmt::Display for MissingTrait {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Detected a missing trait");
//         Ok(())
//     }
// }

// impl std::error::Error for MissingTrait {}

// impl Parse for TraitCheckInput {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let type_ident = input.parse()?;
//         let _comma: Token![,] = input.parse()?;
//         let trait_ident = input.parse()?;
//         Ok(Self {
//             type_ident,
//             _comma,
//             trait_ident,
//         })
//     }
// }

// #[proc_macro]
// pub fn check_trait_impl(input: TokenStream) -> TokenStream {
//     let TraitCheckInput {
//         type_ident,
//         _comma,
//         trait_ident,
//     } = parse_macro_input!(input as TraitCheckInput);

//     let error_message = format!(
//         "ERROR: `{}` must implement `{}` to be used here.",
//         quote!(#type_ident),
//         quote!(#trait_ident)
//     );
//     // let type_ident: syn::token::Type = type_ident.into();
//     // let type_ident = type_ident;
//     let type_ident: Type = parse_quote!(#type_ident);
//     let has_trait = impls!(#type_ident: #trait_ident);
//     let expanded = quote! {
//         const HASTRAIT: bool = impls::impls!(#type_ident : #trait_ident);
//         if !HASTRAIT {
//           std::compile_error!{ #error_message };
//         }
//     };
//     expanded.into()
// }
//

// fn test_this2() {
//     struct Foo;
//     check_trait_impl!(String, Copy);
//     assert_impl_all!(String: Copy);
// }

// mod test {
//     use impls::impls;

//     use super::check_trait_impl;

//     #[test]
//     fn trait_generic() {
//         assert!(impls::impls!(i32: Copy));
//         struct Foo;
//         assert_eq!(impls::impls!(Foo: Copy), false);
//         struct Bar;
//         trait Baz {}
//         impl Baz for Bar {}
//         assert!(impls!(Bar: Baz));

//         check_trait_impl(Foo: Copy);
//     }
// }
