use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    value: i32,
}

impl Default for Props {
    fn default() -> Self {
        Props { value: 0 }
    }
}

pub struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        MyComponent
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Hello, world!"} {&ctx.props().value}</h1>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<MyComponent>();
}
