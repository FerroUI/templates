//! The Android entry point of {{project-name}}.
//!
//! ```text
//! FERROUI_ANDROID_WORKSPACE=$PWD FerroUI/scripts/android/apk.sh package --package {{project-name}}-android \
//!     --label "{{project-name | title_case}}" --application-id {{app_id}}
//! adb install -r <the APK the script names>
//! ```

#[cfg(target_os = "android")]
ferroui_android::android_application!(application::build);

#[cfg(target_os = "android")]
mod application {
    use ferroui_android::log::{self, LogPriority};
    use ferroui_android::AndroidApplicationExtensions;
    use ferroui_base::logging::LogEventLevel;
    use ferroui_controls::AppBuilder;
{%- if theme == "material" %}
    use ferroui_themes_material::MaterialAppBuilderExtension;
{%- endif %}

    /// The application builder the Android backend starts the application with.
    pub fn build() -> AppBuilder {
        AppBuilder::configure::<{{crate_name}}::App>()
{%- if theme == "material" %}
            .with_material_fonts()
{%- endif %}
            .use_android()
            .log_to_delegate(|line| log::write(LogPriority::Warn, "{{project-name}}", line), LogEventLevel::Warning, &[])
    }
}
