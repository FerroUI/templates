# {{project-name | pascal_case}}

A templated control: its class, `{{project-name | snake_case}}.rs`, with the styled property `Header`, and
its control theme, `{{project-name | pascal_case}}Theme.xaml`, in the XAML namespace `{{namespace}}`. To add
it to a project generated from these templates:

1. Move both files into the directory of the Rust module of that namespace (for example
   `src/controls/`), and delete this folder.
2. Declare the module there, in `mod.rs` (or at the crate root):

   ```rust
   mod {{project-name | snake_case}};
   pub use {{project-name | snake_case}}::{{project-name | pascal_case}};
   ```

3. Add `{{project-name | pascal_case}}::TYPE` to the `TYPES` table of the module (registered by
   `register_types()`).
4. Merge the control theme into the resources of the application, in `App.xaml`, by the path of the
   file below `src/`:

   ```xml
   <Application.Resources>
     <ResourceDictionary>
       <ResourceDictionary.MergedDictionaries>
         <ResourceInclude Source="ferres://<Assembly>/controls/{{project-name | pascal_case}}Theme.xaml" />
       </ResourceDictionary.MergedDictionaries>
     </ResourceDictionary>
   </Application.Resources>
   ```

   `<Assembly>` is the `name` of `ASSEMBLY` in `register_types.rs`.

Use it in a document through the namespace, `xmlns:controls="using:{{namespace}}"`:
`<controls:{{project-name | pascal_case}} Header="Title">Content</controls:{{project-name | pascal_case}}>`.
