use wasm_bindgen::{JsValue, JsCast};
use web_sys::HtmlDocument;
use crate::util::exec;

#[derive(Clone)]
pub struct Action {
    pub title: String,
    pub icon: String,
    pub result: fn() -> Result<bool, JsValue>,
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

impl From<LeptosRteActions> for ActionsBuilder {
    fn from(actions: LeptosRteActions) -> Self {
        Self {
            actions
        }
    }
}

impl ActionsBuilder {
    pub fn new() -> Self {
        Self {
            actions: LeptosRteActions::new()
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
    }

    pub fn add_action(&mut self, action: Action) -> &mut Self {
        self.actions.0.push(action);
        self
    }

    pub fn with_bold(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Bold".to_string(),
            icon: "<b>B</b>".to_string(),
            result: || exec("bold", ""),
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
            result: || exec("italic", ""),
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
            result: || exec("underline", ""),
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
            result: || exec("strikeThrough", ""),
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
            result: || exec("formatBlock", "<pre>"),
            state: None
        })
    }

    pub fn with_heading1(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 1".to_string(),
            icon: "<b>H<sub>1</sub></b>".to_string(),
            result: || exec("formatBlock", "<h1>"),
            state: None
        })
    }

    pub fn with_heading2(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 2".to_string(),
            icon: "<b>H<sub>2</sub></b>".to_string(),
            result: || exec("formatBlock", "<h2>"),
            state: None
        })
    }

    pub fn with_heading3(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 3".to_string(),
            icon: "<b>H<sub>3</sub></b>".to_string(),
            result: || exec("formatBlock", "<h3>"),
            state: None
        })
    }

    pub fn with_heading4(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 4".to_string(),
            icon: "<b>H<sub>4</sub></b>".to_string(),
            result: || exec("formatBlock", "<h4>"),
            state: None
        })
    }

    pub fn with_heading5(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 5".to_string(),
            icon: "<b>H<sub>5</sub></b>".to_string(),
            result: || exec("formatBlock", "<h5>"),
            state: None
        })
    }

    pub fn with_heading6(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 6".to_string(),
            icon: "<b>H<sub>6</sub></b>".to_string(),
            result: || exec("formatBlock", "<h6>"),
            state: None
        })
    }

    pub fn with_horizontal_line(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Horizontal Line".to_string(),
            icon: "&#8213;".to_string(),
            result: || exec("insertHorizontalRule", ""),
            state: None
        })
    }

    pub fn with_ordered_list(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Ordered List".to_string(),
            icon: "&#35;".to_string(),
            result: || exec("insertOrderedList", ""),
            state: None
        })
    }

    pub fn with_unordered_list(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Unordered List".to_string(),
            icon: "&#8226;".to_string(),
            result: || exec("insertUnorderedList", ""),
            state: None
        })
    }

    pub fn with_link(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Link".to_string(),
            icon: "&#128279;".to_string(),
            result: || {
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
            result: || {
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
            result: || exec("formatBlock", "<p>"),
            state: None
        })
    }

    pub fn with_quote(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Quote".to_string(),
            icon: "&#8220; &#8221;".to_string(),
            result: || exec("formatBlock", "<blockQuote>"),
            state: None
        })
    }
}