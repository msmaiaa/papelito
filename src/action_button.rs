use crate::action::{Action as PapelitoAction, ActionExtraData};
use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Event, MouseEvent};

#[component]
pub fn ActionButton(
    cx: Scope,
    action: PapelitoAction,
    editor_key: String,
    content_ref: NodeRef<leptos::html::Div>,
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
        let action_data = ActionExtraData {
            menu_key: key_clone.clone(),
            selected_class: class_clone.clone(),
        };
        let _ = (action.compute)(action_data);
        if let Some(state) = action.state {
            let metadata = DataHandleBtnState {
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
        content_ref.on_load(cx, move |c| {
            let metadata = DataHandleBtnState {
                button_id: btn_id.clone(),
                selected_class: selected_class.clone(),
                key: editor_key.clone(),
                state,
            };

            let content_el = c.dyn_ref::<web_sys::HtmlElement>().unwrap();

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

#[cfg(not(feature = "ssr"))]
fn handle_btn_state(data: DataHandleBtnState) {
    let button_el = document()
        .get_element_by_id(&data.button_id.clone())
        .unwrap();
    let button_el = button_el.dyn_ref::<web_sys::HtmlElement>().unwrap();

    let selected_class = data.selected_class.clone();
    let key = data.key.clone();

    let state_data = ActionExtraData {
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
struct DataHandleBtnState {
    button_id: String,
    selected_class: String,
    key: String,
    state: fn(ActionExtraData) -> Result<bool, JsValue>,
}
