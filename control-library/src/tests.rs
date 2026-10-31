//! The tests of the library: the control, and its theme applied in a window.

use crate::{{control}};
use ferroui_base::controls::ResourceDictionary;
use ferroui_base::metadata::from_markup_value;
use ferroui_base::utilities::Uri;
use ferroui_base::Ref;
use ferroui_controls::testing::{TestServices, UnitTestApplication};
use ferroui_controls::{Control, TextBlock, Window};
use ferroui_markup_xaml::FerroXamlLoader;

#[test]
fn the_properties_keep_their_values() {
    let control = {{control}}::new();
    assert_eq!(control.header(), None);
    control.set_header(Some("Header"));
    control.set_is_highlighted(true);
    assert_eq!(control.header().as_deref(), Some("Header"));
    assert!(control.is_highlighted());
}

#[test]
fn the_control_theme_draws_the_header() {
    let _app = UnitTestApplication::start(TestServices::styled_window());
    crate::register_types();
    let uri = Uri::absolute("ferres://{{project-name | pascal_case}}/themes/Generic.xaml").expect("the URI of the theme");
    let theme = FerroXamlLoader::load(&uri, None).expect("the theme loads");
    let theme = from_markup_value::<Ref<ResourceDictionary>>(&Some(theme)).expect("the theme is a resource dictionary");

    let window = Window::new();
    window.resources().merged_dictionaries().add(theme.into());
    let control = {{control}}::new();
    control.set_header(Some("Header"));
    window.set_content(Some(Control::boxed(&control)));
    window.show();
    control.apply_template();
    let header = control
        .get_visual_descendants()
        .filter_map(|visual| visual.cast::<TextBlock>())
        .find(|text| text.name().as_deref() == Some("PART_Header"))
        .expect("the template of the control theme has its header part");
    assert_eq!(header.text().as_deref(), Some("Header"));
}
