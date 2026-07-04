//! g3_ui prelude - import all public symbols in one line.

pub use crate::theme::{
    ComponentMode, G3Mode, G3Theme, G3ThemeProvider, Theme, get_mode, init_auto_mode,
    merge_classes, set_mode, use_component_mode,
};
pub use dioxus::prelude::*;

// Components
pub use crate::components::{
    Button, ButtonSize, ButtonStyle, Field, InfoButton, Line, LineOrientation, SegmentButton,
    SegmentGroup, Spinner, Toggle, ToggleSize,
};
pub use crate::{
    G3Button, G3Field, G3InfoButton, G3Line, G3SegmentButton, G3SegmentGroup, G3Spinner, G3Toggle,
    G3ToggleSize,
};

// Component aliases
pub use crate::components::{
    Card, ConfirmModal, Fab, FabButton, FabContainer, FabHorizontal, FabList, FabListSide,
    FabVertical, Navbar, RightSlot, Select, SelectOption, Separator, Setting, SettingAction,
    SettingLink, SettingsGroup, Sheet, SheetButton, SheetPlacement,
};
pub use crate::{
    G3Card, G3ConfirmModal, G3Fab, G3FabButton, G3FabContainer, G3FabList, G3Modal, G3Navbar,
    G3Select, G3Separator, G3Setting, G3SettingAction, G3SettingLink, G3SettingsGroup, G3Sheet,
    G3SheetButton, G3SheetPlacement,
};

// Layout components
pub use crate::components::{AppWrapper, Body, Header};
pub use crate::{G3AppWrapper, G3Body, G3Header};

pub use crate::{ComponentDescriptor, component_descriptors};
#[cfg(feature = "playground")]
pub use crate::{ComponentPlaygroundDemo, component_playground_demos};
