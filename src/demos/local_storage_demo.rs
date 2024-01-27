use gloo::storage::{LocalStorage, Storage};
use gloo_console::{self, log};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

pub struct Loader;
impl Component for Loader {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("Loader Viewed");

        let value = LocalStorage::get("generic-clicker-game.test-value")
            .unwrap_or(String::from("No values so far."));

        // I started some styling, but this is just a concept so it doesn't matter
        html! {<>
            <p style="display: inline;">{"Value: "} <span style="border-style:solid; border-color:#888888;">{value}</span> </p>
            <button onclick={ctx.link().callback(|_| ())}>{"Load"}</button>
        </>}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        log!("Loader Updated");

        true
    }
}

pub struct Saver {
    body_text: String,
}
pub enum SaverMessage {
    UpdateBodyText(String),
    Save,
}
impl Component for Saver {
    type Message = SaverMessage;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            body_text: "".to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("Saver Viewed");

        html! {<>
            <input type="text" oninput={ctx.link().callback(|e: InputEvent| {
                let target = e.target().unwrap();
                let text_element: HtmlInputElement = target.unchecked_into();
                let text = text_element.value();
                SaverMessage::UpdateBodyText(text)})} />
            <button onclick={ctx.link().callback(|_| SaverMessage::Save)}>{"Save"}</button>
        </>}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SaverMessage::Save => {
                log!("Saving:", self.body_text.clone());
                // apparently setting might return an error, but i have no idea what would cause an error
                let _ =
                    LocalStorage::set("generic-clicker-game.test-value", self.body_text.clone());
                true
            }
            SaverMessage::UpdateBodyText(new_body) => {
                self.body_text = new_body;
                true
            }
        }
    }
}

#[function_component(Clearer)]
pub fn clearer() -> Html {
    let onclick = Callback::from(|_| {
        log!("Cleared");
        LocalStorage::delete("generic-clicker-game.test-value")
    });
    html! {
        <button {onclick}>{"Clear"}</button>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <h1>{"Local Storage Demo"}</h1>
            <p>{"Mess around with the buttons. The values should stay even after reloads."}</p>
            <p>{"Type \"localStorage\" in the console to see the local storage."}</p>
            <p>{"I think that each website has one local storage. I am not sure how github pages will work because it is all {username}.github.io so does every github pages website share a single localstorage?"}</p>
            <br/>
            <br/>
            <Loader/>
            <br/>
            <Saver/>
            <br/>
            <Clearer/>
        </>
    }
}
