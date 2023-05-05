use std::sync::Arc;
use leptos::*;
use leptos::ev::{Event, KeyboardEvent};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use crate::actions::{ActionData, Actions, Action as PapelitoAction};
use lazy_static::lazy_static;
use leptos_dom::{is_browser, html::{Div}};
use web_sys::{HtmlDocument, MouseEvent};
use crate::util::{unchecked_remove_class_from_el};

pub mod util;
pub mod actions;

#[derive(Clone, Debug, PartialEq)]
pub struct PapelitoClasses {
    pub actionbar: String,
    pub button: String,
    pub content: String,
    pub selected: String,
    pub editor: String
}

cfg_if::cfg_if! {
    if #[cfg(not(feature="ssr"))] {
        use std::sync::Mutex;

        type OtherElements = Vec<String>;
        type ContextMenuId = String;

        lazy_static! {
            pub static ref CONTEXT_MENUS: Mutex<Vec<(ContextMenuId, OtherElements)>> = Mutex::new(Vec::new());
            pub static ref ALREADY_INITIALIZED: Mutex<bool> = Mutex::new(false);
        }
    }
}

#[cfg(not(feature="ssr"))]
fn setup() {
    let handle_click: Closure<dyn FnMut(Event)> = Closure::new(move |e: Event| {
        let target = e.target().unwrap().dyn_into::<web_sys::Node>().unwrap();
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
                    let menu = menu_el.dyn_ref::<web_sys::HtmlDivElement>().unwrap();
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
pub fn Papelito(
        cx: Scope,
        key: String,
        content_signal: RwSignal<String>,
        classes: PapelitoClasses,
        #[prop(optional)] actions: Actions,
        #[prop(optional)] default_paragraph_separator: String,
        ) -> impl IntoView
        {
    use crate::util::exec;

    let _classes = classes.clone();
    let _key = key.clone();

    let initial_value = content_signal.get();

    let content_ref = create_node_ref::<Div>(cx);

    let default_paragraph_separator = match default_paragraph_separator.is_empty() {
        true => Arc::new("div".to_string()),
        false => Arc::new(default_paragraph_separator.clone())
    };

    if is_browser() {
        let mut initialized = ALREADY_INITIALIZED.lock().unwrap();
        if !*initialized {
            setup();
            *initialized = true;
        }
    }

    let _key_clone = key.clone();
    content_ref.on_load(cx, move |content| {
        let initial_value_clone = initial_value.clone();
        content
        .inner_html(initial_value_clone);
    });

    let separator_clone = default_paragraph_separator.clone();
    let on_content_change = move |e: Event| {
        let first_child = e.target().unwrap().dyn_ref::<web_sys::Node>().unwrap().first_child();
        match first_child {
            Some(node) => {
                match node.node_type() {
                    3 => {
                        exec("formatBlock", &format!("<{separator_clone}>")).expect("couldn't execute formatBlock");
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
    };

    let keydown_separator_clone = default_paragraph_separator.clone();
    let on_content_keydown = move |e: KeyboardEvent| {
        let doc = document();
        let html_doc = doc.dyn_ref::<HtmlDocument>().expect("");
        if e.key() == "Space" && html_doc.query_command_value("formatBlock").unwrap() == "blockquote" {
            exec("formatBlock", &format!("<{keydown_separator_clone}>")).expect("couldn't execute formatBlock");
        }
    };

    let key_clone = _key.clone();
    let selected_class = classes.selected.clone();
    view! { cx,
        <div class=_classes.editor id=key>
            <div class=_classes.actionbar>
                <For
                    each=move || actions.inner().clone()
                    key=|action| action.title.clone()
                    view = move |cx, action: PapelitoAction| {
                        view! {cx,
                            <ActionButton selected_class=selected_class.clone() content_ref=content_ref action=action editor_key=_key.clone() class=classes.button.clone()/>
                        }
                    }
                />
            </div>
            <div id=format!("{}-content", key_clone) on:keydown=on_content_keydown on:input=on_content_change class=_classes.content ref=content_ref contentEditable="true"></div>
        </div>
    }
}

#[cfg(not(feature="ssr"))]
fn handle_btn_state(data: TempStruct) {
    let button = document().get_element_by_id(&data.button_id.clone()).unwrap();
    let button = button.dyn_ref::<web_sys::HtmlElement>().unwrap();
    let title = button.get_attribute("title").unwrap();
    match (data.state)() {
        Ok(true) => {
            button.class_list().add_1(&data.selected_class).unwrap();
            let document = document();
            let justify_center_id = format!("{}-JustifyCenter-rte-btn", data.key);
            let justify_left_id = format!("{}-JustifyLeft-rte-btn", data.key);
            let justify_right_id = format!("{}-JustifyRight-rte-btn", data.key);
            match title.as_str() {
                "Justify Left" => {
                    unchecked_remove_class_from_el(&document, &justify_center_id, &data.selected_class);
                    unchecked_remove_class_from_el(&document, &justify_right_id, &data.selected_class);
                }
                "Justify Center" => {
                    unchecked_remove_class_from_el(&document, &justify_left_id, &data.selected_class);
                    unchecked_remove_class_from_el(&document, &justify_right_id, &data.selected_class);
                }
                "Justify Right" => {
                    unchecked_remove_class_from_el(&document, &justify_left_id, &data.selected_class);
                    unchecked_remove_class_from_el(&document, &justify_center_id, &data.selected_class);
                }
                _ => {}
            }
        },
        _ =>  {
            button.class_list().remove_1(&data.selected_class).unwrap();
        }
    };
}

#[derive(Clone)]
struct TempStruct {
    button_id: String,
    selected_class: String,
    key: String,
    state: fn() -> Result<bool, JsValue>
}

#[component]
fn ActionButton(cx: Scope,
                action: PapelitoAction,
                editor_key: String,
                content_ref: NodeRef<Div>,
                selected_class: String,
                #[prop(optional)] class: String) -> impl IntoView
        {

    let button_id = format!("{}-{}-rte-btn", editor_key.replace(" ", ""), action.title.replace(" ", ""));

    let key_clone = editor_key.clone();
    let id_clone = button_id.clone();
    let class_clone = selected_class.clone();
    let on_click = move |_: MouseEvent| {
        let _ = (action.compute)(ActionData {
            menu_key: key_clone.clone(),
        });
        let _ = content_ref.get_untracked().unwrap().focus();
        match action.state {
            Some(state) => {
                let metadata = TempStruct {
                    button_id: id_clone.clone(),
                    selected_class: class_clone.clone(),
                    key: key_clone.clone(),
                    state
                };
                handle_btn_state(metadata);
            }
            None => {}
        }
    };


    match action.state {
        Some(state) => {
            let btn_id = button_id.clone();
            content_ref.on_load(cx, move |_| {
                let metadata = TempStruct {
                    button_id: btn_id.clone(),
                    selected_class: selected_class.clone(),
                    key: editor_key.clone(),
                    state
                };
                let content = document().get_element_by_id(&format!("{}-content", editor_key.clone())).unwrap();
                let content = content.dyn_ref::<web_sys::HtmlElement>().unwrap();
                let handler_metadata = metadata.clone();
                let handler: Closure<dyn Fn(Event)> = Closure::new(move |e: Event| {
                    e.stop_propagation();
                    handle_btn_state(handler_metadata.clone());
                });
                content.add_event_listener_with_callback("keyup", handler.as_ref().unchecked_ref()).expect("couldn't add event listener");
                content.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref()).expect("couldn't add event listener");
                handler.forget();
            });
        }
        None => {}
    }

    view! {cx,
        <button title=action.title class=class on:click=on_click id=button_id>
            {(action.icon)(cx)}
        </button>
    }
}