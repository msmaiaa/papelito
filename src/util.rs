
#[cfg(not(feature="ssr"))]
pub fn exec(command: &str, value: &str) -> Result<bool, wasm_bindgen::JsValue> {
    use web_sys::HtmlDocument;
    use wasm_bindgen::JsCast;
    leptos_dom::document()
        .dyn_ref::<HtmlDocument>()
        .expect("couldn't get the HtmlDocument")
        .exec_command_with_show_ui_and_value(command, false, value)
}

#[cfg(not(feature="ssr"))]
pub fn unchecked_remove_class_from_el(document: &web_sys::Document, el_id: &str, class: &str) {
    if let Some(el) = document.get_element_by_id(el_id) {
        el.class_list().remove_1(class).expect(&format!("Couldn't remove class: {class} from element: {el_id}"));
    }
}