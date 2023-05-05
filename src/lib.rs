use std::sync::Arc;
use leptos::*;
use leptos::ev::{Event, KeyboardEvent};
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{HtmlButtonElement, HtmlDivElement, HtmlDocument, Node};
use crate::actions::LeptosRteActions;
use lazy_static::lazy_static;

pub mod util;
pub mod actions;

#[derive(Clone)]
pub struct LeptosRteClasses {
    pub actionbar: String,
    pub button: String,
    pub content: String,
    pub selected: String,
    pub editor: String
}

type JsClosure<T> = Closure<dyn FnMut(T)>;

cfg_if::cfg_if! {
    if #[cfg(not(feature="ssr"))] {
        use std::sync::Mutex;

        type OtherElements = Vec<String>;
        type ContextMenuId = String;

        lazy_static! {
            pub static ref CONTEXT_MENUS: Mutex<Vec<(ContextMenuId, OtherElements)>> = Mutex::new(Vec::new());
        }
    }
}

#[cfg(not(feature="ssr"))]
pub fn setup() {
    let handle_click: Closure<dyn FnMut(Event)> = Closure::new(move |e: Event| {
        let target = e.target().unwrap().dyn_into::<Node>().unwrap();

        CONTEXT_MENUS.lock().unwrap().retain(|(menu_id, other_els)| {
            let menu_el = document().get_element_by_id(menu_id).unwrap();
            if !target.is_same_node(Some(&menu_el)) {
                let mut clicked_on_el = false;
                for el in other_els {
                    let _el = document().get_element_by_id(el).unwrap();
                    if target.is_same_node(Some(&_el)) {
                        clicked_on_el = true;
                    }
                }
                if !clicked_on_el {
                    let menu = menu_el.dyn_ref::<HtmlDivElement>().unwrap();
                    menu.remove();
                    return false;
                }
            }
            return true;
        })
    });

    document().add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref()).unwrap();
    handle_click.forget();
}

