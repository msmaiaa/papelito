
#[cfg(not(feature="ssr"))]
pub fn exec(command: &str, value: &str) -> Result<bool, wasm_bindgen::JsValue> {
    use web_sys::HtmlDocument;
    use wasm_bindgen::JsCast;
    leptos_dom::document()
        .dyn_ref::<HtmlDocument>()
        .expect("couldn't get the HtmlDocument")
        .exec_command_with_show_ui_and_value(command, false, value)
}
