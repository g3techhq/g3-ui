//! Avatars.
use crate::theme::{classes, merge_classes};
use dioxus::prelude::*;

/// [`Avatar`] size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum AvatarSize {
    /// 28px, for dense rows and mentions.
    Sm,
    /// 36px.
    #[default]
    Md,
    /// 48px, for profile headers.
    Lg,
}

/// A round image of a person or team, with initials when there is no image.
/// Like Ionic's `ion-avatar`.
///
/// `name` is the accessible name. Pass an empty `name` for a decorative
/// avatar next to text that already names the person.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # struct Player { photo_url: String }
/// # let player = Player { photo_url: String::new() };
/// rsx! { Avatar { name: "Alex Morgan", initials: "AM", src: player.photo_url } }
/// # }
/// ```
#[component]
pub fn Avatar(
    /// Who the avatar shows; its accessible name.
    name: String,
    /// Image URL.
    src: Option<String>,
    /// Text shown without an image. Defaults to the first letters of `name`.
    initials: Option<String>,
    /// Size. Defaults to [`AvatarSize::Md`].
    size: Option<AvatarSize>,
    /// Extra classes for the avatar.
    class: Option<String>,
) -> Element {
    let size_cls = match size.unwrap_or_default() {
        AvatarSize::Sm => "g3-avatar-sm",
        AvatarSize::Md => "g3-avatar-md",
        AvatarSize::Lg => "g3-avatar-lg",
    };
    let cls = merge_classes(classes(["g3-avatar", size_cls]), class.as_deref());
    let decorative = name.is_empty();
    let src = src.filter(|src| !src.trim().is_empty());
    // Remember the URL that failed rather than a plain boolean. If a reused
    // card later receives a different image it gets a fresh attempt, while a
    // broken channel URL immediately falls back to useful initials instead of
    // showing the browser's broken-image glyph and clipped alt text.
    let mut failed_src = use_signal(|| None::<String>);
    match src.filter(|src| failed_src().as_ref() != Some(src)) {
        Some(src) => {
            let failed = src.clone();
            rsx! {
                span { class: cls,
                    img {
                        src,
                        alt: name,
                        loading: "lazy",
                        decoding: "async",
                        onerror: move |_| failed_src.set(Some(failed.clone())),
                    }
                }
            }
        }
        None => {
            let initials = initials.unwrap_or_else(|| initials_of(&name));
            rsx! {
                span {
                    class: cls,
                    role: (!decorative).then_some("img"),
                    aria_label: (!decorative).then_some(name),
                    aria_hidden: decorative.then_some("true"),
                    span { class: "g3-avatar-fallback", aria_hidden: "true", "{initials}" }
                }
            }
        }
    }
}

fn initials_of(name: &str) -> String {
    // The first letter or digit of a word, so a handle such as "@ada" or a
    // name in quotes still gives a letter rather than its punctuation.
    name.split_whitespace()
        .filter_map(|word| word.chars().find(|c| c.is_alphanumeric()))
        .take(2)
        .flat_map(char::to_uppercase)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn initials_come_from_the_first_two_words() {
        assert_eq!(super::initials_of("alex morgan jones"), "AM");
        assert_eq!(super::initials_of("Émile"), "É");
        assert_eq!(super::initials_of(""), "");
        assert_eq!(super::initials_of("@googledevelopers"), "G");
        assert_eq!(super::initials_of("\"Ada\" Lovelace"), "AL");
    }
}

#[cfg(feature = "playground")]
#[component]
fn AvatarPlaygroundDemo() -> Element {
    rsx! {
        crate::PlaygroundDemoFrame {
            div { class: "playground-row",
                Avatar { name: "Alex Morgan", size: AvatarSize::Sm }
                Avatar { name: "Grace Park" }
                Avatar { name: "Sam Ortiz", size: AvatarSize::Lg }
            }
        }
    }
}

crate::g3_playground! {
    name: "Avatar",
    description: "Round image or initials for a person.",
    demo: AvatarPlaygroundDemo,
    source: "src/components/avatar.rs",
}
