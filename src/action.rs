use crate::util::{color_picker_menu, exec, unchecked_remove_class_from_el};
use leptos::{use_context, view, Scope};
use leptos_dom::{document, window, IntoView, View};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlDocument;

#[derive(Clone, Debug)]
pub struct ActionExtraData {
    pub menu_key: String,
    pub selected_class: String,
}

type ActionIcon = fn(cx: Scope) -> View;

#[derive(Clone)]
pub struct Action {
    pub title: String,
    pub icon: ActionIcon,
    pub compute: fn(ActionExtraData) -> Result<bool, JsValue>,
    pub state: Option<fn(ActionExtraData) -> Result<bool, JsValue>>,
}

#[derive(Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn inner(&self) -> &Vec<Action> {
        &self.0
    }

    pub fn get_action(&self, title: &str) -> Option<&Action> {
        self.0.iter().find(|action| action.title == title)
    }

    pub fn remove_action(&mut self, title: &str) {
        self.0.retain(|action| action.title != title);
    }
}

impl Default for Actions {
    fn default() -> Self {
        ActionsBuilder::new().with_default_actions().build()
    }
}

#[derive(Clone)]
pub struct ActionsBuilder {
    actions: Actions,
}

impl ActionsBuilder {
    pub fn new() -> Self {
        Self {
            actions: Actions::new(),
        }
    }

    pub fn build(&mut self) -> Actions {
        self.actions.clone()
    }

    /// Inserts all the default actions
    pub fn with_default_actions(&mut self) -> &mut Self {
        self.with_bold()
            .with_italic()
            .with_underline()
            .with_strike_through()
            .with_code()
            .with_heading1()
            .with_heading2()
            .with_heading3()
            .with_heading4()
            .with_heading5()
            .with_heading6()
            .with_horizontal_line()
            .with_ordered_list()
            .with_unordered_list()
            .with_link()
            .with_image()
            .with_paragraph()
            .with_quote()
            .with_justify_left()
            .with_justify_center()
            .with_justify_right()
            .with_text_color()
    }

    pub fn add_action(&mut self, action: Action) -> &mut Self {
        self.actions.0.push(action);
        self
    }

