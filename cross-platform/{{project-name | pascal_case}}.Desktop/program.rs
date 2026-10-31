//! The desktop entry point of {{project-name}}: the application in a window.
//!
//! ```text
//! cargo run -p {{project-name}}-desktop
//! cargo run -p {{project-name}}-desktop --profile fast-release
//! ```

use ferroui_base::logging::LogEventLevel;
use ferroui_controls::AppBuilder;
use ferroui_desktop::AppBuilderDesktopExtensions;
{%- if theme == "material" %}
use ferroui_themes_material::MaterialAppBuilderExtension;
{%- endif %}

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let exit_code = AppBuilder::configure::<{{crate_name}}::App>()
        .use_platform_detect()
{%- if theme == "material" %}
        .with_material_fonts()
{%- endif %}
        .log_to_trace(LogEventLevel::Warning, &[])
        .start_with_classic_desktop_lifetime(&args);
    std::process::ExitCode::from(exit_code as u8)
}
