use std::sync::Arc;
use leptos::*;
use leptos::ev::{Event, KeyboardEvent};
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{HtmlButtonElement, HtmlDivElement, HtmlDocument, Node};
use crate::actions::LeptosRteActions;

pub mod util;
pub mod actions;

#[derive(Clone)]
pub struct LeptosRteClasses {
    actionbar: String,
    button: String,
    content: String,
    selected: String,
    editor: String
}

impl Default for LeptosRteClasses {
    fn default() -> Self {
        Self {
            actionbar: "rte-actionbar".to_string(),
            button: "rte-button".to_string(),
            content: "rte-content".to_string(),
            selected: "rte-button-selected".to_string(),
            editor: "rte-editor".to_string(),
        }
    }
}

type JsClosure<T> = Closure<dyn FnMut(T)>;

#[component]
pub fn LeptosRte(
        cx: Scope,
        key: String,
        content_signal: RwSignal<String>,
        #[prop(optional)] actions: LeptosRteActions,
        #[prop(optional)] classes: LeptosRteClasses,
        #[prop(optional)] default_paragraph_separator: String,
        ) -> impl IntoView
    {
    use crate::util::exec;
    let _classes = classes.clone();
    let _key = key.clone();

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
        editor_content_el.set_inner_html(&content_signal.get());

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
                //  need timeout?
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
            button.set_class_name(&classes.button);
            button.set_attribute("type", "button").expect(&format!("couldn't set attribute 'type' for the button for the following action: {}", action.title));

            let on_click_content = _content.clone();
            let title = action.title.clone();
            let on_click: JsClosure<Event> = Closure::new(move |_| {
                let content = on_click_content.dyn_ref::<HtmlDivElement>().unwrap();
                (action.result)().expect(&format!("couldn't execute the result for the '{}' action", title));
                content.focus().expect("couldn't focus on the text editor's content");
            });
            button.set_onclick(Some(on_click.as_ref().unchecked_ref()));
            on_click.forget();

            match action.state {
                Some(s) => {
                    let handler_button = _button.clone();
                    let handler: JsClosure<Event> = Closure::new(move |_| {
                        let button = handler_button.dyn_ref::<HtmlButtonElement>().unwrap();
                        match s() {
                            Ok(true) => {
                                button.class_list().add_1(&classes.selected).expect(&format!("couldn't add the 'selected' class to the button of the following action: {}", action.title));
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