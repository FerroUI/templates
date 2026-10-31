//! The class of `MainView.xaml`: the main view of the application.

use ferroui_base::input::InputElementImpl;
use ferroui_base::interactivity::InteractiveImpl;
use ferroui_base::layout::LayoutableImpl;
use ferroui_base::{ferro_class, ferro_class_info, ferro_impl_classes, instantiate, FerroObjectImpl, Ref, StyledElementImpl, VisualImpl};
use ferroui_controls::primitives::TemplatedControlImpl;
use ferroui_controls::{ContentControlImpl, ControlImpl, UserControl};

#[repr(C)]
pub struct MainView {
    base: UserControl,
}

ferro_class!(MainView: UserControl);
ferro_impl_classes!(
    MainView: FerroObjectImpl,
    StyledElementImpl,
    VisualImpl,
    LayoutableImpl,
    InteractiveImpl,
    InputElementImpl,
    ControlImpl,
    TemplatedControlImpl,
    ContentControlImpl
);
ferro_class_info!(MainView { new: MainView::new });

impl MainView {
    pub fn construct() -> Self {
        Self { base: UserControl::construct() }
    }

    pub fn new() -> Ref<Self> {
        let this = instantiate(Self::construct());
        this.initialize_component();
        this
    }

    /// `InitializeComponent()`: populates the view from the compiled markup of `MainView.xaml`.
    fn initialize_component(&self) {
        if let Err(error) = crate::compiled_main_view::populate(None, &self.to_ref()) {
            panic!("MainView.xaml: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MainView;
    use crate::view_models::MainViewModel;
    use ferroui_base::BoxedValue;
    use ferroui_controls::testing::{TestServices, UnitTestApplication};
    use ferroui_controls::{Button, TextBlock};

    #[test]
    fn the_button_counts_its_clicks() {
        let _app = UnitTestApplication::start(TestServices::styled_window());
        crate::register_types();
        let view = MainView::new();
        view.set_data_context(Some(MainViewModel::new() as BoxedValue));
        let button = view.get_control::<Button>("IncrementButton");
        let text = view.get_control::<TextBlock>("CountText");
        assert_eq!(text.text().as_deref(), Some("Not clicked yet"));

        // What a click of the button does: it runs the command the button is bound to.
        let command = button.command().expect("the button is bound to a command");
        command.execute(None);
        assert_eq!(text.text().as_deref(), Some("Clicked once"));
    }

    #[test]
    fn the_main_window_shows_the_main_view() {
        let _app = UnitTestApplication::start(TestServices::styled_window());
        crate::register_types();
        let window = crate::views::MainWindow::new();
        let content = window.content().expect("the window has content");
        assert!(ferroui_base::data::core::ValueTypes::as_object(&*content).and_then(|object| object.cast::<MainView>()).is_some());
    }
}