    pub fn with_bold(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Bold".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, <svg height="14px" width="14px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M0 64C0 46.3 14.3 32 32 32H80 96 224c70.7 0 128 57.3 128 128c0 31.3-11.3 60.1-30 82.3c37.1 22.4 62 63.1 62 109.7c0 70.7-57.3 128-128 128H96 80 32c-17.7 0-32-14.3-32-32s14.3-32 32-32H48V256 96H32C14.3 96 0 81.7 0 64zM224 224c35.3 0 64-28.7 64-64s-28.7-64-64-64H112V224H224zM112 288V416H256c35.3 0 64-28.7 64-64s-28.7-64-64-64H224 112z"/></svg>}
                }
                .into_view(cx)
            },
            compute: |_| exec("bold", ""),
            state: Some(|_| {
                document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("bold")
            }),
        })
    }

    pub fn with_italic(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Italic".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, <svg height="14px" width="14px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M128 64c0-17.7 14.3-32 32-32H352c17.7 0 32 14.3 32 32s-14.3 32-32 32H293.3L160 416h64c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H90.7L224 96H160c-17.7 0-32-14.3-32-32z"/></svg>}
                }
                .into_view(cx)
            },
            compute: |_| exec("italic", ""),
            state: Some(|_| {
                document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("italic")
            }),
        })
    }

    pub fn with_underline(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Underline".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, <svg height="14px" width="14px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M16 64c0-17.7 14.3-32 32-32h96c17.7 0 32 14.3 32 32s-14.3 32-32 32H128V224c0 53 43 96 96 96s96-43 96-96V96H304c-17.7 0-32-14.3-32-32s14.3-32 32-32h96c17.7 0 32 14.3 32 32s-14.3 32-32 32H384V224c0 88.4-71.6 160-160 160s-160-71.6-160-160V96H48C30.3 96 16 81.7 16 64zM0 448c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32z"/></svg>}
                }
                .into_view(cx)
            },
            compute: |_| exec("underline", ""),
            state: Some(|_| {
                document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("underline")
            }),
        })
    }

    pub fn with_strike_through(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Strikethrough".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, <svg height="14px" width="14px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M161.3 144c3.2-17.2 14-30.1 33.7-38.6c21.1-9 51.8-12.3 88.6-6.5c11.9 1.9 48.8 9.1 60.1 12c17.1 4.5 34.6-5.6 39.2-22.7s-5.6-34.6-22.7-39.2c-14.3-3.8-53.6-11.4-66.6-13.4c-44.7-7-88.3-4.2-123.7 10.9c-36.5 15.6-64.4 44.8-71.8 87.3c-.1 .6-.2 1.1-.2 1.7c-2.8 23.9 .5 45.6 10.1 64.6c4.5 9 10.2 16.9 16.7 23.9H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H270.1c-.1 0-.3-.1-.4-.1l-1.1-.3c-36-10.8-65.2-19.6-85.2-33.1c-9.3-6.3-15-12.6-18.2-19.1c-3.1-6.1-5.2-14.6-3.8-27.4zM348.9 337.2c2.7 6.5 4.4 15.8 1.9 30.1c-3 17.6-13.8 30.8-33.9 39.4c-21.1 9-51.7 12.3-88.5 6.5c-18-2.9-49.1-13.5-74.4-22.1c-5.6-1.9-11-3.7-15.9-5.4c-16.8-5.6-34.9 3.5-40.5 20.3s3.5 34.9 20.3 40.5c3.6 1.2 7.9 2.7 12.7 4.3l0 0 0 0c24.9 8.5 63.6 21.7 87.6 25.6l0 0 .2 0c44.7 7 88.3 4.2 123.7-10.9c36.5-15.6 64.4-44.8 71.8-87.3c3.6-21 2.7-40.4-3.1-58.1H335.1c7 5.6 11.4 11.2 13.9 17.2z"/></svg>}
                }
                .into_view(cx)
            },
            compute: |_| exec("strikeThrough", ""),
            state: Some(|_| {
                document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("strikeThrough")
            }),
        })
    }

    pub fn with_code(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Code".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, <svg height="14px" width="14px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M392.8 1.2c-17-4.9-34.7 5-39.6 22l-128 448c-4.9 17 5 34.7 22 39.6s34.7-5 39.6-22l128-448c4.9-17-5-34.7-22-39.6zm80.6 120.1c-12.5 12.5-12.5 32.8 0 45.3L562.7 256l-89.4 89.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0l112-112c12.5-12.5 12.5-32.8 0-45.3l-112-112c-12.5-12.5-32.8-12.5-45.3 0zm-306.7 0c-12.5-12.5-32.8-12.5-45.3 0l-112 112c-12.5 12.5-12.5 32.8 0 45.3l112 112c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L77.3 256l89.4-89.4c12.5-12.5 12.5-32.8 0-45.3z"/></svg>}
                }
                .into_view(cx)
            },
            compute: |_| exec("formatBlock", "<pre>"),
            state: None,
        })
    }

    pub fn with_heading1(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 1".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, <b>"H"<sub>"1"</sub></b>}
                }
                .into_view(cx)
            },
            compute: |_| exec("formatBlock", "<h1>"),
            state: None,
        })
    }

    pub fn with_heading2(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 2".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, <b>"H"<sub>"2"</sub></b>}
                }
                .into_view(cx)
            },
            compute: |_| exec("formatBlock", "<h2>"),
            state: None,
        })
    }

    pub fn with_heading3(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 3".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, <b>"H"<sub>"3"</sub></b>}
                }
                .into_view(cx)
            },
            compute: |_| exec("formatBlock", "<h3>"),
            state: None,
        })
    }

    pub fn with_heading4(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 4".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, <b>"H"<sub>"4"</sub></b>}
                }
                .into_view(cx)
            },
            compute: |_| exec("formatBlock", "<h4>"),
            state: None,
        })
    }

    pub fn with_heading5(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 5".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, <b>"H"<sub>"5"</sub></b>}
                }
                .into_view(cx)
            },
            compute: |_| exec("formatBlock", "<h5>"),
            state: None,
        })
    }

    pub fn with_heading6(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 6".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, <b>"H"<sub>"6"</sub></b>}
                }
                .into_view(cx)
            },
            compute: |_| exec("formatBlock", "<h6>"),
            state: None,
        })
    }

    pub fn with_horizontal_line(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Horizontal Line".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, "―"}
                }
                .into_view(cx)
            },
            compute: |_| exec("insertHorizontalRule", ""),
            state: None,
        })
    }

    pub fn with_ordered_list(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Ordered List".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, "#"}
                }
                .into_view(cx)
            },
            compute: |_| exec("insertOrderedList", ""),
            state: None,
        })
    }

    pub fn with_unordered_list(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Unordered List".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, "•"}
                }
                .into_view(cx)
            },
            compute: |_| exec("insertUnorderedList", ""),
            state: None,
        })
    }

    pub fn with_link(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Link".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, "🔗"}
                }
                .into_view(cx)
            },
            compute: |_| {
                let url = window().prompt_with_message("Enter the link URL:");
                match url {
                    Ok(Some(url)) => exec("createLink", &url),
                    _ => Ok(false),
                }
            },
            state: None,
        })
    }

    pub fn with_image(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Image".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, "📷"}
                }
                .into_view(cx)
            },
            compute: |_| {
                let url = window().prompt_with_message("Enter the image URL:");
                match url {
                    Ok(Some(url)) => exec("insertImage", &url),
                    _ => Ok(false),
                }
            },
            state: None,
        })
    }

    pub fn with_text_color(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Text color".to_string(),
            icon: |cx: Scope| { view!{cx, <svg  width="16px" height="16px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M512 256c0 .9 0 1.8 0 2.7c-.4 36.5-33.6 61.3-70.1 61.3H344c-26.5 0-48 21.5-48 48c0 3.4 .4 6.7 1 9.9c2.1 10.2 6.5 20 10.8 29.9c6.1 13.8 12.1 27.5 12.1 42c0 31.8-21.6 60.7-53.4 62c-3.5 .1-7 .2-10.6 .2C114.6 512 0 397.4 0 256S114.6 0 256 0S512 114.6 512 256zM128 288a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm0-96a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM288 96a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm96 96a32 32 0 1 0 0-64 32 32 0 1 0 0 64z"/></svg>}}.into_view(cx),
            compute: |data| {
                color_picker_menu(&format!("{}-Textcolor-rte-btn", data.menu_key), 128., 144.);
                Ok(true)
            },
            state: None
        })
    }

    pub fn with_paragraph(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Paragraph".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, "¶"}
                }
                .into_view(cx)
            },
            compute: |_| exec("formatBlock", "<p>"),
            state: None,
        })
    }

    pub fn with_quote(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Quote".to_string(),
            icon: |cx: Scope| {
                {
                    view! {cx, "“ ”"}
                }
                .into_view(cx)
            },
            compute: |_| exec("formatBlock", "<blockQuote>"),
            state: None,
        })
    }

    pub fn with_justify_center(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Justify Center".to_string(),
            icon: |cx: Scope| { view!{cx, <svg height="16px" width="16px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M352 64c0-17.7-14.3-32-32-32H128c-17.7 0-32 14.3-32 32s14.3 32 32 32H320c17.7 0 32-14.3 32-32zm96 128c0-17.7-14.3-32-32-32H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H416c17.7 0 32-14.3 32-32zM0 448c0 17.7 14.3 32 32 32H416c17.7 0 32-14.3 32-32s-14.3-32-32-32H32c-17.7 0-32 14.3-32 32zM352 320c0-17.7-14.3-32-32-32H128c-17.7 0-32 14.3-32 32s14.3 32 32 32H320c17.7 0 32-14.3 32-32z"/></svg>}}.into_view(cx),
            compute: |_| exec("justifyCenter", ""),
            state: Some(|data| {
                let document = document();
                let res = document
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("justifyCenter");
                if let Ok(true) = res {
                    unchecked_remove_class_from_el(&document, &format!("{}-JustifyLeft-rte-btn", data.menu_key), &data.selected_class);
                    unchecked_remove_class_from_el(&document, &format!("{}-JustifyRight-rte-btn", data.menu_key), &data.selected_class);
                }
                res
            })
        })
    }

    pub fn with_justify_left(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Justify Left".to_string(),
            icon: |cx: Scope| { view!{cx, <svg height="16px" width="16px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M288 64c0 17.7-14.3 32-32 32H32C14.3 96 0 81.7 0 64S14.3 32 32 32H256c17.7 0 32 14.3 32 32zm0 256c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H256c17.7 0 32 14.3 32 32zM0 192c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 448c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z"/></svg>}}.into_view(cx),
            compute: |_| exec("justifyLeft", ""),
            state: Some(|data| {
                let document = document();
                let res = document
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("justifyLeft");
                if let Ok(true) = res {
                    unchecked_remove_class_from_el(&document, &format!("{}-JustifyRight-rte-btn", data.menu_key), &data.selected_class);
                    unchecked_remove_class_from_el(&document, &format!("{}-JustifyCenter-rte-btn", data.menu_key), &data.selected_class);
                }
                res
            })
        })
    }

    pub fn with_justify_right(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Justify Right".to_string(),
            icon: |cx: Scope| { view!{cx, <svg height="16px" width="16px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M448 64c0 17.7-14.3 32-32 32H192c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32zm0 256c0 17.7-14.3 32-32 32H192c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32zM0 192c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 448c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z"/></svg>}}.into_view(cx),
            compute: |_| exec("justifyRight", ""),
            state: Some(|data| {
                let document = document();
                let res = document
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("justifyRight");
                if let Ok(true) = res {
                    unchecked_remove_class_from_el(&document, &format!("{}-JustifyLeft-rte-btn", data.menu_key), &data.selected_class);
                    unchecked_remove_class_from_el(&document, &format!("{}-JustifyCenter-rte-btn", data.menu_key), &data.selected_class);
                }
                res
            })
        })
    }
}
