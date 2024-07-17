use std::mem;

use base64::{prelude::BASE64_STANDARD as B64_ENGINE, Engine};
use macro_railroad::{diagram, lowering, parser};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, ItemMacro, LitStr};

/// Internal attribute intented for use by the [cifg] crate only.
///
/// Adds a railroad expansion diagram to the cifg macro's doc comments.
#[proc_macro_attribute]
pub fn gen_rr_diag(args: TokenStream, macro_item: TokenStream) -> TokenStream {
    let diag_ref_key = parse_macro_input!(args as LitStr).value();
    let mut cifg_macro = parse_macro_input!(macro_item as ItemMacro);

    // Strip attributes off of the macro_rules def; the parser doesn't handle them.
    let mut attrs = mem::take(&mut cifg_macro.attrs);

    // Parse the macro_rules def into an intermediate form.
    let mut parsed_macro = parser::parse(&cifg_macro.to_token_stream().to_string())
        .map(lowering::MacroRules::from)
        .expect("Failed to parse macro_rules.");
    parsed_macro.foldcommontails();
    parsed_macro.normalize();

    const WITH_LEGEND: bool = true;

    // // Create a diagram from the intermediate form.
    let mut macro_diag = diagram::into_diagram(parsed_macro, WITH_LEGEND);
    // Adds CSS from 'railroad' crate.
    macro_diag.add_default_css();
    // Adds CSS from the 'macro_railroad' crate; colors and stuff.
    diagram::add_default_css(&mut macro_diag);

    // // Base64 encode so the diagram can be embeded into the md docs.
    let base64_diag = B64_ENGINE.encode(macro_diag.to_string());
    let base64_md_ref = format!("[{diag_ref_key}]: data:image/svg+xml;base64,{base64_diag}");
    let doc_str = LitStr::new(&base64_md_ref, Span::mixed_site());

    // // Push a new doc string with the diagram.
    attrs.push(parse_quote! {
        #[doc = #doc_str]
    });
    cifg_macro.attrs = attrs;

    quote!(#cifg_macro).into()
}
