//! The templated control `{{project-name | pascal_case}}`: a header over content, drawn by its control theme
//! (`{{project-name | pascal_case}}Theme.xaml`).

use ferroui_base::input::InputElementImpl;
use ferroui_base::interactivity::InteractiveImpl;
use ferroui_base::layout::LayoutableImpl;
use ferroui_base::{
    ferro_class, ferro_class_info, ferro_impl_classes, ferro_properties, instantiate, FerroObjectImpl, FerroProperty, Ref,
    StyledElementImpl, StyledProperty, VisualImpl,
};
use ferroui_controls::primitives::TemplatedControlImpl;
use ferroui_controls::{ContentControl, ContentControlImpl, ControlImpl};

/// A header over content. Its look is its control theme: the key of the theme is the type of
/// the control (`{x:Type controls:{{project-name | pascal_case}}}`).
#[repr(C)]
pub struct {{project-name | pascal_case}} {
    base: ContentControl,
}

ferro_class!({{project-name | pascal_case}}: ContentControl);
ferro_class_info!({{project-name | pascal_case}} { new: {{project-name | pascal_case}}::new });
ferro_impl_classes!(
    {{project-name | pascal_case}}: FerroObjectImpl,
    StyledElementImpl,
    VisualImpl,
    LayoutableImpl,
    InteractiveImpl,
    InputElementImpl,
    ControlImpl,
    TemplatedControlImpl,
    ContentControlImpl
);

ferro_properties! {
    impl {{project-name | pascal_case}} {
        /// `Header`: the text above the content.
        pub fn header_property() -> StyledProperty<Option<String>> {
            FerroProperty::register::<{{project-name | pascal_case}}, _>("Header", None)
        }

        /// `IsHighlighted`: whether the control is drawn with the accent of the theme.
        pub fn is_highlighted_property() -> StyledProperty<bool> {
            FerroProperty::register::<{{project-name | pascal_case}}, _>("IsHighlighted", false)
        }
    }
}

impl {{project-name | pascal_case}} {
    pub fn construct() -> Self {
        Self { base: ContentControl::construct() }
    }

    pub fn new() -> Ref<Self> {
        instantiate(Self::construct())
    }

    /// `Header`.
    pub fn header(&self) -> Option<String> {
        self.get_value(Self::header_property())
    }

    /// Sets `Header`.
    pub fn set_header(&self, value: Option<&str>) {
        self.set_value(Self::header_property(), value.map(str::to_owned));
    }

    /// `IsHighlighted`.
    pub fn is_highlighted(&self) -> bool {
        self.get_value(Self::is_highlighted_property())
    }

    /// Sets `IsHighlighted`.
    pub fn set_is_highlighted(&self, value: bool) {
        self.set_value(Self::is_highlighted_property(), value);
    }
}
