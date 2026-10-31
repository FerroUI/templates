//! The base of the view models: the property change notification the bindings of the views
//! listen to.

use ferroui_base::data::model::{Event, INotifyPropertyChanged};
use std::cell::{Cell, RefCell};

/// The base of a view model. A view model holds it as a field and forwards
/// [`INotifyPropertyChanged`] to it.
#[derive(Default)]
pub struct ViewModelBase {
    property_changed: Event<str>,
}

impl INotifyPropertyChanged for ViewModelBase {
    fn property_changed(&self) -> &Event<str> {
        &self.property_changed
    }
}

impl ViewModelBase {
    pub fn new() -> Self {
        Self::default()
    }

    /// Stores `value` and raises the notification of `property_name` when it differs from the
    /// stored value (`RaiseAndSetIfChanged`). Returns whether it did.
    pub fn raise_and_set_if_changed<T: PartialEq>(&self, field: &RefCell<T>, value: T, property_name: &str) -> bool {
        if *field.borrow() == value {
            return false;
        }
        // The borrow ends before the handlers run: they read the property.
        *field.borrow_mut() = value;
        self.raise_property_changed(property_name);
        true
    }

    /// [`raise_and_set_if_changed`](Self::raise_and_set_if_changed) for a `Copy` field.
    pub fn raise_and_set_if_changed_cell<T: PartialEq + Copy>(&self, field: &Cell<T>, value: T, property_name: &str) -> bool {
        if field.get() == value {
            return false;
        }
        field.set(value);
        self.raise_property_changed(property_name);
        true
    }

    /// Raises the notification of `property_name` (`RaisePropertyChanged`).
    pub fn raise_property_changed(&self, property_name: &str) {
        self.property_changed.raise(property_name);
    }
}
