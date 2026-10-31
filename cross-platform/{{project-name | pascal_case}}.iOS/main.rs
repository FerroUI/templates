//! The iOS entry point of {{project-name}} (UIKit; built on macOS with Xcode).
//!
//! ```text
//! FERROUI_IOS_WORKSPACE=$PWD FerroUI/scripts/ios/bundle.sh --package {{project-name}}-ios \
//!     --bin {{project-name}}-ios --name "{{project-name | title_case}}" --bundle-id {{app_id}}
//! ```

#[cfg(not(target_os = "ios"))]
fn main() {
    eprintln!("{{project-name}}-ios is the iOS entry point of {{project-name}}; it needs UIKit. Run {{project-name}}-desktop on this system.");
}

#[cfg(target_os = "ios")]
fn main() {
    ferroui_ios::run_application_with(std::rc::Rc::new(host::AppDelegate))
}

#[cfg(target_os = "ios")]
mod host {
    use ferroui_base::logging::LogEventLevel;
    use ferroui_controls::AppBuilder;
    use ferroui_ios::{FerroApplicationDelegate, IFerroAppDelegate, IosApplicationExtensions};
{%- if theme == "material" %}
    use ferroui_themes_material::MaterialAppBuilderExtension;
{%- endif %}
    use std::rc::Rc;

    /// The application delegate: the application builder the iOS backend starts the application
    /// with.
    pub struct AppDelegate;

    impl FerroApplicationDelegate for AppDelegate {
        fn create_app_builder(&self, app_delegate: Rc<dyn IFerroAppDelegate>) -> AppBuilder {
            AppBuilder::configure::<{{crate_name}}::App>()
{%- if theme == "material" %}
                .with_material_fonts()
{%- endif %}
                .use_ios_with_delegate(app_delegate)
                .log_to_trace(LogEventLevel::Warning, &[])
        }
    }
}
