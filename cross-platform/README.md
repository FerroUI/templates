# {{project-name}}

A cross-platform application built with [FerroUI](https://github.com/wieslawsoltes/FerroUI), a
Rust UI framework with XAML. One crate holds the application; the hosts start it on each platform.

| Crate | Directory | What it is |
|---|---|---|
| `{{project-name}}` | `{{project-name | pascal_case}}/` | The application: `App.xaml`, the views (`views/`) and the view models (`view_models/`) |
{%- if desktop %}
| `{{project-name}}-desktop` | `{{project-name | pascal_case}}.Desktop/` | The desktop host (Windows, macOS, Linux) |
{%- endif %}
{%- if browser %}
| `{{project-name}}-browser` | `{{project-name | pascal_case}}.Browser/` | The browser host (WebAssembly), with its page in `wwwroot/` |
{%- endif %}
{%- if android %}
| `{{project-name}}-android` | `{{project-name | pascal_case}}.Android/` | The Android host |
{%- endif %}
{%- if ios %}
| `{{project-name}}-ios` | `{{project-name | pascal_case}}.iOS/` | The iOS host |
{%- endif %}

FerroUI is a git submodule at `FerroUI/`: the crates are path dependencies into it, and the
packaging scripts of the browser, Android and iOS come with it ([FerroUI/docs/apps.md](https://github.com/wieslawsoltes/FerroUI/blob/main/docs/apps.md)).

## Setup

Once, after the project was generated:

```sh
scripts/setup.sh
```

It adds the submodule, writes the FerroUI blocks of `Cargo.toml`
(`FerroUI/scripts/apps/sync-workspace.py`) and starts `Cargo.lock` from FerroUI's.

## Build and run
{% if desktop %}
```sh
cargo run -p {{project-name}}-desktop
cargo run -p {{project-name}}-desktop --profile fast-release   # optimised, for trying the application
cargo test -p {{project-name}}
```
{%- else %}
```sh
cargo test -p {{project-name}}
```
{%- endif %}
{%- if browser %}

The browser site, with the toolchain of `FerroUI/scripts/browser/setup.sh`:

```sh
source ~/browser-tools/env.sh
FERROUI_BROWSER_WORKSPACE=$PWD FerroUI/scripts/build-browser.sh {{project-name}}-browser
```

The site is written to `target/browser/{{project-name}}-browser`; serve that directory with any
static web server.
{%- endif %}
{%- if android %}

The Android package, with the SDK and NDK of `FerroUI/scripts/android/env.sh`:

```sh
FERROUI_ANDROID_WORKSPACE=$PWD FerroUI/scripts/android/apk.sh package --package {{project-name}}-android \
    --label "{{project-name | title_case}}" --application-id {{app_id}}
```
{%- endif %}
{%- if ios %}

The iOS application bundle, on macOS with Xcode:

```sh
FERROUI_IOS_WORKSPACE=$PWD FerroUI/scripts/ios/bundle.sh --package {{project-name}}-ios --bin {{project-name}}-ios \
    --name "{{project-name | title_case}}" --bundle-id {{app_id}}
```
{%- endif %}

## CI

`.github/workflows/ci.yml` calls the shared workflow of FerroUI: the conventions, the tests{% if desktop %}, the
desktop build{% endif %}{% if browser %}, the browser site with a smoke run in headless Chrome and its deployment to
GitHub Pages from main{% endif %}{% if android %}, the Android APK{% endif %}{% if ios %}, the iOS bundle for the simulator{% endif %}.
{%- if browser %} Enable GitHub Pages once, with GitHub Actions as its source:

```sh
gh api -X POST repos/<owner>/<repository>/pages -f build_type=workflow
```
{%- endif %}

## Updating FerroUI

```sh
git -C FerroUI fetch origin && git -C FerroUI checkout origin/main
python3 FerroUI/scripts/apps/sync-workspace.py
cargo metadata --format-version 1 > /dev/null
git add FerroUI Cargo.toml Cargo.lock && git commit
```
