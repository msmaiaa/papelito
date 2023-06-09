use leptos_dom::document;
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, JsValue};
use web_sys::{Event, HtmlDivElement, HtmlElement};

pub fn exec(command: &str, value: &str) -> Result<bool, wasm_bindgen::JsValue> {
    use web_sys::HtmlDocument;

    leptos_dom::document()
        .dyn_ref::<HtmlDocument>()
        .expect("couldn't get the HtmlDocument")
        .exec_command_with_show_ui_and_value(command, false, value)
}

pub fn unchecked_remove_class_from_el(document: &web_sys::Document, el_id: &str, class: &str) {
    if let Some(el) = document.get_element_by_id(el_id) {
        el.class_list().remove_1(class).expect(&format!(
            "Couldn't remove class: {class} from element: {el_id}"
        ));
    }
}

pub fn exec_format_block(separator: String) -> Result<bool, wasm_bindgen::JsValue> {
    exec("formatBlock", &format!("<{separator}>"))
}

pub enum ContextMenuPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

pub fn add_context_menu_to_el(
    element_id: &str,
    width: f64,
    height: f64,
    position: ContextMenuPosition,
) -> HtmlDivElement {
    let menu_el = document().create_element("div").unwrap();
    let menu_el = menu_el.dyn_ref::<HtmlDivElement>().unwrap();

    let target_el = document().get_element_by_id(element_id).unwrap();
    let target_el = target_el.dyn_ref::<HtmlElement>().unwrap();
    let target_el_rect = target_el.get_bounding_client_rect();

    menu_el
        .style()
        .set_property("width", &format!("{}px", width))
        .unwrap();
    let menu_id = format!("{element_id}_ctx_menu");
    menu_el.set_id(&menu_id);
    menu_el
        .style()
        .set_property("height", &format!("{}px", height))
        .unwrap();
    menu_el
        .style()
        .set_property("position", "absolute")
        .unwrap();

    let top = match position {
        ContextMenuPosition::TopLeft => target_el_rect.top() - height,
        ContextMenuPosition::TopCenter => target_el_rect.top() - height,
        ContextMenuPosition::TopRight => target_el_rect.top() - height,
        ContextMenuPosition::BottomLeft => target_el_rect.bottom(),
        ContextMenuPosition::BottomCenter => target_el_rect.bottom(),
        ContextMenuPosition::BottomRight => target_el_rect.bottom(),
    };

    let left = match position {
        ContextMenuPosition::TopLeft => target_el_rect.left(),
        ContextMenuPosition::TopCenter => {
            target_el_rect.left() + (target_el_rect.width() / 2.0) - (width / 2.0)
        }
        ContextMenuPosition::TopRight => target_el_rect.right() - width,
        ContextMenuPosition::BottomLeft => target_el_rect.left(),
        ContextMenuPosition::BottomCenter => {
            target_el_rect.left() + (target_el_rect.width() / 2.0) - (width / 2.0)
        }
        ContextMenuPosition::BottomRight => target_el_rect.right() - width,
    };
    menu_el
        .style()
        .set_property("top", &format!("{}px", top))
        .unwrap();
    menu_el
        .style()
        .set_property("left", &format!("{}px", left))
        .unwrap();

    crate::CONTEXT_MENUS
        .lock()
        .unwrap()
        .push((menu_id, vec![element_id.to_string()]));

    document().body().unwrap().append_child(&menu_el).unwrap();

    menu_el.clone()
}

static CONTEXT_MENU_COLORS: [&str; 56] = [
    "#000000", "#44B8FF", "#1E92F7", "#0074D9", "#005DC2", "#00369B", "#b3d5f4", "#444444",
    "#C3FFFF", "#9DF9FF", "#7FDBFF", "#68C4E8", "#419DC1", "#d9f4ff", "#666666", "#72FF84",
    "#4CEA5E", "#2ECC40", "#17B529", "#008E02", "#c0f0c6", "#888888", "#FFFF44", "#FFFA1E",
    "#FFDC00", "#E8C500", "#C19E00", "#fff5b3", "#aaaaaa", "#FFC95F", "#FFA339", "#FF851B",
    "#E86E04", "#C14700", "#ffdbbb", "#cccccc", "#FF857A", "#FF5F54", "#FF4136", "#E82A1F",
    "#C10300", "#ffc6c3", "#eeeeee", "#FF56FF", "#FF30DC", "#F012BE", "#D900A7", "#B20080",
    "#fbb8ec", "#ffffff", "#F551FF", "#CF2BE7", "#B10DC9", "#9A00B2", "#9A00B2", "#e8b6ef",
];

pub fn color_picker_menu(element_id: &str, width: f64, height: f64) {
    let menu = add_context_menu_to_el(element_id, width, height, ContextMenuPosition::TopCenter);
    style_color_picker_menu(&menu).unwrap();

    let menu_inner = document().create_element("div").unwrap();
    let menu_inner = menu_inner.dyn_ref::<HtmlElement>().unwrap();
    style_color_picker_menu_inner(menu_inner).unwrap();

    menu.append_child(&menu_inner).unwrap();

    for color in CONTEXT_MENU_COLORS {
        let el = document().create_element("a").unwrap();
        let el = el.dyn_ref::<HtmlElement>().unwrap();
        style_color_picker_item(el, color).unwrap();

        let on_click_item: Closure<dyn Fn(Event)> = Closure::new(|e: Event| {
            let target = e.target().unwrap();
            let target = target.dyn_ref::<HtmlElement>().unwrap();
            let color = target
                .style()
                .get_property_value("background-color")
                .unwrap();
            let _ = exec("foreColor", &color);
        });
        el.add_event_listener_with_callback("click", on_click_item.as_ref().unchecked_ref())
            .unwrap();
        on_click_item.forget();
        menu_inner.append_child(&el).unwrap();
    }
}

fn style_color_picker_menu_inner(el: &HtmlElement) -> Result<(), JsValue> {
    el.style().set_property("width", "100%")?;
    el.style().set_property("height", "100%")?;
    el.style().set_property("display", "flex")?;
    el.style().set_property("flex-wrap", "wrap")
}

fn style_color_picker_menu(el: &HtmlDivElement) -> Result<(), JsValue> {
    el.style().set_property("border", "1px solid black")?;
    el.style().set_property("background-color", "white")
}

fn style_color_picker_item(el: &HtmlElement, color: &str) -> Result<(), JsValue> {
    el.style().set_property("width", "14px")?;
    el.style().set_property("height", "14px")?;
    el.style().set_property("background-color", color)?;
    el.style().set_property("border", "2px solid #ffffff")?;
    el.style().set_property("user-select", "none")
}
