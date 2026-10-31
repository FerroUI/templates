//! The windows and views of the application (namespace `{{project-name | pascal_case}}.Views`).

use ferroui_base::TypeInfo;

mod main_window;

pub use main_window::MainWindow;

/// The classes of this namespace.
pub(crate) const TYPES: &[&TypeInfo] = &[MainWindow::TYPE];
