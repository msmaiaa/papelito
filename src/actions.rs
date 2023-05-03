use wasm_bindgen::{JsValue, JsCast};
use web_sys::HtmlDocument;
use crate::util::exec;

#[derive(Clone)]
pub struct Action {
    pub title: String,
    pub icon: String,
    pub compute: fn() -> Result<bool, JsValue>,
    pub state: Option<fn() -> Result<bool, JsValue>>
}

#[derive(Clone)]
pub struct LeptosRteActions(Vec<Action>);

impl LeptosRteActions {
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

impl Default for LeptosRteActions {
    fn default() -> Self {
        ActionsBuilder::new().with_default_actions().build()
    }
}

#[derive(Clone)]
pub struct ActionsBuilder {
    actions: LeptosRteActions
}

impl ActionsBuilder {
    pub fn new() -> Self {
        Self {
            actions: LeptosRteActions::new(),
        }
    }

    pub fn build(&mut self) -> LeptosRteActions {
        self.actions.clone()
    }

    /// Inserts all the default actions
    pub fn with_default_actions(&mut self) -> &mut Self {
        self
            .with_bold()
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
    }

    pub fn add_action(&mut self, action: Action) -> &mut Self {
        self.actions.0.push(action);
        self
    }

    pub fn with_bold(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Bold".to_string(),
            icon: "<b>B</b>".to_string(),
            compute: || exec("bold", ""),
            state: Some(|| {
                leptos_dom::document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("bold")
            })
        })
    }

    pub fn with_italic(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Italic".to_string(),
            icon: "<i>I</i>".to_string(),
            compute: || exec("italic", ""),
            state: Some(|| {
                leptos_dom::document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("italic")
            })
        })
    }

    pub fn with_underline(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Underline".to_string(),
            icon: "<u>U</u>".to_string(),
            compute: || exec("underline", ""),
            state: Some(|| {
                leptos_dom::document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("underline")
            })
        })
    }

    pub fn with_strike_through(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Strikethrough".to_string(),
            icon: "<strike>S</strike>".to_string(),
            compute: || exec("strikeThrough", ""),
            state: Some(|| {
                leptos_dom::document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("strikeThrough")
            })
        })
    }

    pub fn with_code(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Code".to_string(),
            icon: "&lt;/&gt;".to_string(),
            compute: || exec("formatBlock", "<pre>"),
            state: None
        })
    }

    pub fn with_heading1(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 1".to_string(),
            icon: "<b>H<sub>1</sub></b>".to_string(),
            compute: || exec("formatBlock", "<h1>"),
            state: None
        })
    }

    pub fn with_heading2(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 2".to_string(),
            icon: "<b>H<sub>2</sub></b>".to_string(),
            compute: || exec("formatBlock", "<h2>"),
            state: None
        })
    }

    pub fn with_heading3(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 3".to_string(),
            icon: "<b>H<sub>3</sub></b>".to_string(),
            compute: || exec("formatBlock", "<h3>"),
            state: None
        })
    }

    pub fn with_heading4(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 4".to_string(),
            icon: "<b>H<sub>4</sub></b>".to_string(),
            compute: || exec("formatBlock", "<h4>"),
            state: None
        })
    }

    pub fn with_heading5(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 5".to_string(),
            icon: "<b>H<sub>5</sub></b>".to_string(),
            compute: || exec("formatBlock", "<h5>"),
            state: None
        })
    }

    pub fn with_heading6(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 6".to_string(),
            icon: "<b>H<sub>6</sub></b>".to_string(),
            compute: || exec("formatBlock", "<h6>"),
            state: None
        })
    }

    pub fn with_horizontal_line(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Horizontal Line".to_string(),
            icon: "&#8213;".to_string(),
            compute: || exec("insertHorizontalRule", ""),
            state: None
        })
    }

    pub fn with_ordered_list(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Ordered List".to_string(),
            icon: "&#35;".to_string(),
            compute: || exec("insertOrderedList", ""),
            state: None
        })
    }

    pub fn with_unordered_list(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Unordered List".to_string(),
            icon: "&#8226;".to_string(),
            compute: || exec("insertUnorderedList", ""),
            state: None
        })
    }

    pub fn with_link(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Link".to_string(),
            icon: "&#128279;".to_string(),
            compute: || {
                let url = leptos_dom::window().prompt_with_message("Enter the link URL:");
                match url {
                    Ok(Some(url)) => exec("createLink", &url),
                    _ => Ok(false)
                }
            },
            state: None
        })
    }

    pub fn with_image(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Image".to_string(),
            icon: "&#128247;".to_string(),
            compute: || {
                let url = leptos_dom::window().prompt_with_message("Enter the image URL:");
                match url {
                    Ok(Some(url)) => exec("insertImage", &url),
                    _ => Ok(false)
                }
            },
            state: None
        })
    }

    pub fn with_paragraph(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Paragraph".to_string(),
            icon: "&#182;".to_string(),
            compute: || exec("formatBlock", "<p>"),
            state: None
        })
    }

    pub fn with_quote(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Quote".to_string(),
            icon: "&#8220; &#8221;".to_string(),
            compute: || exec("formatBlock", "<blockQuote>"),
            state: None
        })
    }

    pub fn with_justify_center(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Justify Center".to_string(),
            icon: r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path d="M352 64c0-17.7-14.3-32-32-32H128c-17.7 0-32 14.3-32 32s14.3 32 32 32H320c17.7 0 32-14.3 32-32zm96 128c0-17.7-14.3-32-32-32H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H416c17.7 0 32-14.3 32-32zM0 448c0 17.7 14.3 32 32 32H416c17.7 0 32-14.3 32-32s-14.3-32-32-32H32c-17.7 0-32 14.3-32 32zM352 320c0-17.7-14.3-32-32-32H128c-17.7 0-32 14.3-32 32s14.3 32 32 32H320c17.7 0 32-14.3 32-32z"/></svg>"#.to_string(),
            compute: || exec("justifyCenter", ""),
            state: Some(|| {
                leptos_dom::document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("justifyCenter")
            })
        })
    }

    pub fn with_justify_left(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Justify Left".to_string(),
            icon: r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path d="M288 64c0 17.7-14.3 32-32 32H32C14.3 96 0 81.7 0 64S14.3 32 32 32H256c17.7 0 32 14.3 32 32zm0 256c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H256c17.7 0 32 14.3 32 32zM0 192c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 448c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z"/></svg>"#.to_string(),
            compute: || exec("justifyLeft", ""),
            state: Some(|| {
                leptos_dom::document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("justifyLeft")
            })
        })
    }

    pub fn with_justify_right(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Justify Right".to_string(),
            icon: r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path d="M448 64c0 17.7-14.3 32-32 32H192c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32zm0 256c0 17.7-14.3 32-32 32H192c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32zM0 192c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 448c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z"/></svg>"#.to_string(),
            compute: || exec("justifyRight", ""),
            state: Some(|| {
                leptos_dom::document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("justifyRight")
            })
        })
    }
}