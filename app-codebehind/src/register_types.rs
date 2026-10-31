//! The type table of the crate: its namespace, its classes, and what it states about itself for
//! markup.

use crate::{App, MainWindow};
use ferroui_base::metadata::MarkupAssembly;
use ferroui_base::TypeInfo;

/// The XAML namespace of the crate (`x:Class="{{project-name | pascal_case}}.MainWindow"`).
const NAMESPACES: &[(&str, &str)] = &[("{{crate_name}}", "{{project-name | pascal_case}}")];

/// What the crate states about itself for markup: its name is the host of the URIs of its
/// documents (`ferres://{{project-name | pascal_case}}/MainWindow.xaml`).
pub static ASSEMBLY: MarkupAssembly = MarkupAssembly {
    name: "{{project-name | pascal_case}}",
    crate_name: "{{crate_name}}",
    xmlns_definitions: &[],
    xmlns_prefixes: &[],
    metadata: &[],
};

/// The classes of the crate.
const TYPES: &[&TypeInfo] = &[App::TYPE, MainWindow::TYPE];

/// Registers the types of the crate, of the theme and of the runtime library of the compiled
/// markup, and the loader table of the compiled markup. Cheap and idempotent.
pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        ferroui_markup_xaml::register_types();
        {{theme_module}}::register_types();
        TypeInfo::register_namespaces(NAMESPACES);
        TypeInfo::register_all(TYPES);
        MarkupAssembly::register(&ASSEMBLY);
        crate::compiled_markup::register();
    });
}
