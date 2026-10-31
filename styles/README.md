# {{project-name | pascal_case}}

A set of styles, `{{project-name | pascal_case}}.xaml`. To add it to a project generated from these
templates, move the file below `src/` (for example to `src/styles/`), delete this folder, and include it
into the styles of the application in `App.xaml`, after the theme, by the path of the file below `src/`:

```xml
<Application.Styles>
  <WinUITheme />
  <StyleInclude Source="ferres://<Assembly>/styles/{{project-name | pascal_case}}.xaml" />
</Application.Styles>
```

`<Assembly>` is the `name` of `ASSEMBLY` in `register_types.rs`. The build script of the project compiles
every `*.xaml` file below `src/`; a document without a class is reached by its URI.
