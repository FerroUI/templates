# {{project-name | pascal_case}}

A resource dictionary, `{{project-name | pascal_case}}.xaml`. To add it to a project generated from these
templates, move the file below `src/` (for example to `src/resources/`), delete this folder, and merge it
into the resources of the application in `App.xaml`, by the path of the file below `src/`:

```xml
<Application.Resources>
  <ResourceDictionary>
    <ResourceDictionary.MergedDictionaries>
      <ResourceInclude Source="ferres://<Assembly>/resources/{{project-name | pascal_case}}.xaml" />
    </ResourceDictionary.MergedDictionaries>
  </ResourceDictionary>
</Application.Resources>
```

`<Assembly>` is the `name` of `ASSEMBLY` in `register_types.rs`. The build script of the project compiles
every `*.xaml` file below `src/`; a document without a class is reached by its URI.
