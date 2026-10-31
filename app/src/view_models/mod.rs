//! The view models of the application (namespace `{{project-name | pascal_case}}.ViewModels`).

use ferroui_base::data::core::ValueTypes;
use ferroui_base::metadata::{MarkupType, MarkupTyped};

mod main_window_view_model;
{%- if mvvm == "minimal" %}
mod view_model_base;
{%- endif %}

pub use main_window_view_model::MainWindowViewModel;
{%- if mvvm == "minimal" %}
pub use view_model_base::ViewModelBase;
{%- endif %}

/// The view models of this namespace, as markup sees them (`ferro_markup_type!`).
pub(crate) const MARKUP_TYPES: &[&MarkupType] = &[<MainWindowViewModel as MarkupTyped>::MARKUP];

/// What the untyped value conversions must know about the view models: a data context holds
/// one by reference.
pub(crate) fn register_value_types() {
    ValueTypes::register_reference::<MainWindowViewModel>();
}
