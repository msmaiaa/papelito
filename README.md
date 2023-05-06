# Papelito
![license: MIT](https://img.shields.io/crates/l/syn-rsx.svg)

Papelito is a simple [WYSIWYG](https://en.wikipedia.org/wiki/WYSIWYG) editor for [leptos](https://github.com/leptos-rs/leptos).  
The library is in its early stages, "it works" but its modularity is far from what I would consider ideal, so expect breaking changes on future updates.

## Usage
```toml
[dependencies]
papelito = {git = "https://github.com/msmaiaa/papelito"}
```

```rust
use leptos::*;
use papelito::*;

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let editor_content = create_rw_signal(cx, String::new());
    
    let classes = PapelitoClasses {
        actionbar: "rte-actionbar".to_string(),
        button: "rte-button".to_string(),
        content: "rte-content".to_string(),
        selected: "rte-button-selected".to_string(),
        editor: "rte-editor".to_string(),
    };
    
    //  Use the ActionsBuilder struct to build the action bar (it is a optional parameter)
    //  let actions = ActionsBuilder::new().with_bold().with_heading1().build();
    let actions = ActionsBuilder::new().with_default_actions().build();
    
    view! {cx,
        <Papelito actions=actions content_signal=editor_content classes=classes key="my_unique_key".to_string()/>
    }
}