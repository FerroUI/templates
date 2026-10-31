// The application id of the mobile hosts: com.example.<project name> unless one was given, and the
// display name of the application.
let name = variable::get("project-name");
let id = variable::get("app_id");
if id == "" {
    let simple = "";
    for c in name.to_lower().chars() {
        if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') {
            simple += c;
        }
    }
    if simple == "" || (simple[0] >= '0' && simple[0] <= '9') {
        simple = "app" + simple;
    }
    variable::set("app_id", "com.example." + simple);
}
if !(variable::get("desktop") || variable::get("browser") || variable::get("android") || variable::get("ios")) {
    throw "choose at least one host (desktop, browser, android or ios)";
}
