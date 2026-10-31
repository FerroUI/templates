//! The type table of the crate: its namespaces, its classes and view models, and what it states
//! about itself for markup.

use crate::{view_models, views, App};
use ferroui_base::data::core::ValueTypes;
use ferroui_base::metadata::{MarkupAssembly, MarkupType};
use ferroui_base::TypeInfo;

/// The XAML namespaces of the modules of the crate (`x:Class="{{project-name | pascal_case}}.Views.MainWindow"`).
const NAMESPACES: &[(&str, &str)] = &[
    ("{{crate_name}}", "{{project-name | pascal_case}}"),
    ("{{crate_name}}::view_models", "{{project-name | pascal_case}}.ViewModels"),
    ("{{crate_name}}::views", "{{project-name | pascal_case}}.Views"),
];

/// What the crate states about itself for markup: its name is the host of the URIs of its
/// documents (`ferres://{{project-name | pascal_case}}/views/MainView.xaml`).
pub static ASSEMBLY: MarkupAssembly = MarkupAssembly {
    name: "{{project-name | pascal_case}}",
    crate_name: "{{crate_name}}",
    xmlns_definitions: &[],
    xmlns_prefixes: &[],
    metadata: &[],
};

/// The classes of the root namespace.
const TYPES: &[&TypeInfo] = &[App::TYPE];

/// Registers the types of the crate, of the theme and of the runtime library of the compiled
/// markup, and the loader table of the compiled markup. Cheap and idempotent.
pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        ferroui_markup_xaml::register_types();
        {{theme_module}}::register_types();
        TypeInfo::register_namespaces(NAMESPACES);
        for types in [TYPES, views::TYPES] {
            TypeInfo::register_all(types);
        }
        MarkupType::register_all(view_models::MARKUP_TYPES);
        ValueTypes::register_global(view_models::register_value_types);
        MarkupAssembly::register(&ASSEMBLY);
        crate::compiled_markup::register();
    });
}
