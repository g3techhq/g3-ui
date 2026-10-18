//! Static props extracted from component signatures and rustdoc at build time.
use super::*;

#[derive(Clone, Copy, PartialEq)]
pub enum DocPart {
    Text(&'static str),
    Code(&'static str),
}

pub struct VariantReference {
    pub value: &'static str,
    pub description: &'static [DocPart],
}

pub struct PropReference {
    pub name: &'static str,
    pub ty: &'static str,
    pub required: bool,
    pub description: &'static [DocPart],
    pub variants: &'static [VariantReference],
}

pub struct ComponentReference {
    pub name: &'static str,
    pub generics: &'static str,
    pub props: &'static [PropReference],
}

include!(concat!(env!("OUT_DIR"), "/component_props.rs"));

#[component]
fn DocText(parts: &'static [DocPart]) -> Element {
    rsx! {
        for part in parts {
            match part {
                DocPart::Text(text) => rsx! { "{text}" },
                DocPart::Code(code) => rsx! { code { "{code}" } },
            }
        }
    }
}

/// The props of each component a demo shows, one g3 [`Table`] per
/// component. A phone scrolls the table sideways with the prop names pinned.
#[component]
pub fn PropsReference(components: Vec<&'static str>) -> Element {
    rsx! {
        if !components.is_empty() {
            section { class: "component-props", aria_label: "Component props",
                h2 { "Props" }
                for name in components {
                    if let Some(component) = COMPONENTS.iter().find(|component| component.name == name) {
                        g3_ui::Table {
                            key: "{name}",
                            caption: format!("{}{}", component.name, component.generics),
                            fill: true,
                            wrap: true,
                            sticky_first_column: true,
                            class: "component-props-table",
                            thead { tr {
                                th { scope: "col", "Prop" }
                                th { scope: "col", "Type" }
                                th { scope: "col", "Description" }
                            } }
                            tbody {
                                for prop in component.props {
                                    tr { key: "{prop.name}",
                                        th { scope: "row",
                                            code { "{prop.name}" }
                                            if prop.required {
                                                g3_ui::Badge { color: g3_ui::Color::Accent, class: "component-props-required", "Required" }
                                            }
                                        }
                                        td {
                                            code { "{prop.ty}" }
                                            if !prop.variants.is_empty() {
                                                details { class: "component-props-variants",
                                                    summary { "{prop.variants.len()} variants" }
                                                    dl {
                                                        for variant in prop.variants {
                                                            dt { code { "{variant.value}" } }
                                                            dd { DocText { parts: variant.description } }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        td { DocText { parts: prop.description } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "../props_model.rs"]
mod parser_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_gallery_component_has_generated_props_with_descriptions() {
        for demo in component_playground_demos() {
            for name in demo.components {
                // Some galleries also list hooks alongside their components.
                if name.starts_with("use_") {
                    continue;
                }
                let reference = COMPONENTS
                    .iter()
                    .find(|component| component.name == *name)
                    .unwrap_or_else(|| panic!("missing props for {name}"));
                assert!(!reference.props.is_empty(), "{name}");
                for prop in reference.props {
                    assert!(!prop.description.is_empty(), "{name}.{}", prop.name);
                    assert!(
                        !prop
                            .description
                            .contains(&DocPart::Text("No description provided.")),
                        "missing documentation for {name}.{}",
                        prop.name
                    );
                }
            }
        }
        let button = COMPONENTS
            .iter()
            .find(|component| component.name == "Button")
            .unwrap();
        assert!(
            !button
                .props
                .iter()
                .find(|prop| prop.name == "children")
                .unwrap()
                .required
        );
        let sheet = COMPONENTS
            .iter()
            .find(|component| component.name == "BottomSheet")
            .unwrap();
        assert!(
            sheet
                .props
                .iter()
                .find(|prop| prop.name == "open")
                .unwrap()
                .required
        );
    }
}
