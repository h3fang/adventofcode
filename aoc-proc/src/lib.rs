use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{ToTokens, quote};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Expr, Token};

#[proc_macro]
pub fn aoc_event(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
    let days = parser.parse(input).unwrap();
    let days = days.iter().collect::<Vec<_>>();
    let mods = days
        .iter()
        .map(|day| {
            let name = format!("day{}", day.to_token_stream());
            Ident::new(&name, Span::call_site())
        })
        .collect::<Vec<_>>();

    let output = quote! {
        use std::time::{Duration, Instant};

        #( mod #mods; )*

        fn run_day(day: &str) {
            #(
                if stringify!(#days) == day {
                    println!("day {}:", day);
                    let start = Instant::now();
                    #mods::main();
                    let dt =  Instant::now() - start;
                    println!("time: {:?}\n", dt);
                    return;
                }
            )*
            panic!("invalid day: {}", day);
        }

        fn run_all() {
            let mut total = Duration::default();
            #(
                println!("day {}:", #days);
                let start = Instant::now();
                #mods::main();
                let dt =  Instant::now() - start;
                println!("time: {:?}\n", dt);
                total += dt;
            )*
            println!("total: {:?}", total);
        }
    };

    output.into()
}