#[component]
pub fn LeptosRte(
        cx: Scope,
        key: String,
        content_signal: RwSignal<String>,
        classes: LeptosRteClasses,
        #[prop(optional)] actions: LeptosRteActions,
        #[prop(optional)] default_paragraph_separator: String,
        ) -> impl IntoView
    {
    use crate::util::exec;
    use crate::util::unchecked_remove_class_from_el;
    let _classes = classes.clone();
    let _key = key.clone();

    let initial_value = content_signal.get();

    create_effect(cx, move |_| {
        let default_paragraph_separator = match default_paragraph_separator.is_empty() {
            true => Arc::new("div".to_string()),
            false => Arc::new(default_paragraph_separator.clone())
        };

        let document = document();

        let action_bar_el = document.create_element("div").expect("couldn't create element");
        action_bar_el.dyn_ref::<HtmlDivElement>().unwrap().set_class_name(&classes.actionbar);

        let editor_el = document.get_element_by_id(&_key).expect("couldn't create element");
        editor_el.append_child(&action_bar_el).expect("couldn't append element");

        let _content: Arc<web_sys::Element> = Arc::new(document.create_element("div").expect("couldn't create element"));
        let editor_content_el = _content.clone();
        let editor_content_el = editor_content_el.dyn_ref::<web_sys::HtmlDivElement>().expect("couldn't cast to HtmlDivElement");
        editor_content_el.set_inner_html(&initial_value);

        editor_content_el.set_content_editable("true");
        editor_content_el.set_class_name(&classes.content);

        let separator = default_paragraph_separator.clone();
        let on_input: Closure<dyn Fn(Event)> = Closure::new(move |e: Event| {
            let first_child = e.target().unwrap().dyn_ref::<Node>().unwrap().first_child();
            match first_child {
                Some(node) => {
                    match node.node_type() {
                        3 => {
                            exec("formatBlock", &format!("<{separator}>")).expect("couldn't execute formatBlock");
                        }
                        _ => {}
                    }
                }
                None => {
                    let t = e.target().unwrap();
                    let t = t.dyn_ref::<web_sys::HtmlElement>().unwrap();
                    if t.inner_html() == "<br>" {
                        t.set_inner_html("");
                    }
                }
            }
            let t = e.target().unwrap();
            let t = t.dyn_ref::<web_sys::HtmlElement>().unwrap();
            content_signal.update(|v| *v = t.inner_html());
        });


        editor_content_el.set_oninput(Some(on_input.as_ref().unchecked_ref()));
        on_input.forget();

        let separator = default_paragraph_separator.clone();
        let on_keydown: JsClosure<Event> = Closure::new(move |e: Event| {
            let e = e.dyn_ref::<KeyboardEvent>().unwrap();
            let doc = leptos_dom::document();
            let html_doc = doc.dyn_ref::<HtmlDocument>().expect("");
            if e.key() == "Space" && html_doc.query_command_value("formatBlock").unwrap() == "blockquote" {
                exec("formatBlock", &format!("<{separator}>")).expect("couldn't execute formatBlock");
            }
        });
        editor_content_el.set_onkeydown(Some(on_keydown.as_ref().unchecked_ref()));
        on_keydown.forget();

        editor_el.append_child(&editor_content_el).expect("couldn't append element");

        for action in actions.inner().clone() {
            let classes = classes.clone();

            let _button: Arc<web_sys::Element> = Arc::new(document.create_element("button").expect("couldn't create element"));
            let button = _button.clone();
            let button = button.dyn_ref::<HtmlButtonElement>().expect("couldn't cast the button to HtmlButtonElement");
            button.set_inner_html(&action.icon);
            button.set_title(&action.title);
            button.set_id(format!("{}-rte-btn", action.title.replace(" ", "")).as_str());
            button.set_class_name(&classes.button);
            button.set_attribute("type", "button").expect(&format!("couldn't set attribute 'type' for the button for the following action: {}", action.title));

            match button.first_child() {
                Some(child) => {
                    match child.node_type() {
                        3 => {
                            let wrapper = document.create_element("span").expect("couldn't create element");
                            button.append_child(&wrapper).expect("couldn't append element");
                            wrapper.append_child(&child).expect("couldn't append element");
                            wrapper.dyn_ref::<web_sys::HtmlElement>().unwrap().set_attribute("pointer-events", "none").expect("couldn't set attribute 'pointer-events' for the button's child");
                            //  FIXME: this errors out (?)
                            let _ = button.remove_child(&child);
                        }
                        1 => {
                            child.dyn_ref::<web_sys::Element>().unwrap().set_attribute("pointer-events", "none").expect("couldn't set attribute 'pointer-events' for the button's child");
                        }
                        _ => {}
                    }
                }
                None => {
                    let wrapper = document.create_element("span").expect("couldn't create element");
                    wrapper.set_inner_html(&button.dyn_ref::<web_sys::HtmlElement>().unwrap().inner_html());
                    wrapper.dyn_ref::<web_sys::HtmlElement>().unwrap().set_attribute("pointer-events", "none").expect("couldn't set attribute 'pointer-events' for the button's child");
                    button.append_child(&wrapper).expect("couldn't append element");
                }
            }

            let on_click_content = _content.clone();
            let title = action.title.clone();
            let on_click: JsClosure<Event> = Closure::new(move |_| {
                let content = on_click_content.dyn_ref::<HtmlDivElement>().unwrap();
                (action.compute)().expect(&format!("couldn't execute the result for the '{}' action", title));
                content.focus().expect("couldn't focus on the text editor's content");
            });
            button.set_onclick(Some(on_click.as_ref().unchecked_ref()));
            on_click.forget();

            match action.state {
                Some(state) => {
                    let handler_button = _button.clone();
                    let handler: JsClosure<Event> = Closure::new(move |e: Event| {
                        e.stop_propagation();
                        let button = handler_button.dyn_ref::<HtmlButtonElement>().unwrap();
                        match state() {
                            Ok(true) => {
                                button.class_list().add_1(&classes.selected).expect(&format!("couldn't add the 'selected' class to the button of the following action: {}", action.title));
                                let document = leptos_dom::document();
                                match action.title.as_str() {
                                    "Justify Left" => {
                                        unchecked_remove_class_from_el(&document, "JustifyCenter-rte-btn", &classes.selected);
                                        unchecked_remove_class_from_el(&document, "JustifyRight-rte-btn", &classes.selected);
                                    }
                                    "Justify Center" => {
                                        unchecked_remove_class_from_el(&document, "JustifyLeft-rte-btn", &classes.selected);
                                        unchecked_remove_class_from_el(&document, "JustifyRight-rte-btn", &classes.selected);
                                    }
                                    "Justify Right" => {
                                        unchecked_remove_class_from_el(&document, "JustifyLeft-rte-btn", &classes.selected);
                                        unchecked_remove_class_from_el(&document, "JustifyCenter-rte-btn", &classes.selected);
                                    }
                                    _ => {}
                                }
                            },
                            _ =>  {
                                button.class_list().remove_1(&classes.selected).expect(&format!("couldn't remove the 'selected' class to the button of the following action: {}", action.title));
                            }
                        };

                    });
                    let editor_content_el = _content.clone();
                    let editor_content_el = editor_content_el.dyn_ref::<HtmlDivElement>().expect("couldn't the editor element cast to HtmlDivElement");
                    editor_content_el.add_event_listener_with_callback("keyup", handler.as_ref().unchecked_ref()).expect("couldn't add event listener");
                    editor_content_el.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref()).expect("couldn't add event listener");
                    button.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).expect("couldn't add event listener");
                    handler.forget();
                }
                None => {}
            }
            action_bar_el.append_child(&button).expect("couldn't append element");
        }

        exec("defaultParagraphSeparator", &default_paragraph_separator).expect("couldn't execute defaultParagraphSeparator");
    });

    view! { cx,
        <div class=_classes.editor id=key></div>
    }
}