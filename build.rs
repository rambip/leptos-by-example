use std::fs;
use std::fs::{File, read_dir};
use std::io;
use std::io::Write;
use std::path::Path;
use quote::quote;
use proc_macro2::{TokenStream, Ident, Span};

use serde::Deserialize;

use syntect::{
    highlighting::ThemeSet, 
    html::highlighted_html_for_string, 
    parsing::SyntaxSet
};

/// pre-process a code snippet to add html 
/// syntax-highlighting
fn highlight(code: &str) -> String {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let theme = &ts.themes["base16-ocean.light"];
    highlighted_html_for_string(code, &ps, syntax, &theme).unwrap()
}

/// the `example.toml` representation
#[derive(Debug, Deserialize)]
struct Info {
    description: String,
    motivation: String,
    related: Option<String>,
}

fn extract_toml_info(file_name: &str) -> std::result::Result<Info, toml::de::Error> {
    let raw_toml_info = fs::read(format!("examples/{file_name}.toml"))
        .expect("please create examples/{file_name}.toml to provide some documentation");
    let toml_info = String::from_utf8_lossy(&raw_toml_info);
    toml::from_str(&toml_info)
}

fn quote_option(text: Option<String>) -> TokenStream {
    match text {
        Some(x) => quote!{Some(#x)},
        None => quote!{None}
    }
}

fn html_from_markdown(file_name: &str, input: String) -> String {
    use pulldown_cmark::{Tag, Event};
    let parser = pulldown_cmark::Parser::new(&input);

    let checked_parser = parser
        .map(|x| match x {
            Event::Start(Tag::Heading(_,_,_)) => panic!(
                "{file_name}.toml: headings are not allowed in this field"
            ),
            _ => x,
    });

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, checked_parser);
    html_output
}

/// reads the `example` directory.
/// For each `foo.rs`, it will read it,
/// preprocess for syntax-highlighting,
/// read and parse corresponding `foo.toml` metadata
/// and eventually load `foo.css`
fn read_examples(path: &Path, 
                 includes: &mut TokenStream, 
                 examples: &mut TokenStream, 
                 n_examples: &mut usize) -> Result<(), io::Error>{
    for f in read_dir(path)? {
        let f = f?;
        let meta = f.metadata()?;
        if meta.is_file() && f.path().extension().unwrap()=="rs" {
            let file_name = f.path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let raw_css = fs::read(format!("examples/{file_name}.css")).unwrap_or(Vec::new());
            let css = String::from_utf8_lossy(&raw_css);

            let info = match extract_toml_info(&file_name) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("please provide all the required fields in {file_name}.toml");
                    eprintln!("The missing field is {:?}", e.message());
                    panic!()
                }
            };

            let description = info.description;

            let motivation = html_from_markdown(&file_name, info.motivation);
            let related=quote_option(
                info.related.map(|x| html_from_markdown(&file_name, x))
            );

            format!("examples/{file_name}.css");

            let example_name = Ident::new(&file_name, Span::call_site());
            let relative_path = format!("../examples/{file_name}.rs");


            let highlighted_source = highlight(
                std::str::from_utf8(
                    &fs::read(f.path())?
                ).unwrap()
            );

            examples.extend(
                quote!{
                    (
                        #file_name, 
                        Example {
                        highlighted_source: #highlighted_source,
                        code: pack_example(#example_name::showcase),
                        css: stylist::style!(#css).unwrap(),
                        description: #description,
                        motivation: #motivation,
                        related: #related,
                    }
                    ),
                }
            );

            includes.extend(
                quote!{
                    mod #example_name {
                        include!(#relative_path);
                    }
                }
            );

            *n_examples += 1;
        }
    };
    Ok(())
}

fn main() -> Result<(), io::Error> {

    let mut includes = TokenStream::new();
    let mut examples = TokenStream::new();

    let mut n_examples = 0usize;

    read_examples(Path::new("./examples"),
                  &mut includes,
                  &mut examples,
                  &mut n_examples)?;


    let generated_rust = quote!{
        //! generated automatically by build.rs

        #includes

        use super::{Example, pack_example};

        pub const N_EXAMPLES: usize = #n_examples;
        pub type Examples = std::collections::HashMap<&'static str, Example>;

        pub fn examples() -> Examples {
            [
                #examples
            ]
            .into_iter()
            .collect()
        }
    };

    let pretty = prettyplease::unparse(&syn::parse2(generated_rust).unwrap());

    File::create("src/examples.rs")?
        .write_all(pretty.as_bytes())?;

    Ok(())

}
