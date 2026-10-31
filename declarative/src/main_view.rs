//! The content of the main window, made with builders.

use ferroui_base::layout::{HorizontalAlignment, VerticalAlignment};
use ferroui_view::prelude::*;

/// A greeting and a button that counts its clicks: the count is a signal, and the text of the
/// button follows it.
pub fn main_view() -> Element {
    let clicks = signal(0u32);
    stack_panel()
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
        .spacing(16.0)
        .child(text_block().text("Welcome to FerroUI!").font_size(24.0).horizontal_alignment(HorizontalAlignment::Center))
        .child(
            button()
                .name("Count")
                .horizontal_alignment(HorizontalAlignment::Center)
                .on_click(move |_| clicks.update(|count| *count += 1))
                .child(text_block().name("CountText").text(move || clicked_text(clicks.get()))),
        )
        .into()
}

/// The text of the button after `clicks` clicks.
pub fn clicked_text(clicks: u32) -> String {
    match clicks {
        0 => "Click me".to_owned(),
        1 => "Clicked once".to_owned(),
        _ => format!("Clicked {clicks} times"),
    }
}

#[cfg(test)]
mod tests {
    use super::main_view;
    use ferroui_base::interactivity::RoutedEventArgs;
    use ferroui_base::{ObjectType, Ref, StyledElement};
    use ferroui_controls::testing::{TestServices, UnitTestApplication};
    use ferroui_controls::{Button, TextBlock};
    use ferroui_view::Owner;

    /// The control named `name` below `root`, of class `T`.
    fn find<T: ObjectType>(root: &Ref<StyledElement>, name: &str) -> Ref<T> {
        fn search(element: &Ref<StyledElement>, name: &str) -> Option<Ref<StyledElement>> {
            if element.name().as_deref() == Some(name) {
                return Some(element.clone());
            }
            element.logical_children().to_vec().iter().find_map(|child| search(child, name))
        }
        search(root, name).and_then(|found| found.cast::<T>()).unwrap_or_else(|| panic!("no control named {name}"))
    }

    #[test]
    fn the_button_counts_its_clicks() {
        // The dispatcher of the thread, which the controls post their work to.
        let _app = UnitTestApplication::start(TestServices::mock_threading_interface());
        let owner = Owner::new_root();
        let root: Ref<StyledElement> = owner.with(|| main_view().mount()).upcast();
        let text = find::<TextBlock>(&root, "CountText");
        assert_eq!(text.text().as_deref(), Some("Click me"));

        let button = find::<Button>(&root, "Count");
        button.raise_event(&RoutedEventArgs::with_event(Button::click_event()));
        button.raise_event(&RoutedEventArgs::with_event(Button::click_event()));
        assert_eq!(text.text().as_deref(), Some("Clicked 2 times"));
        owner.dispose();
    }
}
