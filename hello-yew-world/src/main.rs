
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {<Button value=0/>}
}

#[derive(Properties, PartialEq)]
struct ButtonProp {
    value: i64
}

#[function_component(Button)]
fn increment_button(button: &ButtonProp) -> Html {
    let counter = use_state(|| button.value);
    let on_click = {
        let counter = counter.clone();
        move |_|{
            let new_value = *counter+1;
            counter.set(new_value);
        }
    };
    html! {
        <div>
            <button onclick={on_click}>
                { "+1" }
            </button>
            <p>{*counter}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}