//! The class of `MainWindow.xaml`: the main window of the application.

use ferroui_base::input::InputElementImpl;
use ferroui_base::interactivity::InteractiveImpl;
use ferroui_base::layout::LayoutableImpl;
use ferroui_base::{ferro_class, ferro_class_info, ferro_impl_classes, instantiate, FerroObjectImpl, Ref, StyledElementImpl, VisualImpl};
use ferroui_controls::platform::PlatformManager;
use ferroui_controls::primitives::TemplatedControlImpl;
use ferroui_controls::{ContentControlImpl, ControlImpl, TopLevelImpl, Window, WindowBaseImpl, WindowImpl};

#[repr(C)]
pub struct MainWindow {
    base: Window,
}

ferro_class!(MainWindow: Window);
ferro_impl_classes!(
    MainWindow: FerroObjectImpl,
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
ferro_class_info!(MainWindow { new: MainWindow::new });

impl MainWindow {
    pub fn construct() -> Self {
        Self { base: Window::construct(PlatformManager::create_window()) }
    }

    pub fn new() -> Ref<Self> {
        let this = instantiate(Self::construct());
        this.initialize_component();
        this
    }

    /// `InitializeComponent()`: populates the window from the compiled markup of
    /// `MainWindow.xaml`.
    fn initialize_component(&self) {
        if let Err(error) = crate::compiled_main_window::populate(None, &self.to_ref()) {
            panic!("MainWindow.xaml: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MainWindow;
    use crate::view_models::MainWindowViewModel;
    use ferroui_base::BoxedValue;
    use ferroui_controls::testing::{TestServices, UnitTestApplication};
    use ferroui_controls::{Button, TextBlock};

    #[test]
    fn the_button_counts_its_clicks() {
        let _app = UnitTestApplication::start(TestServices::styled_window());
        crate::register_types();
        let window = MainWindow::new();
        window.set_data_context(Some(MainWindowViewModel::new() as BoxedValue));
        let button = window.get_control::<Button>("IncrementButton");
        let text = window.get_control::<TextBlock>("CountText");
        assert_eq!(text.text().as_deref(), Some("Not clicked yet"));

        // What a click of the button does: it runs the command the button is bound to.
        let command = button.command().expect("the button is bound to a command");
        assert!(command.can_execute(None));
        command.execute(None);
        assert_eq!(text.text().as_deref(), Some("Clicked once"));
    }
}
