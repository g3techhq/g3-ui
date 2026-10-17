//! Shared presentation for guide snippets and component source panels.
use super::*;

#[component]
pub fn CodeBlock(src: String, #[props(default = Language::Rust)] language: Language) -> Element {
    let label = match language {
        Language::Rust => "Rust",
        Language::Toml => "TOML",
        _ => language.slug(),
    };
    rsx! {
        section { class: "playground-code-block", aria_label: "{label} code example",
            div { class: "playground-code-heading", "{label}" }
            Code {
                src: SourceCode::new(language, src),
                theme: CodeTheme::system(Theme::GITHUB_LIGHT, Theme::GITHUB_DARK),
            }
        }
    }
}
