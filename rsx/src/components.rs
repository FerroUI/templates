//! The components of the application, written with `rsx!`.

use ferroui_base::layout::{HorizontalAlignment, VerticalAlignment};
use ferroui_view::prelude::*;

/// The content of the main window.
#[component]
pub fn MainView() -> Element {
    rsx! {
        StackPanel { horizontal_alignment: HorizontalAlignment::Center, vertical_alignment: VerticalAlignment::Center, spacing: 16.0,
            TextBlock { text: "Welcome to FerroUI!", font_size: 24.0, horizontal_alignment: HorizontalAlignment::Center }
            Counter {}
            Greeting {}
        }
    }
}

/// A button that counts its clicks. The component runs once: the count is a signal, and the
/// text of the button follows it.
#[component]
pub fn Counter() -> Element {
    let clicks = signal(0u32);
    rsx! {
        Button { name: "Count", horizontal_alignment: HorizontalAlignment::Center, on_click: move |_| clicks.update(|count| *count += 1),
            TextBlock { name: "CountText", text: move || clicked_text(clicks.get()) }
        }
    }
}

/// A field for a name and a greeting. The component runs again whenever the name changes (its
/// state is a hook, `use_signal`), and the reconciler patches the text of the greeting.
#[component(rerender)]
pub fn Greeting() -> Element {
    let name = use_signal(String::new);
    let greeting = greeting_text(&name.get());
    rsx! {
        StackPanel { spacing: 8.0, horizontal_alignment: HorizontalAlignment::Center,
            TextBox { name: "Name", placeholder_text: "Your name", width: 220.0, text: name.two_way() }
            TextBlock { name: "GreetingText", horizontal_alignment: HorizontalAlignment::Center, text: greeting }
        }
    }
}

/// The text of the button after `clicks` clicks.
pub fn clicked_text(clicks: u32) -> String {
    match clicks {
        0 => "Click me".to_owned(),
        1 => "Clicked once".to_owned(),
        _ => format!("Clicked {clicks} times"),
    }
}

/// The greeting for `name`.
pub fn greeting_text(name: &str) -> String {
    match name.trim() {
        "" => "Hello!".to_owned(),
        name => format!("Hello, {name}!"),
    }
}

#[cfg(test)]
mod tests {
    use super::MainView;
    use ferroui_base::interactivity::RoutedEventArgs;
    use ferroui_base::{ObjectType, Ref, StyledElement};
    use ferroui_controls::testing::{TestServices, UnitTestApplication};
    use ferroui_controls::{Button, TextBlock, TextBox};
    use ferroui_view::{IntoElement, Owner};

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
    fn the_counter_counts_and_the_greeting_follows_the_name() {
        // The dispatcher of the thread, which the controls post their work to.
        let _app = UnitTestApplication::start(TestServices::mock_threading_interface());
        let owner = Owner::new_root();
        let root: Ref<StyledElement> = owner.with(|| MainView().into_element().mount()).upcast();

        let button = find::<Button>(&root, "Count");
        button.raise_event(&RoutedEventArgs::with_event(Button::click_event()));
        assert_eq!(find::<TextBlock>(&root, "CountText").text().as_deref(), Some("Clicked once"));

        assert_eq!(find::<TextBlock>(&root, "GreetingText").text().as_deref(), Some("Hello!"));
        find::<TextBox>(&root, "Name").set_text(Some("Ada"));
        assert_eq!(find::<TextBlock>(&root, "GreetingText").text().as_deref(), Some("Hello, Ada!"));
        owner.dispose();
    }
}
