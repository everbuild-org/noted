use std::collections::HashMap;
use lightningcss::printer::PrinterOptions;
use lightningcss::rules::CssRule;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use lightningcss::traits::ToCss;
use quote::quote;

const CSS_FILE: &'static str = include_str!("../../data/lucide/lucide.css");

#[proc_macro]
pub fn lucide_icons(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let stylesheet = StyleSheet::parse(CSS_FILE, ParserOptions::default()).unwrap();
    let mut icons: HashMap<String, String> = HashMap::new();

    for rule in stylesheet.rules.0 {
        match rule {
            CssRule::Style(rule) => {
                if rule.selectors.0.len() > 1 {
                    continue;
                }

                let selector = rule.selectors.0[0].clone().to_css_string(PrinterOptions::default()).unwrap();
                let selector = selector.replace(".", "");
                let selector = selector.replace("icon-", "lucide_");
                let selector = selector.replace("-", "_");
                let selector = selector.replace(":before", "");

                // get the content property
                for decl in rule.declarations.declarations {
                    if decl.property_id().name() == "content" {
                        let content = decl.value_to_css_string(PrinterOptions::default()).unwrap();
                        icons.insert(selector.clone(), content);
                    }
                }
            }
            _ => {}
        }
    }

    fn icon_function(name: &str, content: &str) -> proc_macro2::TokenStream {
        let content = content.trim_matches('"');
        let content = content.trim_matches('\"');

        let name = syn::Ident::new(name, proc_macro2::Span::call_site());
        let content = syn::LitStr::new(content, proc_macro2::Span::call_site());

        quote! {
            pub fn #name() -> String {
                #content.to_string()
            }
        }
    }

    let mut tokens = proc_macro2::TokenStream::new();
    for (name, content) in icons {
        let function = icon_function(&name, &content);
        tokens.extend(function);
    }

    tokens.into()
}
