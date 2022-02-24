use web_sys::{console, HtmlElement, MouseEvent};
use yew::prelude::*;
use yew::{html, Callback, TargetCast};

#[function_component(App)]
fn app() -> Html {
    let onmousemove = Callback::from(|e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let rect = target.get_bounding_client_rect();
            let x = (e.client_x() as f64) - rect.left();
            let y = (e.client_y() as f64) - rect.top();
            console::log_1(&format!("Left: {}, Top: {}", x, y).into());
        }
    });

    html! {
        <div id="mousemoveme" {onmousemove}>{ "console" }</div>
    }
}

fn main() {
    yew::start_app::<App>();
}
