//! {{project-name}}: a FerroUI application written with the React-like API of `ferroui-view`.
//!
//! ```text
//! cargo run
//! cargo run --profile fast-release   # optimised, for trying the application
//! ```
//!
//! The user interface is made of components (`#[component]`) that return `rsx!` trees: a
//! component that runs once with signals its controls follow (`Counter`), and a component that
//! runs again when the state of its hooks changes (`Greeting`, `#[component(rerender)]`).

mod components;

use ferroui_view::prelude::*;
{%- if theme == "material" %}
use ferroui_controls::{AppBuilder, Application};
use ferroui_desktop::AppBuilderDesktopExtensions;
use ferroui_themes_material::{MaterialAppBuilderExtension, MaterialTheme};
{%- else %}
use {{theme_module}}::{{theme_class}};
{%- endif %}

/// The main window.
fn main_window() -> Element {
    window().title("{{project-name}}").width(480.0).height(320.0).child(components::MainView()).into()
}
{% if theme == "material" %}
fn main() {
    // The Material theme comes with its fonts (Roboto), which the application builder adds.
    let args: Vec<String> = std::env::args().skip(1).collect();
    AppBuilder::configure::<Application>().use_platform_detect().with_material_fonts().start(
        |app, _| {
            app.styles().add(MaterialTheme::new().as_style());
            let window = mount_window(main_window);
            #[cfg(debug_assertions)]
            ferroui_view::attach_inspector(&window);
            app.run_window(window);
        },
        &args,
    );
}
{%- else %}
fn main() {
    launch_themed(|| {{theme_class}}::new().as_style(), main_window, |_| {});
}
{%- endif %}
