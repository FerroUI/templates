//! The class of `MainWindow.xaml`: the main window and the handlers of its events.

use ferroui_base::input::InputElementImpl;
use ferroui_base::interactivity::{IRoutedEventArgs, InteractiveImpl, RoutedEventArgs};
use ferroui_base::layout::LayoutableImpl;
use ferroui_base::{
    ferro_class, ferro_class_info, ferro_impl_classes, instantiate, BoxedValue, FerroObjectImpl, Ref, StyledElementImpl, VisualImpl,
};
use ferroui_controls::platform::PlatformManager;
use ferroui_controls::primitives::TemplatedControlImpl;
use ferroui_controls::{ContentControlImpl, ControlImpl, TextBlock, TopLevelImpl, Window, WindowBaseImpl, WindowImpl};
use std::cell::Cell;
use std::rc::Rc;

#[repr(C)]
pub struct MainWindow {
    base: Window,
    clicks: Cell<u32>,
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
ferro_class_info!(MainWindow {
    new: MainWindow::new,
    markup: {
        // The event handlers `MainWindow.xaml` names (`Click="OnGreetClick"`).
        methods: [
            fn OnGreetClick(Option<BoxedValue>, Rc<dyn IRoutedEventArgs>) =>
                |this: &Ref<MainWindow>, sender: Option<BoxedValue>, e: Rc<dyn IRoutedEventArgs>| {
                    this.on_greet_click(&sender, e.as_routed_event_args())
                },
        ],
    },
});

impl MainWindow {
    pub fn construct() -> Self {
        Self { base: Window::construct(PlatformManager::create_window()), clicks: Cell::new(0) }
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

    /// The `Click` handler of the button.
    pub fn on_greet_click(&self, _sender: &Option<BoxedValue>, _e: &RoutedEventArgs) {
        self.clicks.set(self.clicks.get() + 1);
        let message = self.get_control::<TextBlock>("Message");
        message.set_text(Some(&match self.clicks.get() {
            1 => "Clicked once".to_owned(),
            clicks => format!("Clicked {clicks} times"),
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::MainWindow;
    use ferroui_base::interactivity::RoutedEventArgs;
    use ferroui_controls::testing::{TestServices, UnitTestApplication};
    use ferroui_controls::{Button, TextBlock};

    #[test]
    fn a_click_of_the_button_updates_the_message() {
        let _app = UnitTestApplication::start(TestServices::styled_window());
        crate::register_types();
        let window = MainWindow::new();
        let button = window.get_control::<Button>("GreetButton");
        let message = window.get_control::<TextBlock>("Message");
        assert_eq!(message.text().as_deref(), Some("Not clicked yet"));

        button.raise_event(&RoutedEventArgs::with_event(Button::click_event()));
        assert_eq!(message.text().as_deref(), Some("Clicked once"));
    }
}
