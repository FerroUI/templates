//! The view models of the application (namespace `{{project-name | pascal_case}}.ViewModels`).

use ferroui_base::data::core::ValueTypes;
use ferroui_base::metadata::{MarkupType, MarkupTyped};

mod main_view_model;
mod view_model_base;

pub use main_view_model::MainViewModel;
pub use view_model_base::ViewModelBase;

/// The view models of this namespace, as markup sees them (`ferro_markup_type!`).
pub(crate) const MARKUP_TYPES: &[&MarkupType] = &[<MainViewModel as MarkupTyped>::MARKUP];

/// What the untyped value conversions must know about the view models: a data context holds
/// one by reference.
pub(crate) fn register_value_types() {
    ValueTypes::register_reference::<MainViewModel>();
}
