use std::sync::Arc;
use leptos::*;
use leptos::ev::{Event, InputEvent, KeyboardEvent};
use wasm_bindgen::{JsValue, JsCast, closure::Closure};
use web_sys::HtmlDocument;
mod actions;

cfg_if::cfg_if! {
    if #[cfg(not(feature="ssr"))] {
        fn append_child(parent: web_sys::Element, child: web_sys::Element) {
            parent.append_child(&child).unwrap();
        }

        fn create_element(name: &str) -> Result<web_sys::Element, JsValue> {
            leptos_dom::document().create_element(name)
        }

        fn exec(command: &str, value: &str) {
            let _ = leptos_dom::document()
                .dyn_ref::<HtmlDocument>()
                .expect("")
                .exec_command_with_show_ui_and_value(command, false, value);
        }
    }
}

struct Action {
    title: String,
    icon: String,
    result: Box<dyn Fn() -> ()>,
    state: Option<Box<dyn Fn() -> Result<bool, JsValue>>>
}

struct Settings {
    pub actions: Vec<Action>
}

cfg_if::cfg_if! {
    if #[cfg(not(feature="ssr"))] {
        impl Settings {
    pub fn new() -> Self {
        let actions: Vec<Action> = vec![
            Action {
                title: "Bold".to_string(),
                icon: "<b>B</b>".to_string(),
                result: Box::new(|| exec("bold", "")),
                state: Some(Box::new(|| {
                    leptos_dom::document()
                        .dyn_ref::<HtmlDocument>()
                        .expect("")
                        .query_command_state("bold")
                }))
            },
            Action {
                title: "Code".to_string(),
                icon: "&lt;/&gt;".to_string(),
                result: Box::new(|| exec("formatBlock", "<pre>")),
                state: None
            },
            Action {
                title: "Heading 1".to_string(),
                icon: "<b>H<sub>1</sub></b>".to_string(),
                result: Box::new(|| exec("formatBlock", "<h1>")),
                state: None
            },
            Action {
                title: "Heading 2".to_string(),
                icon: "<b>H<sub>2</sub></b>".to_string(),
                result: Box::new(|| exec("formatBlock", "<h2>")),
                state: None
            },
            Action {
                title: "Image".to_string(),
                icon: "&#128247;".to_string(),
                result: Box::new(|| {
                    let url = leptos_dom::window().prompt_with_message("Enter the image URL:");
                    match url {
                        Ok(Some(url)) => exec("insertImage", &url),
                        _ => {}
                    }
                }),
                state: None
            },
            Action {
                title: "Italic".to_string(),
                icon: "<i>I</i>".to_string(),
                result: Box::new(|| exec("italic", "")),
                state: None
            },
            Action {
                title: "Horizontal Line".to_string(),
                icon: "&#8213;".to_string(),
                result: Box::new(|| exec("insertHorizontalRule", "")),
                state: None
            },
            Action {
                title: "Link".to_string(),
                icon: "&#128279;".to_string(),
                result: Box::new(|| {
                    let url = leptos_dom::window().prompt_with_message("Enter the link URL:");
                    match url {
                        Ok(Some(url)) => exec("createLink", &url),
                        _ => {}
                    }
                }),
                state: None
            },
            Action {
                title: "Ordered List".to_string(),
                icon: "&#35;".to_string(),
                result: Box::new(|| exec("insertOrderedList", "")),
                state: None
            },
            Action {
                title: "Paragraph".to_string(),
                icon: "&#182;".to_string(),
                result: Box::new(|| exec("formatBlock", "<p>")),
                state: None
            },
            Action {
                title: "Quote".to_string(),
                icon: "&#8220; &#8221;".to_string(),
                result: Box::new(|| exec("formatBlock", "<blockQuote>")),
                state: None
            },
            Action {
                title: "Strike-Through".to_string(),
                icon: "<strike>S</strike>".to_string(),
                result: Box::new(|| exec("strikeThrough", "")),
                state: Some(Box::new(|| {
                    leptos_dom::document()
                        .dyn_ref::<HtmlDocument>()
                        .expect("")
                        .query_command_state("strikeThrough")
                }))
            },
            Action {
                title: "Unordered List".to_string(),
                icon: "&#8226;".to_string(),
                result: Box::new(|| exec("insertUnorderedList", "")),
                state: None
            },
            Action {
                title: "Underline".to_string(),
                icon: "<u>U</u>".to_string(),
                result: Box::new(|| exec("underline", "")),
                state: Some(Box::new(|| {
                    leptos_dom::document()
                        .dyn_ref::<HtmlDocument>()
                        .expect("")
                        .query_command_state("underline")
                }))
            }
        ];
        Self {
            actions
        }
    }
}
    }

}

