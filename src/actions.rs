use leptos_dom::{document, window};
use web_sys::{HtmlDocument};
use wasm_bindgen::{JsValue, JsCast};
use crate::util::{color_picker_menu, exec};

#[derive(Clone, Debug)]
pub struct ActionData {
    pub menu_key: String
}

#[derive(Clone, Debug)]
pub struct Action {
    pub title: String,
    pub icon: String,
    pub compute: fn(ActionData) -> Result<bool, JsValue>,
    pub state: Option<fn() -> Result<bool, JsValue>>
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct ActionsBuilder {
    actions: Actions
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
            .with_text_color()
    }

    pub fn add_action(&mut self, action: Action) -> &mut Self {
        self.actions.0.push(action);
        self
    }

    pub fn with_bold(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Bold".to_string(),
            icon: "<b>B</b>".to_string(),
            compute: |_| exec("bold", ""),
            state: Some(|| {
                document()
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
            compute: |_| exec("italic", ""),
            state: Some(|| {
                document()
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
            compute: |_| exec("underline", ""),
            state: Some(|| {
                document()
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
            compute: |_| exec("strikeThrough", ""),
            state: Some(|| {
                document()
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
            compute: |_| exec("formatBlock", "<pre>"),
            state: None
        })
    }

    pub fn with_heading1(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 1".to_string(),
            icon: "<b>H<sub>1</sub></b>".to_string(),
            compute: |_| exec("formatBlock", "<h1>"),
            state: None
        })
    }

    pub fn with_heading2(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 2".to_string(),
            icon: "<b>H<sub>2</sub></b>".to_string(),
            compute: |_| exec("formatBlock", "<h2>"),
            state: None
        })
    }

    pub fn with_heading3(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 3".to_string(),
            icon: "<b>H<sub>3</sub></b>".to_string(),
            compute: |_| exec("formatBlock", "<h3>"),
            state: None
        })
    }

    pub fn with_heading4(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 4".to_string(),
            icon: "<b>H<sub>4</sub></b>".to_string(),
            compute: |_| exec("formatBlock", "<h4>"),
            state: None
        })
    }

    pub fn with_heading5(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 5".to_string(),
            icon: "<b>H<sub>5</sub></b>".to_string(),
            compute: |_| exec("formatBlock", "<h5>"),
            state: None
        })
    }

    pub fn with_heading6(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Heading 6".to_string(),
            icon: "<b>H<sub>6</sub></b>".to_string(),
            compute: |_| exec("formatBlock", "<h6>"),
            state: None
        })
    }

    pub fn with_horizontal_line(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Horizontal Line".to_string(),
            icon: "&#8213;".to_string(),
            compute: |_| exec("insertHorizontalRule", ""),
            state: None
        })
    }

    pub fn with_ordered_list(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Ordered List".to_string(),
            icon: "&#35;".to_string(),
            compute: |_| exec("insertOrderedList", ""),
            state: None
        })
    }

    pub fn with_unordered_list(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Unordered List".to_string(),
            icon: "&#8226;".to_string(),
            compute: |_| exec("insertUnorderedList", ""),
            state: None
        })
    }

    pub fn with_link(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Link".to_string(),
            icon: "&#128279;".to_string(),
            compute: |_| {
                let url = window().prompt_with_message("Enter the link URL:");
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
            compute: |_| {
                let url = window().prompt_with_message("Enter the image URL:");
                match url {
                    Ok(Some(url)) => exec("insertImage", &url),
                    _ => Ok(false)
                }
            },
            state: None
        })
    }

    pub fn with_text_color(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Text color".to_string(),
            icon: r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path d="M512 256c0 .9 0 1.8 0 2.7c-.4 36.5-33.6 61.3-70.1 61.3H344c-26.5 0-48 21.5-48 48c0 3.4 .4 6.7 1 9.9c2.1 10.2 6.5 20 10.8 29.9c6.1 13.8 12.1 27.5 12.1 42c0 31.8-21.6 60.7-53.4 62c-3.5 .1-7 .2-10.6 .2C114.6 512 0 397.4 0 256S114.6 0 256 0S512 114.6 512 256zM128 288a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm0-96a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM288 96a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm96 96a32 32 0 1 0 0-64 32 32 0 1 0 0 64z"/></svg>"#.to_string(),
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
            icon: "&#182;".to_string(),
            compute: |_| exec("formatBlock", "<p>"),
            state: None
        })
    }

    pub fn with_quote(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Quote".to_string(),
            icon: "&#8220; &#8221;".to_string(),
            compute: |_| exec("formatBlock", "<blockQuote>"),
            state: None
        })
    }

    pub fn with_justify_center(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Justify Center".to_string(),
            icon: r#"<svg height="16px" width="16px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path d="M352 64c0-17.7-14.3-32-32-32H128c-17.7 0-32 14.3-32 32s14.3 32 32 32H320c17.7 0 32-14.3 32-32zm96 128c0-17.7-14.3-32-32-32H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H416c17.7 0 32-14.3 32-32zM0 448c0 17.7 14.3 32 32 32H416c17.7 0 32-14.3 32-32s-14.3-32-32-32H32c-17.7 0-32 14.3-32 32zM352 320c0-17.7-14.3-32-32-32H128c-17.7 0-32 14.3-32 32s14.3 32 32 32H320c17.7 0 32-14.3 32-32z"/></svg>"#.to_string(),
            compute: |_| exec("justifyCenter", ""),
            state: Some(|| {
                document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("justifyCenter")
            })
        })
    }

    pub fn with_justify_left(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Justify Left".to_string(),
            icon: r#"<svg height="16px" width="16px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path d="M288 64c0 17.7-14.3 32-32 32H32C14.3 96 0 81.7 0 64S14.3 32 32 32H256c17.7 0 32 14.3 32 32zm0 256c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H256c17.7 0 32 14.3 32 32zM0 192c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 448c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z"/></svg>"#.to_string(),
            compute: |_| exec("justifyLeft", ""),
            state: Some(|| {
                document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("justifyLeft")
            })
        })
    }

    pub fn with_justify_right(&mut self) -> &mut Self {
        self.add_action(Action {
            title: "Justify Right".to_string(),
            icon: r#"<svg height="16px" width="16px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path d="M448 64c0 17.7-14.3 32-32 32H192c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32zm0 256c0 17.7-14.3 32-32 32H192c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32zM0 192c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 448c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z"/></svg>"#.to_string(),
            compute: |_| exec("justifyRight", ""),
            state: Some(|| {
                document()
                    .dyn_ref::<HtmlDocument>()
                    .expect("")
                    .query_command_state("justifyRight")
            })
        })
    }
}