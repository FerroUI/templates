#!/usr/bin/env bash
# Adds every item template to a project generated from the `app` template, wired as the README of
# each item says, with a test that creates each item, then builds and tests the project
# (.github/workflows/ci.yml, job "Items").
#
#   scripts/items-smoke.sh <directory of a project generated from the app template>
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
app="$(cd "$1" && pwd)"
cargo=${CARGO:-cargo}
crate="$(sed -n 's/^name = "\(.*\)"/\1/p' "$app/Cargo.toml" | head -n 1)"
assembly="$(sed -n 's/^    name: "\(.*\)",/\1/p' "$app/src/register_types.rs" | head -n 1)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

generate() { # template, name, defines...
  local template=$1 name=$2
  shift 2
  ( cd "$work" && cargo generate --path "$root" "$template" --name "$name" --silent --vcs none --force "$@" )
}

generate window SettingsWindow --define namespace="$assembly.Views"
generate user-control StatusView --define namespace="$assembly.Views"
generate templated-control BadgeCard --define namespace="$assembly.Views"
generate resource-dictionary AppResources
generate styles AppStyles

mv "$work"/SettingsWindow/SettingsWindow.xaml "$work"/SettingsWindow/settings_window.rs "$app/src/views/"
mv "$work"/StatusView/StatusView.xaml "$work"/StatusView/status_view.rs "$app/src/views/"
mv "$work"/BadgeCard/BadgeCardTheme.xaml "$work"/BadgeCard/badge_card.rs "$app/src/views/"
mkdir -p "$app/src/resources" "$app/src/styles"
mv "$work"/AppResources/AppResources.xaml "$app/src/resources/"
mv "$work"/AppStyles/AppStyles.xaml "$app/src/styles/"

python3 - "$app" "$assembly" <<'PY'
import pathlib, sys
app, assembly = pathlib.Path(sys.argv[1]), sys.argv[2]
views = app / "src/views/mod.rs"
text = views.read_text()
text = text.replace("mod main_window;\n", "mod badge_card;\nmod main_window;\nmod settings_window;\nmod status_view;\n")
text = text.replace("pub use main_window::MainWindow;\n",
                    "pub use badge_card::BadgeCard;\npub use main_window::MainWindow;\npub use settings_window::SettingsWindow;\npub use status_view::StatusView;\n")
text = text.replace("&[MainWindow::TYPE]", "&[MainWindow::TYPE, SettingsWindow::TYPE, StatusView::TYPE, BadgeCard::TYPE]")
views.write_text(text)

app_xaml = app / "src/App.xaml"
text = app_xaml.read_text()
text = text.replace("  <Application.Styles>\n", f"""  <Application.Resources>
    <ResourceDictionary>
      <ResourceDictionary.MergedDictionaries>
        <ResourceInclude Source="ferres://{assembly}/resources/AppResources.xaml" />
        <ResourceInclude Source="ferres://{assembly}/views/BadgeCardTheme.xaml" />
      </ResourceDictionary.MergedDictionaries>
    </ResourceDictionary>
  </Application.Resources>
  <Application.Styles>
""")
text = text.replace("  </Application.Styles>\n", f"""    <StyleInclude Source="ferres://{assembly}/styles/AppStyles.xaml" />
  </Application.Styles>
""")
app_xaml.write_text(text)

main = app / "src/main.rs"
main.write_text(main.read_text() + "\n#[cfg(test)]\nmod items_tests;\n")
(app / "src/items_tests.rs").write_text('''//! The items of the item templates, created (scripts/items-smoke.sh of the templates).

use crate::views::{BadgeCard, SettingsWindow, StatusView};
use ferroui_controls::testing::{TestServices, UnitTestApplication};

#[test]
fn the_items_are_created() {
    let _app = UnitTestApplication::start(TestServices::styled_window());
    crate::register_types();
    let _window = SettingsWindow::new();
    let _view = StatusView::new();
    let card = BadgeCard::new();
    card.set_header(Some("Header"));
    assert_eq!(card.header().as_deref(), Some("Header"));
}

#[test]
fn the_application_document_includes_the_items() {
    let _app = UnitTestApplication::start(TestServices::styled_window());
    crate::register_types();
    // App.xaml merges the resource dictionary and the control theme and includes the styles.
    let app = crate::App::new();
    ferroui_controls::ApplicationImpl::initialize(&*app);
}
''')
PY

cd "$app"
$cargo test
