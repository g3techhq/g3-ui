//! Shared by the build script and parser regression tests.
use pulldown_cmark::{Event, Parser, TagEnd};
use quote::ToTokens;
use std::collections::BTreeMap;
use std::fmt::Write;
use syn::{Attribute, Expr, FnArg, Item, Lit, Meta, Pat, Type, Visibility};

#[derive(Clone, Debug, PartialEq)]
pub enum DocPart {
    Text(String),
    Code(String),
}

#[derive(Clone, Debug)]
pub struct Variant {
    pub value: String,
    pub description: Vec<DocPart>,
}

#[derive(Debug)]
pub struct Prop {
    pub name: String,
    pub ty: String,
    pub required: bool,
    pub description: Vec<DocPart>,
    pub variants: Vec<Variant>,
}

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub generics: String,
    pub props: Vec<Prop>,
}

fn docs(attributes: &[Attribute]) -> String {
    attributes
        .iter()
        .filter_map(|attribute| {
            if !attribute.path().is_ident("doc") {
                return None;
            }
            let Meta::NameValue(value) = &attribute.meta else {
                return None;
            };
            let Expr::Lit(value) = &value.value else {
                return None;
            };
            let Lit::Str(value) = &value.lit else {
                return None;
            };
            Some(value.value().trim().to_string())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Rustdoc links keep their label; code spans stay code. No HTML is emitted.
pub fn doc_parts(markdown: &str) -> Vec<DocPart> {
    let mut parts = Vec::new();
    let parser = Parser::new_with_broken_link_callback(
        markdown,
        pulldown_cmark::Options::empty(),
        Some(|link: pulldown_cmark::BrokenLink<'_>| {
            Some(("".into(), link.reference.into_string().into()))
        }),
    );
    for event in parser {
        match event {
            Event::Text(text) => parts.push(DocPart::Text(text.into_string())),
            Event::Code(code) => parts.push(DocPart::Code(code.into_string())),
            Event::SoftBreak | Event::HardBreak | Event::End(TagEnd::Paragraph) => {
                parts.push(DocPart::Text(" ".into()));
            }
            _ => {}
        }
    }
    parts
}

fn rust(tokens: impl ToTokens) -> String {
    tokens
        .to_token_stream()
        .to_string()
        .replace(" :: ", "::")
        .replace(" < ", "<")
        .replace("< ", "<")
        .replace(" >", ">")
        .replace(" ,", ",")
        .replace(" : ", ": ")
        .replace("& ", "&")
}

fn enum_for_type<'a>(
    ty: &Type,
    enums: &'a BTreeMap<String, Vec<Variant>>,
) -> Option<&'a Vec<Variant>> {
    let Type::Path(path) = ty else {
        return None;
    };
    let last = path.path.segments.last()?;
    if let Some(variants) = enums.get(&last.ident.to_string()) {
        return Some(variants);
    }
    if let syn::PathArguments::AngleBracketed(arguments) = &last.arguments {
        for argument in &arguments.args {
            if let syn::GenericArgument::Type(ty) = argument
                && let Some(variants) = enum_for_type(ty, enums)
            {
                return Some(variants);
            }
        }
    }
    None
}

