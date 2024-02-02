use gloo::storage::{LocalStorage, Storage};
use yew::*;

use crate::{game_logic::buy_upgrade, upgrades::get_upgrade_list};

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

        html! {
            <>
                <p style="display: inline;"> {"Score: "} {LocalStorage::get("generic-clicker-game.points").unwrap_or(0)} </p>
                <br/>
                <p style="display: inline;"> {"Helpers: "} {LocalStorage::get("generic-clicker-game.helpers").unwrap_or(0)} </p>
                <br/>
                <p style="display: inline;"> {"Helper2s: "} {LocalStorage::get("generic-clicker-game.helper2s").unwrap_or(0)} </p>
                <ul>
                { get_upgrade_list().into_iter().map(|upgrade| {
                    let link = ctx.link().clone();
                    let upgrade_clone = upgrade.clone();
                    let onclick = move |_| {buy_upgrade(&upgrade_clone.clone()); link.clone().send_message(AppMessage::Rerender)};

                    html! {
                        <li><button {onclick}>{format!("{}", upgrade)}</button></li>
                    }
                }).collect::<Html>() }
                </ul>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::Rerender => true,
        }
    }
}
