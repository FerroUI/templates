//! The class of `App.xaml`: the application. Its main view is the view of `views/MainView.xaml`
//! with its view model as the data context: in the window of `views/MainWindow.xaml` on the
//! desktop, as the single view of the page or of the screen in the browser and on mobile.

use crate::view_models::MainViewModel;
use crate::views::{MainView, MainWindow};
use ferroui_base::{ferro_class, ferro_class_info, ferro_impl_classes, instantiate, BoxedValue, FerroObjectImpl, Ref};
use ferroui_controls::{Application, ApplicationImpl, ApplicationImplExt, Control, NewApplication};
use std::rc::Rc;

#[repr(C)]
pub struct App {
    base: Application,
}

ferro_class!(App: Application);
ferro_class_info!(App { new: App::new });
ferro_impl_classes!(App: FerroObjectImpl);

impl NewApplication for App {
    fn new_application() -> Ref<Self> {
        Self::new()
    }
}

impl ApplicationImpl for App {
    fn initialize(this: &Self) {
        crate::register_types();
        this.initialize_component();
    }

    fn on_framework_initialization_completed(this: &Self) {
        let lifetime = this.application_lifetime();
        if let Some(desktop) = lifetime.as_ref().and_then(|l| l.as_classic_desktop_style_application_lifetime()) {
            let window = MainWindow::new();
            window.set_data_context(Some(MainViewModel::new() as BoxedValue));
            desktop.set_main_window(Some(window.upcast()));
        } else if let Some(activity) = lifetime.as_ref().and_then(|l| l.as_activity_application_lifetime()) {
            // Android: a view for each activity.
            activity.set_main_view_factory(Some(Rc::new(|| main_view().upcast())));
        } else if let Some(single_view) = lifetime.as_ref().and_then(|l| l.as_single_view_application_lifetime()) {
            // The browser and iOS: one view.
            single_view.set_main_view(Some(main_view().upcast()));
        }
        Self::parent_on_framework_initialization_completed(this);
    }
}

/// The main view with a new view model.
fn main_view() -> Ref<Control> {
    let view = MainView::new();
    view.set_data_context(Some(MainViewModel::new() as BoxedValue));
    view.upcast()
}

impl App {
    pub fn construct() -> Self {
        Self { base: Application::construct() }
    }

    pub fn new() -> Ref<Self> {
        instantiate(Self::construct())
    }

    /// `InitializeComponent()`: populates the application from the compiled markup of
    /// `App.xaml`.
    fn initialize_component(&self) {
        if let Err(error) = crate::compiled_app::populate(None, &self.to_ref()) {
            panic!("App.xaml: {error}");
        }
    }
}
