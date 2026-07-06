//! Flat component module for g3_ui.

mod app_wrapper;
mod body;
pub(crate) mod body_styles;
mod button;
pub(crate) mod button_styles;
mod card;
mod card_styles;
mod checkbox;
mod checkbox_styles;
mod confirm_modal;
mod fab;
mod fab_styles;
mod field;
mod field_styles;
mod header;
pub(crate) mod header_styles;
mod info_button;
mod info_button_styles;
mod line;
mod line_styles;
mod list;
mod list_styles;
mod modal;
mod modal_styles;
mod navbar;
pub(crate) mod navbar_styles;
mod primitives;
mod primitives_styles;
mod radio;
mod radio_styles;
mod segment;
mod segment_styles;
mod select;
mod select_styles;
mod settings_group;
mod settings_group_styles;
mod sheet;
mod sheet_button;
mod sheet_styles;
mod shell_styles;
mod spinner;
mod spinner_styles;
mod toggle;
mod toggle_styles;

pub use app_wrapper::*;
pub use body::*;
pub use button::*;
pub use card::*;
pub use checkbox::*;
pub use confirm_modal::*;
pub use fab::*;
pub use field::*;
pub use header::*;
pub use info_button::*;
pub use line::*;
pub use list::*;
pub use modal::*;
pub use navbar::*;
pub use primitives::*;
pub use radio::*;
pub use segment::*;
pub use select::*;
pub use settings_group::*;
pub use sheet::*;
pub use sheet_button::*;
pub use spinner::*;
pub use toggle::*;

use crate::ComponentDescriptor;

pub fn component_descriptors() -> Vec<ComponentDescriptor> {
    vec![
        button::DESCRIPTOR,
        toggle::DESCRIPTOR,
        field::DESCRIPTOR,
        spinner::DESCRIPTOR,
        info_button::DESCRIPTOR,
        line::DESCRIPTOR,
        list::DESCRIPTOR,
        segment::DESCRIPTOR,
        card::DESCRIPTOR,
        checkbox::DESCRIPTOR,
        sheet::DESCRIPTOR,
        select::DESCRIPTOR,
        confirm_modal::DESCRIPTOR,
        settings_group::DESCRIPTOR,
        navbar::DESCRIPTOR,
        primitives::DESCRIPTOR,
        radio::DESCRIPTOR,
        header::DESCRIPTOR,
        body::DESCRIPTOR,
        fab::DESCRIPTOR,
        sheet_button::DESCRIPTOR,
        app_wrapper::DESCRIPTOR,
    ]
}

#[cfg(feature = "playground")]
pub fn component_playground_demos() -> Vec<crate::ComponentPlaygroundDemo> {
    vec![
        button::PLAYGROUND,
        toggle::PLAYGROUND,
        field::PLAYGROUND,
        spinner::PLAYGROUND,
        info_button::PLAYGROUND,
        line::PLAYGROUND,
        list::PLAYGROUND,
        segment::PLAYGROUND,
        card::PLAYGROUND,
        checkbox::PLAYGROUND,
        sheet::PLAYGROUND,
        select::PLAYGROUND,
        confirm_modal::PLAYGROUND,
        settings_group::PLAYGROUND,
        navbar::PLAYGROUND,
        primitives::PLAYGROUND,
        radio::PLAYGROUND,
        header::PLAYGROUND,
        body::PLAYGROUND,
        fab::PLAYGROUND,
        sheet_button::PLAYGROUND,
        app_wrapper::PLAYGROUND,
    ]
}
