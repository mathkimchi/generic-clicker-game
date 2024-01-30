use gloo::storage::{LocalStorage, Storage};
use yew::*;

use crate::game_logic::{buy_helper, increment};

pub struct App;
pub enum AppMessage {
    Rerender,
}
impl Component for App {
    type Message = AppMessage;

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> Html {
        // using a button allows for players to spam enter, but I think that is fine
        let link = ctx.link().clone();
        let link1 = ctx.link().clone(); // FIXME: theres gotta be a less sketchy method

        html! {
            <>
                <p style="display: inline;"> {"Score: "} {LocalStorage::get("generic-clicker-game.points").unwrap_or(0)} </p>
                <button onclick={move |_| {increment(); link.clone().send_message(AppMessage::Rerender)}}> {"Increment"} </button>
                <br/>
                <p style="display: inline;"> {"Helpers: "} {LocalStorage::get("generic-clicker-game.helpers").unwrap_or(0)} </p>
                <button onclick={move |_| {buy_helper(); link1.clone().send_message(AppMessage::Rerender)}}> {"Buy Helper (+1pts/sec, -2pts)"} </button>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::Rerender => true,
        }
    }
}