#[derive(Clone)]
struct DefaultClasses {
    actionbar: String,
    button: String,
    content: String,
    selected: String,
}

impl DefaultClasses {
    fn new() -> Self {
        Self {
            actionbar: "pell-actionbar".to_string(),
            button: "pell-button".to_string(),
            content: "pell-content".to_string(),
            selected: "pell-button-selected".to_string(),
        }
    }
}


#[component]
pub fn TextEditor<F>(cx: Scope, on_change: F) -> impl IntoView
    where F: Fn(String) + 'static + Copy
    {

    create_effect(cx, move |_| {
        let settings = Settings::new();
        let classes = DefaultClasses::new();
        let default_paragraph_separator = "div";

        let document = leptos_dom::document();
        let action_bar = document.create_element("div").expect("couldn't create element");
        let mock_parent = document.get_element_by_id("texteditor").expect("couldn't create element");
        mock_parent.append_child(&action_bar).expect("couldn't append element");


        let _content: Arc<web_sys::Element> = Arc::new(document.create_element("div").expect("couldn't create element"));
        let content = _content.clone();
        let content = content.dyn_ref::<web_sys::HtmlDivElement>().unwrap();
        content.set_content_editable("true");
        content.set_class_name("texteditor");

        let on_input: Closure<dyn Fn(Event)> = Closure::new(move |e: Event| {
            let first_child = e.target().unwrap().dyn_ref::<web_sys::Node>().unwrap().first_child();
            match first_child {
                Some(node) => {
                    match node.node_type() {
                        3 => {
                            exec("formatBlock", &format!("<{default_paragraph_separator}>"));
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
            on_change(t.inner_html());
        });


        content.set_oninput(Some(on_input.as_ref().unchecked_ref()));
        on_input.forget();

        let on_keydown: Closure<dyn Fn(Event)> = Closure::new(move |e: Event| {
            let e = e.dyn_ref::<KeyboardEvent>().unwrap();
            let doc = leptos_dom::document();
            let html_doc = doc.dyn_ref::<HtmlDocument>().expect("");
            if e.key() == "Space" && html_doc.query_command_value("formatBlock").unwrap() == "blockquote" {
                //  need timeout?
                exec("formatBlock", &format!("<{default_paragraph_separator}>"));
            }
        });
        content.set_onkeydown(Some(on_keydown.as_ref().unchecked_ref()));
        on_keydown.forget();

        mock_parent.append_child(&content).expect("couldn't append element");

        for action in settings.actions {
            let classes = classes.clone();

            let _button: Arc<web_sys::Element> = Arc::new(document.create_element("button").expect("couldn't create element"));
            let button = _button.clone();
            let button = button.dyn_ref::<web_sys::HtmlButtonElement>().unwrap();
            button.set_inner_html(&action.icon);
            button.set_title(&action.title);
            button.set_class_name("texteditor-button");
            button.set_attribute("type", "button").expect("couldn't set attribute");
            let on_click_content = _content.clone();
            let on_click: Closure<dyn Fn(Event)> = Closure::new(move |_| {
                let content = on_click_content.dyn_ref::<web_sys::HtmlDivElement>().unwrap();
                (action.result)();
                content.focus();
            });
            button.set_onclick(Some(on_click.as_ref().unchecked_ref()));
            on_click.forget();

            match action.state {
                Some(s) => {
                    let handler_button = _button.clone();
                    let handler: Closure<dyn Fn(Event)> = Closure::new(move |_| {
                        let button = handler_button.dyn_ref::<web_sys::HtmlButtonElement>().unwrap();
                        match s() {
                            Ok(true) => {
                                button.class_list().add_1(&classes.selected);
                            },
                            _ =>  {
                                button.class_list().remove_1(&classes.selected);
                            }
                        };
                    });
                    let content = _content.clone();
                    let content = content.dyn_ref::<web_sys::HtmlDivElement>().unwrap();
                    content.set_onkeyup(Some(handler.as_ref().unchecked_ref()));
                    content.set_onmouseup(Some(handler.as_ref().unchecked_ref()));
                    button.set_onclick(Some(handler.as_ref().unchecked_ref()));
                    handler.forget();
                }
                None => {}
            }
            action_bar.append_child(&button).expect("couldn't append element");
        }

        exec("defaultParagraphSeparator", &default_paragraph_separator);
    });


    let (text, set_text) = create_signal(cx, "Hello World!".to_string());
    let on_change = move |ev: Event| {
        let value = event_target_value(&ev);
        set_text(value);
    };

    view! { cx,
        <div id="texteditor"></div>
    }
}