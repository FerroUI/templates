//! The class of `{{project-name | pascal_case}}.xaml`: a window.

use ferroui_base::input::InputElementImpl;
use ferroui_base::interactivity::InteractiveImpl;
use ferroui_base::layout::LayoutableImpl;
use ferroui_base::{ferro_class, ferro_class_info, ferro_impl_classes, instantiate, FerroObjectImpl, Ref, StyledElementImpl, VisualImpl};
use ferroui_controls::platform::PlatformManager;
use ferroui_controls::primitives::TemplatedControlImpl;
use ferroui_controls::{ContentControlImpl, ControlImpl, TopLevelImpl, Window, WindowBaseImpl, WindowImpl};

#[repr(C)]
pub struct {{project-name | pascal_case}} {
    base: Window,
}

ferro_class!({{project-name | pascal_case}}: Window);
ferro_impl_classes!(
    {{project-name | pascal_case}}: FerroObjectImpl,
    StyledElementImpl,
    VisualImpl,
    LayoutableImpl,
    InteractiveImpl,
    InputElementImpl,
    ControlImpl,
    TemplatedControlImpl,
    ContentControlImpl,
    TopLevelImpl,
    WindowBaseImpl,
    WindowImpl
);
ferro_class_info!({{project-name | pascal_case}} { new: {{project-name | pascal_case}}::new });

impl {{project-name | pascal_case}} {
    pub fn construct() -> Self {
        Self { base: Window::construct(PlatformManager::create_window()) }
    }

    pub fn new() -> Ref<Self> {
        let this = instantiate(Self::construct());
        this.initialize_component();
        this
    }

    /// `InitializeComponent()`: populates the window from the compiled markup of
    /// `{{project-name | pascal_case}}.xaml`.
    fn initialize_component(&self) {
        if let Err(error) = crate::compiled_{{project-name | snake_case}}::populate(None, &self.to_ref()) {
            panic!("{{project-name | pascal_case}}.xaml: {error}");
        }
    }
}
