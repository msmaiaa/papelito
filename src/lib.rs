use crate::actions::{Action as PapelitoAction, ActionData, Actions};
use crate::util::exec_format_block;
use lazy_static::lazy_static;
use leptos::ev::{Event, KeyboardEvent};
use leptos::*;
use leptos_dom::{html::Div, is_browser};
use std::sync::Arc;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{HtmlDocument, MouseEvent};

pub mod actions;
pub mod util;

#[derive(Clone, Debug, PartialEq)]
pub struct PapelitoClasses {
    pub actionbar: String,
    pub button: String,
    pub content: String,
    pub selected: String,
    pub editor: String,
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

#[cfg(not(feature = "ssr"))]
fn setup() {
    let handle_click: Closure<dyn FnMut(Event)> = Closure::new(move |e: Event| {
        let target = e.target().unwrap().dyn_into::<web_sys::Node>().unwrap();
        CONTEXT_MENUS
            .lock()
            .unwrap()
            .retain(|(menu_id, other_els)| {
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

    document()
        .add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())
        .unwrap();
    handle_click.forget();
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
const ELEMENT_TEXT_NODE: u16 = 3;

#[component]
pub fn Papelito(
    cx: Scope,
    /// A unique key for this editor instance
    key: String,
    content_signal: RwSignal<String>,
    classes: PapelitoClasses,
    #[prop(optional)] actions: Actions,
    #[prop(optional)] default_paragraph_separator: String,
) -> impl IntoView {
    use crate::util::exec;

    let _classes = classes.clone();
    let _key = key.clone();

    let initial_value = content_signal.get();

    let content_ref = create_node_ref::<Div>(cx);

    let default_paragraph_separator = match default_paragraph_separator.is_empty() {
        true => Arc::new("div".to_string()),
        false => Arc::new(default_paragraph_separator.clone()),
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
        content.inner_html(initial_value_clone);
    });

    let separator_clone = default_paragraph_separator.clone();
    let on_content_change = move |e: Event| {
        let first_child = e
            .target()
            .unwrap()
            .dyn_ref::<web_sys::Node>()
            .unwrap()
            .first_child();

        match first_child {
            Some(node) => match node.node_type() {
                ELEMENT_TEXT_NODE => {
                    exec_format_block(separator_clone.to_string())
                        .expect("couldn't execute formatBlock");
                }
                _ => {}
            },
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
        if e.key() == "Space"
            && html_doc.query_command_value("formatBlock").unwrap() == "blockquote"
        {
            exec("formatBlock", &format!("<{keydown_separator_clone}>"))
                .expect("couldn't execute formatBlock");
        }
    };

    let key_clone = _key.clone();
    let selected_class = classes.selected.clone();
    let content_unique_id = format!("{}-content", key_clone);
    view! { cx,
        <div class=_classes.editor id=key>
            <div class=_classes.actionbar>
                <For
                    each=move || actions.inner().clone()
                    key=|action| action.title.clone()
                    view = move |cx, action: PapelitoAction| {
                        view! {cx,
                            <ActionButton  selected_class=selected_class.clone() content_ref=content_ref action=action editor_key=_key.clone() class=classes.button.clone()/>
                        }
                    }
                />
            </div>
            <div id=content_unique_id on:keydown=on_content_keydown on:input=on_content_change class=_classes.content ref=content_ref contentEditable="true"></div>
        </div>
    }
}

#[cfg(not(feature = "ssr"))]
fn handle_btn_state(data: TempStruct) {
    let button_el = document()
        .get_element_by_id(&data.button_id.clone())
        .unwrap();
    let button_el = button_el.dyn_ref::<web_sys::HtmlElement>().unwrap();

    let selected_class = data.selected_class.clone();
    let key = data.key.clone();

    let state_data = ActionData {
        menu_key: key,
        selected_class,
    };

    match (data.state)(state_data) {
        Ok(true) => {
            button_el.class_list().add_1(&data.selected_class).unwrap();
        }
        _ => {
            button_el
                .class_list()
                .remove_1(&data.selected_class)
                .unwrap();
        }
    };
}

#[derive(Clone)]
struct TempStruct {
    button_id: String,
    selected_class: String,
    key: String,
    state: fn(ActionData) -> Result<bool, JsValue>,
}

#[component]
fn ActionButton(
    cx: Scope,
    action: PapelitoAction,
    editor_key: String,
    content_ref: NodeRef<Div>,
    selected_class: String,
    #[prop(optional)] class: String,
) -> impl IntoView {
    let unique_btn_id = format!(
        "{}-{}-rte-btn",
        editor_key.replace(" ", ""),
        action.title.replace(" ", "")
    );

    let unique_btn_id_clone = unique_btn_id.clone();

    let class_clone = selected_class.clone();
    let key_clone = editor_key.clone();
    let on_click_btn = move |_: MouseEvent| {
        let action_data = ActionData {
            menu_key: key_clone.clone(),
            selected_class: class_clone.clone(),
        };
        let _ = (action.compute)(action_data);
        if let Some(state) = action.state {
            let metadata = TempStruct {
                button_id: unique_btn_id_clone.clone(),
                selected_class: class_clone.clone(),
                key: key_clone.clone(),
                state,
            };
            handle_btn_state(metadata);
        }
    };

    if let Some(state) = action.state {
        let btn_id = unique_btn_id.clone();
        content_ref.on_load(cx, move |_| {
            let metadata = TempStruct {
                button_id: btn_id.clone(),
                selected_class: selected_class.clone(),
                key: editor_key.clone(),
                state,
            };

            let content_el = document()
                .get_element_by_id(&format!("{}-content", editor_key.clone()))
                .unwrap();
            let content_el = content_el.dyn_ref::<web_sys::HtmlElement>().unwrap();

            let handler_metadata = metadata.clone();

            let event_handler: Closure<dyn Fn(Event)> = Closure::new(move |e: Event| {
                e.stop_propagation();
                handle_btn_state(handler_metadata.clone());
            });

            content_el
                .add_event_listener_with_callback("keyup", event_handler.as_ref().unchecked_ref())
                .expect("couldn't add event listener");
            content_el
                .add_event_listener_with_callback("mouseup", event_handler.as_ref().unchecked_ref())
                .expect("couldn't add event listener");

            event_handler.forget();
        });
    }

    view! {cx,
        <button title=action.title class=class on:click=on_click_btn id=unique_btn_id>
            {(action.icon)(cx)}
        </button>
    }
}
