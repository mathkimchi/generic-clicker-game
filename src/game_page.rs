use gloo::storage::{LocalStorage, Storage};
use gloo_console::log;
use yew::*;

use crate::{
    game_logic::buy_upgrade, game_page::_SimpleChildProps::update_callback,
    upgrades::get_upgrade_list,
};

#[derive(Properties, PartialEq)]
pub struct SimpleChildProps {
    pub update_callback: Callback<()>,
}
pub enum SimpleChildMessage {
    UpdateParent,
}

pub struct MoneyDiv {
    pub update_callback: Callback<()>,
}
impl Component for MoneyDiv {
    type Message = SimpleChildMessage;
    type Properties = SimpleChildProps;

    fn create(ctx: &Context<Self>) -> Self {
        MoneyDiv {
            update_callback: ctx.props().update_callback.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| {
            buy_upgrade(&get_upgrade_list()[0]);
            SimpleChildMessage::UpdateParent
        });

        html! {
            <div id="money-div" class="row">
                {"Money: $"} {LocalStorage::get("generic-clicker-game.points").unwrap_or(0i128)}
                <button {onclick}>{"Work"}</button>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.update_callback.clone().emit(());

        true
    }
}

pub struct App;
pub enum AppMessage {
    Update,
}
impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> Html {
        // FIXME: the way yew property works, I can't name this update_callback normally
        let update_callback0: Callback<()> = ctx.link().callback(|_| AppMessage::Update);

        html! {
            <>
                <div id="column1" class="col">
                    <MoneyDiv update_callback={update_callback0}/>
                    <div id="sleep-div" class="row">
                        <div id="sleep-display">
                            {"Sleepiness"}
                            <br/>
                            <progress id="file" max="100" value="70" style="width: 3.3ch;">{"76%"}</progress>{"7.6/10"}
                        </div>
                        <button onclick={ctx.link().callback(|_| AppMessage::Update)}>{"Sleep"}</button>
                    </div>
                </div>
                <span class="horizontal-divider"></span>
                <div id="column2" class="col">
                    <div id="items-div" class="lists">
                        <div>
                            {"Items"}
                            <button style="float: right;">{"Shop"}</button>
                        </div>
                        <ul>
                            <li>{"Sleeping Pill (3)"}</li>
                            <li>{"Diaper (1)"}</li>
                            <li>{"Some Other Thing (5)"}</li>
                        </ul>
                    </div>
                    <div id="upgrades-div" class="lists">
                        {"Upgrades"}
                        <ul>
                            <li>{"Dream Catcher"}</li>
                            <li>{"Sleepy 1"}</li>
                            <li>{"Hard Worker 4"}</li>
                            <li>{"Obtained Upgrade"}</li>
                            <li class="obtainable-upgrade">{"Obtainable but unobtained Upgrade"}</li>
                            <li class="unobtained-upgrade">{"Unobtained Upgrade"}</li>
                            <li class="unobtained-upgrade">{"Hard Worker 5"}</li>
                            <li class="unobtained-upgrade">{"Sleepy 2"}</li>
                            <li class="unobtained-upgrade">{"Dream Stealer"}</li>
                            <li class="unobtained-upgrade">{"Prestige"}</li>
                        </ul>
                    </div>
                    <div id="prestige-div" class="lists">
                        <div>
                            {"Prestige"}
                            <span style="float: right;">{290pp}</span>
                        </div>
                        <ul>
                            <li>{"idk"}</li>
                            <li class="obtainable-upgrade">{"idk"}</li>
                            <li class="unobtained-upgrade">{"idk"}</li>
                        </ul>
                    </div>
                </div>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("parent updatin");
        match msg {
            AppMessage::Update => true,
        }
    }
}
