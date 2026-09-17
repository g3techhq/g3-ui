//! The element behind every tappable component: a `<button>`, a plain link,
//! or a router link, chosen from the props the caller set.
use dioxus::prelude::*;

/// Where a tappable component goes when activated.
#[derive(Clone, PartialEq, Default)]
pub(crate) enum Target {
    /// No navigation: a `<button>`.
    #[default]
    Action,
    /// A plain `<a href>`.
    Href { href: String, new_tab: bool },
    /// A Dioxus router link.
    Route { to: NavigationTarget, new_tab: bool },
}

impl Target {
    pub(crate) fn from_props(
        href: Option<String>,
        to: Option<NavigationTarget>,
        new_tab: bool,
    ) -> Self {
        match (to, href) {
            (Some(to), _) => Target::Route { to, new_tab },
            (None, Some(href)) => Target::Href { href, new_tab },
            (None, None) => Target::Action,
        }
    }

    pub(crate) fn is_link(&self) -> bool {
        !matches!(self, Target::Action)
    }
}

/// HTML `type` of a `<button>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ButtonType {
    /// A plain button that does nothing on its own.
    #[default]
    Button,
    /// Submits its form.
    Submit,
    /// Resets its form.
    Reset,
}

impl ButtonType {
    fn as_str(self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
        }
    }
}

#[component]
pub(crate) fn Pressable(
    class: String,
    #[props(default)] target: Target,
    #[props(default)] disabled: bool,
    #[props(default)] button_type: ButtonType,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let handle_click = move |event: MouseEvent| {
        if disabled {
            event.prevent_default();
            return;
        }
        if let Some(onclick) = onclick {
            onclick.call(event);
        }
    };
    match target {
        Target::Action => rsx! {
            button {
                class,
                r#type: button_type.as_str(),
                disabled,
                onclick: handle_click,
                ..attributes,
                {children}
            }
        },
        // A disabled link has no href, so it cannot be followed or focused.
        Target::Href { .. } | Target::Route { .. } if disabled => rsx! {
            a {
                class,
                role: "link",
                aria_disabled: "true",
                ..attributes,
                {children}
            }
        },
        Target::Href { href, new_tab } => rsx! {
            a {
                class,
                href,
                target: new_tab.then_some("_blank"),
                rel: new_tab.then_some("noopener noreferrer"),
                onclick: handle_click,
                ..attributes,
                {children}
            }
        },
        Target::Route { to, new_tab } => rsx! {
            Link {
                class,
                to,
                new_tab,
                onclick: handle_click,
                attributes,
                {children}
            }
        },
    }
}
