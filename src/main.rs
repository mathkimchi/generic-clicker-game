use gloo::timers::callback::Interval;
use yew::Renderer;

mod demos;
mod game_logic;
mod game_page;

const MILLISEC_PER_UPDATE: u32 = 1000; // 1 sec = 1000 ms

fn start_game() {
    let app = Renderer::<game_page::App>::new().render();

    Interval::new(MILLISEC_PER_UPDATE, move || {
        game_logic::tick_logic();
        app.send_message(game_page::AppMessage::Rerender);
    })
    .forget();
}

fn main() {
    start_game();
}
