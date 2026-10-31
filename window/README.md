# {{project-name | pascal_case}}

A window: `{{project-name | pascal_case}}.xaml` and its class, `{{project-name | snake_case}}.rs`, in the XAML
namespace `{{namespace}}`. To add it to a project generated from these templates:

1. Move both files into the directory of the Rust module of that namespace (for the `app` template,
   `src/views/` for `{{namespace}}` when it ends in `.Views`), and delete this folder.
2. Declare the module there, in `mod.rs`:

   ```rust
   mod {{project-name | snake_case}};
   pub use {{project-name | snake_case}}::{{project-name | pascal_case}};
   ```

3. Add `{{project-name | pascal_case}}::TYPE` to the `TYPES` table of the module (registered by
   `register_types()`).

The build script of the project compiles every `*.xaml` file below `src/`: the document is the module
`crate::compiled_{{project-name | snake_case}}`, whose `populate` the constructor calls. Open the window with:

```rust
let window = {{project-name | pascal_case}}::new();
window.show();
```
