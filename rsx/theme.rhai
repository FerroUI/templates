// The names of the theme the template was generated with: the crate (`ferroui-themes-<theme>`),
// its Rust module and the class of the theme, which App.xaml (or the code) adds to the styles of
// the application.
let theme = variable::get("theme");
let class = switch theme {
    "winui" => "WinUITheme",
    "fluent" => "FluentTheme",
    "simple" => "SimpleTheme",
    "macos" => "MacOSTheme",
    "material" => "MaterialTheme",
    _ => throw "unknown theme " + theme + " (winui, fluent, simple, macos or material)",
};
variable::set("theme_class", class);
variable::set("theme_crate", "ferroui-themes-" + theme);
variable::set("theme_module", "ferroui_themes_" + theme);
