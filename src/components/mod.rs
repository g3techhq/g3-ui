//! Every component, in a flat module tree.
mod accordion;
mod action_sheet;
mod alert;
mod app_wrapper;
mod avatar;
mod back_button;
mod badge;
mod bottom_sheet;
mod button;
mod card;
mod checkbox;
mod chip;
mod color;
mod confirm_modal;
mod content;
#[cfg(feature = "playground")]
mod demo_app;
mod divider;
mod fab;
mod field;
mod header;
mod infinite_scroll;
mod info_button;
mod keyboard;
mod layout;
mod list;
mod media;
mod modal;
mod nav;
mod navigation_drawer;
pub(crate) mod overlay;
mod overlay_host;
mod popover;
pub(crate) mod pressable;
mod progress;
mod radio;
mod range;
mod refresher;
mod searchbar;
mod segment;
mod select;
mod sheet;
mod side_sheet;
mod skeleton;
mod spinner;
mod swipe;
mod tab_layout;
mod tabs;
mod text;
mod toast;
mod toggle;

pub use accordion::{AccordionGroup, AccordionItem};
pub use action_sheet::{ActionSheet, ActionSheetButton};
pub use alert::{Alert, AlertButton, AlertButtonRole, AlertInput, AlertResult};
pub use app_wrapper::AppWrapper;
pub use avatar::{Avatar, AvatarSize};
pub use back_button::BackButton;
pub use badge::Badge;
pub use bottom_sheet::BottomSheet;
pub use button::{Button, ButtonExpand, ButtonFill, ButtonSize};
pub use card::{Card, CardVariant};
pub use checkbox::{Checkbox, ControlLabelPlacement};
pub use chip::Chip;
pub use color::Color;
pub use confirm_modal::ConfirmModal;
pub use content::{Content, ContentWidth};
pub use divider::{Divider, DividerOrientation};
pub use fab::{Fab, FabButton, FabHorizontal, FabList, FabListSide, FabMenu, FabSize, FabVertical};
#[cfg(test)]
pub(crate) use field::within_typing_limits as field_limits;
pub use field::{Input, InputType, TextArea};
pub use header::Header;
pub(crate) use header::HeaderToolbarContext;
pub use infinite_scroll::InfiniteScroll;
pub use info_button::InfoButton;
pub use layout::{Grid, GridColumns, Space, Stack, StackAlign, StackJustify};
pub use list::{Item, ItemDetail, List, ListHeader, ListLines, ListVariant};
pub use media::{Img, ImgFit, Tooltip, TooltipPlacement};
pub use modal::{Modal, ModalRole, ModalSize};
pub use nav::{AdaptiveNav, AdaptiveNavCompact, NavBar, NavItem, NavItemGroup, NavRail};
pub use navigation_drawer::NavigationDrawer;
pub use overlay_host::{
    ActionSheetOptions, ActionSheets, AlertOptions, Alerts, ToastId, ToastOptions, Toaster,
    use_action_sheet, use_alert, use_toast,
};
pub use popover::{Menu, MenuItem, Popover, PopoverPlacement};
pub use pressable::ButtonType;
pub use progress::Progress;
pub use radio::{Radio, RadioGroup};
pub use range::{Range, Stepper};
pub use refresher::Refresher;
pub use searchbar::Searchbar;
pub use segment::{SegmentButton, SegmentGroup};
pub use select::{Select, SelectOption};
pub use sheet::{SheetBackdrop, SheetEdge, SideSheetBehavior, open_sheet_count};
pub use side_sheet::SideSheet;
pub use skeleton::{Skeleton, SkeletonShape};
pub use spinner::{Spinner, SpinnerSize};
pub use swipe::{SwipeAction, SwipeBehavior, SwipeItem, SwipeSide, SwipeState};
pub use tab_layout::TabLayout;
pub use tabs::{Tab, TabList, TabPanel, Tabs};
pub use text::{Text, TextTone, TextVariant};
pub use toast::{Toast, ToastDuration, ToastPosition};
pub use toggle::{Toggle, ToggleSize};

/// Declares the gallery: every module that registers a playground entry, in
/// the order the gallery lists them.
macro_rules! gallery {
    ($($module:ident),* $(,)?) => {
        pub(crate) fn component_descriptors() -> Vec<crate::ComponentDescriptor> {
            vec![$($module::DESCRIPTOR),*]
        }

        #[cfg(feature = "playground")]
        pub(crate) fn component_playground_demos() -> Vec<crate::ComponentPlaygroundDemo> {
            vec![demo_app::PLAYGROUND, $($module::PLAYGROUND),*]
        }
    };
}

gallery![
    app_wrapper,
    content,
    header,
    nav,
    navigation_drawer,
    button,
    fab,
    info_button,
    popover,
    field,
    select,
    checkbox,
    toggle,
    radio,
    range,
    searchbar,
    card,
    list,
    layout,
    media,
    avatar,
    badge,
    chip,
    divider,
    progress,
    skeleton,
    spinner,
    accordion,
    segment,
    tabs,
    bottom_sheet,
    side_sheet,
    alert,
    confirm_modal,
    toast,
    refresher,
    infinite_scroll,
];
