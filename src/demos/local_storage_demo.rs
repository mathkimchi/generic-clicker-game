use gloo_console::{self, log};
use yew::prelude::*;

pub struct Loader {
    count: u8,
}

impl Component for Loader {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("Loader Viewed");

        html! {<>
            <p>{self.count}</p>
            <button onclick={ctx.link().callback(|_| ())}>{"Load"}</button>
        </>}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        log!("Loader Updated");
        self.count += 1;

        true
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // TODOs: load button, load area paragraph, save button, save area textbox

    html! {
        <>

        </>
    }
}
