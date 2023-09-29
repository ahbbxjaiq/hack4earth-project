use yew::prelude::*;
use gloo_net::http::Request;

#[function_component]
fn App() -> Html {
    let get_request = Request::get("https://hcpss.instructure.com/api/v1/courses/228282/assignments");
    html! {
        <div>
            <span> { format!("{:?}", get_request) } </span>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
