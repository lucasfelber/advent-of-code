// pub mod args;
// pub mod handler;
// pub mod types;

use proc_macro::TokenStream;
use syn::{meta::ParseNestedMeta, parse_macro_input, Ident, ItemFn, LitInt};

#[derive(Default)]
struct Date {
    year: Option<LitInt>,
    day: Option<LitInt>,
}

impl Date {
    fn parse(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if meta.path.is_ident("year") {
            self.year = meta.value()?.parse()?;
            Ok(())
        } else if meta.path.is_ident("day") {
            self.day = meta.value()?.parse()?;
            Ok(())
        } else {
            Err(meta.error("unsupported date attribute"))
        }
    }
}

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut attrs = Date::default();
    let parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with parser);

    let year = attrs
        .year
        .expect("year required")
        .base10_parse::<i32>()
        .expect("could not parse year");
    let day = attrs
        .day
        .expect("day required")
        .base10_parse::<u32>()
        .expect("could not parse day");

    let input_path = format!("../../data/inputs/{}-{:02}.in", year, day);

    let mut solution = parse_macro_input!(input as ItemFn);
    solution.sig.ident = Ident::new("solution", solution.sig.ident.span());

    let tokens = quote::quote! {
        const INPUT: &str = include_str!(#input_path);
        #solution
        fn main() {
            let now = std::time::Instant::now();
            let (part1, part2) = solution(INPUT.trim_end());
            let elapsed = now.elapsed();


            println!("-------------\n\
                      | {} , {:02} |\n\
                      -------------", #year, #day);
            match part1 {
                Some(result) => println!("Part one: {}", result),
                _ => println!("Part one: None"),
            }
            match part2 {
                Some(result) => println!("Part two: {}", result),
                _ => println!("Part two: None"),
            }

            if elapsed.as_millis() > 0 {
                println!("Time: {}ms", elapsed.as_millis());
              } else {
                println!("Time: {}μs", elapsed.as_micros());
              }
        }
    };

    // println!("args: {}-{}", &year, &day);
    // let args = parse_macro_input!(args with Punctuated::<LitInt, Token![,]>::parse_terminated);

    // let input_path = format!("../../inputs/{}-{}", args.first().unwrap().token(), args.last().unwrap().token());

    // println!("{input_path}");
    // println!("input: {}", input);
    // let (year, day) = parse_macro_input!(args as LitInt);
    TokenStream::from(tokens)
}
