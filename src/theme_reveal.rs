//! Switching the theme or mode with the new look growing out of a circle
//! from where the user clicked, instead of flashing.

use dioxus::prelude::*;

const REVEAL_SCRIPT: &str = include_str!("theme_reveal.js");

/// Applies a theme or mode change inside a view transition: the page as it
/// was stays on screen while the new look grows over it in a circle from
/// `origin`, a point in the viewport such as a click (`client_coordinates`),
/// until it covers the window. Like Aperture's theme sweep.
///
/// `apply` makes the change: set the [`crate::AppWrapper`] `theme` or `mode`
/// it reads, or [`crate::use_theme`] / [`crate::use_mode`]. It runs once,
/// either way; without View Transitions, with reduced motion, or with no
/// `origin`, it runs straight away with no animation.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let mut dark = use_signal(|| false);
/// let mut origin = use_signal(|| None::<(f64, f64)>);
/// rsx! {
///     button {
///         onpointerdown: move |e| {
///             let p = e.client_coordinates();
///             origin.set(Some((p.x, p.y)));
///         },
///         onclick: move |_| async move {
///             g3_ui::reveal_theme(origin(), move || dark.toggle()).await;
///         },
///         "Dark"
///     }
/// }
/// # }
/// ```
pub async fn reveal_theme(origin: Option<(f64, f64)>, apply: impl FnOnce()) {
    let origin = match origin {
        Some((x, y)) if x.is_finite() && y.is_finite() => format!("{{ x: {x}, y: {y} }}"),
        _ => "null".to_string(),
    };
    let mut eval = document::eval(&REVEAL_SCRIPT.replace("__ORIGIN__", &origin));
    // The script asks for the change once the old look is captured (or at
    // once when it won't animate); the change lands either way.
    let asked = eval.recv::<String>().await;
    apply();
    if asked.is_ok() {
        let _ = eval.send("applied");
        // Keep the script's channel open until the reveal has finished: the
        // ack is lost if the handle is dropped before the script reads it.
        let _ = eval.recv::<String>().await;
    }
}
