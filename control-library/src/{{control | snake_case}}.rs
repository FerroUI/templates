//! The templated control `{{control}}`: a header over content, drawn by its control theme
//! (`themes/Generic.xaml`).

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
/// the control (`{x:Type controls:{{control}}}`).
#[repr(C)]
pub struct {{control}} {
    base: ContentControl,
}

ferro_class!({{control}}: ContentControl);
ferro_class_info!({{control}} { new: {{control}}::new });
ferro_impl_classes!(
    {{control}}: FerroObjectImpl,
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
    impl {{control}} {
        /// `Header`: the text above the content.
        pub fn header_property() -> StyledProperty<Option<String>> {
            FerroProperty::register::<{{control}}, _>("Header", None)
        }

        /// `IsHighlighted`: whether the control is drawn with the accent of the theme.
        pub fn is_highlighted_property() -> StyledProperty<bool> {
            FerroProperty::register::<{{control}}, _>("IsHighlighted", false)
        }
    }
}

impl {{control}} {
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
