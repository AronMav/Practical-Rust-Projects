use yew::prelude::*;
use yew::{html, Html, use_state};
use std::ops::Deref;
use gloo_file::File;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FileList, HtmlInputElement};
use base64::{engine::general_purpose, Engine};
use stylist::{yew::styled_component, Style};

#[styled_component(App)]
fn app() -> Html {
    const CSS: &str = include_str!("index.css");
    let stylesheet = Style::new(CSS).unwrap();
    let cat_list = use_state(||Vec::<CatDetails>::new());

    let on_change = {
        let cat_list = cat_list.clone();
        move |e: Event| {
            let cat_list = cat_list.clone();
            spawn_local(async move {
                let input: HtmlInputElement = e.target_unchecked_into();
                let files = upload_file(input.files());
                let mut  interior_cat_list = cat_list.deref().clone();

                for file in files {
                    let new_details = CatDetails {
                        name: file.name(),
                        image: gloo_file::futures::read_as_bytes(&file)
                            .await
                            .unwrap(),
                    };
                    interior_cat_list.push(new_details)
                }
                cat_list.set(interior_cat_list);
            })
        }
    };

    let delete_cat = {
        let cat_list = cat_list.clone();
        Callback::from(move |name: String|{
            let interior_cat_list = cat_list.deref().clone();
            let new_cat_list: Vec<_> = interior_cat_list
                .into_iter()
                .filter(|cat| cat.name != name).collect();
            cat_list.set(new_cat_list);
        })
    };

    html! {
        <div class= {stylesheet}>
            <h1>{format!("Catdex")}</h1>
            <label for="file-upload"/>
            <input
                type="file"
                accept="image/*"
                onchange={on_change}
            />
            <section class="cats">
                { for cat_list.iter().map(
                    |val| cat(val, delete_cat.clone())
                )}
            </section>
        </div>
    }
}

fn upload_file(files: Option<FileList>) -> Vec<File> {
    files
        .map(|files|{
            js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from)
                .collect()
        })
        .unwrap_or_default()
}

#[derive(Clone)]
struct CatDetails {
    name: String,
    image: Vec<u8>,
}

fn cat(cat: &CatDetails, callback: Callback<String>) -> Html {
    html! {
        <article class="cat">
            <h3>{ format!( "{}", cat.name)}</h3>
            <Button text={"Delete".to_string()}
                name = {cat.name.clone()}
                on_click ={callback}
            />
            <img src ={format!(
                "data:image;base64,{}",
                general_purpose::STANDARD.encode(&cat.image))
            } />
        </article>
    }
}

#[derive(Properties, PartialEq)]
struct ButtonProp {
    text: String,
    name: String,
    on_click: Callback<String>
}

#[function_component(Button)]
fn delete_button(button: &ButtonProp) -> Html {
    let on_click = {
        let name = button.name.clone();
        let callback = button.on_click.clone();
        move |_| {
            callback.emit(name.clone())
        }
    };
    html! {
        <div>
            <button onclick = {on_click}>
                {{button.text.clone()}}
            </button>
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
