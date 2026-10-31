{%- if mvvm == "minimal" -%}
//! The view model of the main window.

use super::ViewModelBase;
use ferroui_base::data::model::{Event, INotifyPropertyChanged};
use ferroui_base::ferro_markup_type;
use std::cell::Cell;
use std::rc::Rc;

/// The view model of the main window: a greeting and a count of clicks.
#[derive(Default)]
pub struct MainWindowViewModel {
    base: ViewModelBase,
    count: Cell<u32>,
}

impl INotifyPropertyChanged for MainWindowViewModel {
    fn property_changed(&self) -> &Event<str> {
        self.base.property_changed()
    }
}

impl PartialEq for MainWindowViewModel {
    /// A view model is shared: two are equal when they are the same object.
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl MainWindowViewModel {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    /// `Greeting`.
    pub fn greeting(&self) -> String {
        "Welcome to FerroUI!".to_owned()
    }

    /// `Count`: how many times the button was clicked.
    pub fn count(&self) -> u32 {
        self.count.get()
    }

    /// `CountText`: the count as the window shows it.
    pub fn count_text(&self) -> String {
        count_text(self.count())
    }

    /// `Increment`, the command of the button: counts one more click.
    pub fn increment(&self) {
        if self.base.raise_and_set_if_changed_cell(&self.count, self.count.get() + 1, "Count") {
            self.base.raise_property_changed("CountText");
        }
    }
}

ferro_markup_type!(class MainWindowViewModel {
    this: Rc<MainWindowViewModel>,
    handles: [MainWindowViewModel, Rc<MainWindowViewModel>, Option<Rc<MainWindowViewModel>>],
    constructors: [() => MainWindowViewModel::new],
    properties: [
        Greeting: String { get: |this: &Rc<MainWindowViewModel>| this.greeting() },
        Count: u32 { get: |this: &Rc<MainWindowViewModel>| this.count() },
        CountText: String { get: |this: &Rc<MainWindowViewModel>| this.count_text() },
    ],
    methods: [
        fn Increment() => |this: &Rc<MainWindowViewModel>| this.increment(),
    ],
    notify_property_changed: MainWindowViewModel,
});
{%- else -%}
//! The view model of the main window, with ReactiveUI.

use ferroui_base::ferro_markup_type;
use ferroui_base::input::ICommand;
use ferroui_reactive::ObservableExt;
use ferroui_reactiveui::{
    reactive_object, IReactiveObjectExt, ObservableAsPropertyHelper, ReactiveCommand, ReactiveObject, ReactiveUIBindingExtensions,
    ToPropertyMixins,
};
use ferroui_reactiveui_controls::ReactiveCommandAdapter;
use ferroui_system::prop;
use std::cell::{Cell, OnceCell};
use std::rc::Rc;

/// The view model of the main window: a greeting and a count of clicks.
///
/// A plain struct rather than one written with `reactive_properties!`: the XAML compiler reads the
/// view models the views bind to from the sources of the crate.
pub struct MainWindowViewModel {
    reactive: ReactiveObject,
    /// `Count`: how many times the button was clicked.
    count: Cell<u32>,
    /// `CountText`: the count as the window shows it, derived from `Count`.
    count_text: OnceCell<ObservableAsPropertyHelper<String>>,
    /// `Increment`: counts one more click.
    increment: OnceCell<ReactiveCommand<(), ()>>,
    /// The command of the button: `Increment` as a command of the controls.
    increment_command: OnceCell<Rc<dyn ICommand>>,
}
reactive_object!(MainWindowViewModel);

impl PartialEq for MainWindowViewModel {
    /// A view model is shared: two are equal when they are the same object.
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl MainWindowViewModel {
    pub fn new() -> Rc<Self> {
        let view_model = Rc::new(Self {
            reactive: ReactiveObject::new(),
            count: Cell::new(0),
            count_text: OnceCell::new(),
            increment: OnceCell::new(),
            increment_command: OnceCell::new(),
        });

        // `CountText` follows `Count`.
        let count_text = view_model
            .when_any_value(prop!(MainWindowViewModel => count))
            .select(count_text)
            .to_property(&view_model, prop!(MainWindowViewModel => count_text));
        let _ = view_model.count_text.set(count_text);

        let weak = Rc::downgrade(&view_model);
        let increment = ReactiveCommand::create(move || {
            if let Some(view_model) = weak.upgrade() {
                view_model.set_count(view_model.count() + 1);
            }
        });
        let _ = view_model.increment_command.set(ReactiveCommandAdapter::command_of(Rc::new(increment.clone())));
        let _ = view_model.increment.set(increment);
        view_model
    }

    /// `Greeting`.
    pub fn greeting(&self) -> String {
        "Welcome to FerroUI!".to_owned()
    }

    /// `Count`.
    pub fn count(&self) -> u32 {
        self.count.get()
    }

    /// Sets `Count` and raises its change.
    pub fn set_count(&self, value: u32) {
        let _ = self.raise_and_set_if_changed(&self.count, value, "Count");
    }

    /// `CountText`.
    pub fn count_text(&self) -> String {
        self.count_text.get().map(|helper| helper.value()).unwrap_or_default()
    }

    /// `Increment`.
    pub fn increment(&self) -> ReactiveCommand<(), ()> {
        self.increment.get().expect("created with the view model").clone()
    }
}

ferro_markup_type!(class MainWindowViewModel {
    this: Rc<MainWindowViewModel>,
    handles: [MainWindowViewModel, Rc<MainWindowViewModel>, Option<Rc<MainWindowViewModel>>],
    constructors: [() => MainWindowViewModel::new],
    properties: [
        Greeting: String { get: |this: &Rc<MainWindowViewModel>| this.greeting() },
        Count: u32 { get: |this: &Rc<MainWindowViewModel>| this.count() },
        CountText: String { get: |this: &Rc<MainWindowViewModel>| this.count_text() },
        IncrementCommand: Rc<dyn ICommand> {
            get: |this: &Rc<MainWindowViewModel>| this.increment_command.get().expect("created with the view model").clone()
        },
    ],
    notify_property_changed: MainWindowViewModel,
});
{%- endif %}

/// The text of the count of clicks.
fn count_text(count: u32) -> String {
    match count {
        0 => "Not clicked yet".to_owned(),
        1 => "Clicked once".to_owned(),
        _ => format!("Clicked {count} times"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_increment_counts_one_more_click() {
        let view_model = MainWindowViewModel::new();
        assert_eq!(view_model.count_text(), "Not clicked yet");
{%- if mvvm == "minimal" %}
        view_model.increment();
        view_model.increment();
{%- else %}
        let _ = view_model.increment().execute(()).subscribe_next(|_| {});
        let _ = view_model.increment().execute(()).subscribe_next(|_| {});
{%- endif %}
        assert_eq!(view_model.count(), 2);
        assert_eq!(view_model.count_text(), "Clicked 2 times");
    }
}
