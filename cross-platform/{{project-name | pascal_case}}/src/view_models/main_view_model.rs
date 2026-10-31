//! The view model of the main view.

use super::ViewModelBase;
use ferroui_base::data::model::{Event, INotifyPropertyChanged};
use ferroui_base::ferro_markup_type;
use std::cell::Cell;
use std::rc::Rc;

/// The view model of the main view: a greeting and a count of clicks.
#[derive(Default)]
pub struct MainViewModel {
    base: ViewModelBase,
    count: Cell<u32>,
}

impl INotifyPropertyChanged for MainViewModel {
    fn property_changed(&self) -> &Event<str> {
        self.base.property_changed()
    }
}

impl PartialEq for MainViewModel {
    /// A view model is shared: two are equal when they are the same object.
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl MainViewModel {
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

ferro_markup_type!(class MainViewModel {
    this: Rc<MainViewModel>,
    handles: [MainViewModel, Rc<MainViewModel>, Option<Rc<MainViewModel>>],
    constructors: [() => MainViewModel::new],
    properties: [
        Greeting: String { get: |this: &Rc<MainViewModel>| this.greeting() },
        Count: u32 { get: |this: &Rc<MainViewModel>| this.count() },
        CountText: String { get: |this: &Rc<MainViewModel>| this.count_text() },
    ],
    methods: [
        fn Increment() => |this: &Rc<MainViewModel>| this.increment(),
    ],
    notify_property_changed: MainViewModel,
});

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
        let view_model = MainViewModel::new();
        assert_eq!(view_model.count_text(), "Not clicked yet");
        view_model.increment();
        view_model.increment();
        assert_eq!(view_model.count(), 2);
        assert_eq!(view_model.count_text(), "Clicked 2 times");
    }
}
