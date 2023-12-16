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

    html! {
        <div class= {stylesheet}>
            <h1>{format!("Catdex")}</h1>
            <label for="file-upload"/>
            <input
                id="file-upload"
                type="file"
                accept="image/*"
                onchange={on_change}
            />
            <section class="cats">
                { for cat_list.iter().map(cat) }
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

fn cat(cat: &CatDetails) -> Html {
    html! {
        <article class="cat">
            <h3>{ format!( "{}", cat.name)}</h3>
            <img src ={
                format!("data:image;base64,{}",
                general_purpose::STANDARD.encode(&cat.image))
            } />
        </article>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