pub fn components(files: &[syn::File]) -> Vec<Component> {
    let mut enums = BTreeMap::new();
    for item in files.iter().flat_map(|file| &file.items) {
        if let Item::Enum(item) = item
            && matches!(item.vis, Visibility::Public(_))
        {
            enums.insert(
                item.ident.to_string(),
                item.variants
                    .iter()
                    .map(|variant| Variant {
                        value: format!("{}{}", variant.ident, rust(&variant.fields)),
                        description: doc_parts(&docs(&variant.attrs)),
                    })
                    .collect(),
            );
        }
    }
    let mut components = Vec::new();
    for item in files.iter().flat_map(|file| &file.items) {
        let Item::Fn(function) = item else {
            continue;
        };
        if !matches!(function.vis, Visibility::Public(_))
            || !function
                .attrs
                .iter()
                .any(|attribute| attribute.path().is_ident("component"))
        {
            continue;
        }
        let mut props = Vec::new();
        for argument in &function.sig.inputs {
            let FnArg::Typed(argument) = argument else {
                continue;
            };
            let Pat::Ident(pattern) = &*argument.pat else {
                panic!("component prop must be an identifier");
            };
            let name = pattern
                .ident
                .to_string()
                .trim_start_matches("r#")
                .to_string();
            let mut defaulted = name == "children";
            let mut ignore_option = false;
            let mut skip = false;
            let mut extends = None;
            for attribute in &argument.attrs {
                if !attribute.path().is_ident("props") {
                    continue;
                }
                let options = attribute
                    .parse_args_with(
                        syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                    )
                    .expect("valid props attributes");
                for option in options {
                    let path = option.path();
                    defaulted |= path.is_ident("default") || path.is_ident("default_code");
                    ignore_option |= path.is_ident("ignore_option");
                    skip |= path.is_ident("skip");
                    if path.is_ident("extends") {
                        defaulted = true;
                        if let Meta::NameValue(value) = option {
                            extends = Some(rust(value.value));
                        }
                    }
                }
            }
            if skip {
                continue;
            }
            let optional = !ignore_option
                && matches!(&*argument.ty, Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "Option"));
            let mut description = docs(&argument.attrs);
            if description.is_empty() {
                description = if name == "children" {
                    "Body content. Defaults to empty.".into()
                } else if let Some(element) = extends {
                    format!("Additional `{element}` HTML attributes forwarded to the element.")
                } else {
                    "No description provided.".into()
                };
            }
            props.push(Prop {
                name,
                ty: rust(&argument.ty),
                required: !defaulted && !optional,
                description: doc_parts(&description),
                variants: enum_for_type(&argument.ty, &enums)
                    .cloned()
                    .unwrap_or_default(),
            });
        }
        components.push(Component {
            name: function.sig.ident.to_string(),
            generics: rust(&function.sig.generics),
            props,
        });
    }
    components.sort_by(|a, b| a.name.cmp(&b.name));
    components
}

fn write_docs(output: &mut String, parts: &[DocPart]) {
    output.push_str("&[");
    for part in parts {
        match part {
            DocPart::Text(text) => write!(output, "DocPart::Text({text:?}),").unwrap(),
            DocPart::Code(code) => write!(output, "DocPart::Code({code:?}),").unwrap(),
        }
    }
    output.push(']');
}

pub fn generate(components: &[Component]) -> String {
    let mut output = String::from("pub static COMPONENTS: &[ComponentReference] = &[\n");
    for component in components {
        write!(
            output,
            "ComponentReference {{ name: {:?}, generics: {:?}, props: &[",
            component.name, component.generics
        )
        .unwrap();
        for prop in &component.props {
            write!(
                output,
                "PropReference {{ name: {:?}, ty: {:?}, required: {}, description: ",
                prop.name, prop.ty, prop.required
            )
            .unwrap();
            write_docs(&mut output, &prop.description);
            output.push_str(", variants: &[");
            for variant in &prop.variants {
                write!(
                    output,
                    "VariantReference {{ value: {:?}, description: ",
                    variant.value
                )
                .unwrap();
                write_docs(&mut output, &variant.description);
                output.push_str("},");
            }
            output.push_str("] },\n");
        }
        output.push_str("] },\n");
    }
    output.push_str("];\n");
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_generic_props_docs_attributes_and_required_state() {
        let source = syn::parse_file(
            r#"
            pub enum Fill { /// No background.
                Clear, /// Minimum column width.
                Fit(f64) }
            #[component]
            pub fn Sample<T: Clone>(
                /// Selected values, including commas and `code`.
                mut value: Signal<(T, T)>,
                /// Uses [`Fill::Clear`].
                fill: Option<Fill>,
                #[props(default = false)] disabled: bool,
                #[props(default, into)] to: Destination,
                #[props(extends = input)] attributes: Vec<Attribute>,
                children: Element,
            ) -> Element { todo!() }
        "#,
        )
        .unwrap();
        let model = components(&[source]);
        let props = &model[0].props;
        assert_eq!(model[0].generics, "<T: Clone>");
        assert_eq!(props[0].ty, "Signal<(T, T)>");
        assert!(props[0].required);
        assert!(props[1..].iter().all(|prop| !prop.required));
        assert_eq!(props[1].variants[1].value, "Fit(f64)");
        assert!(
            props[1]
                .description
                .contains(&DocPart::Code("Fill::Clear".into()))
        );
        assert!(generate(&model).contains("ComponentReference"));
    }

    #[test]
    fn rustdoc_links_keep_code_labels_without_destinations_or_raw_markup() {
        let parts = doc_parts(
            "Defaults to [`Strings::cancel`](crate::Strings::cancel).\nUse **translated** labels.",
        );
        assert!(parts.contains(&DocPart::Code("Strings::cancel".into())));
        let text = parts
            .iter()
            .map(|part| match part {
                DocPart::Text(text) | DocPart::Code(text) => text.as_str(),
            })
            .collect::<String>();
        assert_eq!(
            text.trim(),
            "Defaults to Strings::cancel. Use translated labels."
        );
    }
}
