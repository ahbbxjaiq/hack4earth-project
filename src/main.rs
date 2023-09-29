use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <span> { "Hello World!" } </span>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
