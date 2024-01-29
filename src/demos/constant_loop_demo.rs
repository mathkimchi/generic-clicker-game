use chrono::Local;
use gloo::timers::callback::Interval;
use yew::{html, Component};

// this is a really simple example just to try an incrementing thing

const MILLISEC_PER_UPDATE: u32 = 1000; // 1 sec = 1000 ms

pub struct TimeDisplay {
    start_time: i64,
    current_time_est: i64,
    tick_count: u64,
}
pub enum TimeDisplayMessage {
    Tick,
}
impl Component for TimeDisplay {
    type Message = TimeDisplayMessage;

    type Properties = ();

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        // Based off https://github.com/yewstack/yew/blob/master/examples/boids/src/simulation.rs#L88
        let link = ctx.link().clone();
        Interval::new(MILLISEC_PER_UPDATE, move || {
            link.send_message(TimeDisplayMessage::Tick)
        })
        .forget();

        Self {
            start_time: chrono::Local::now().timestamp_millis(),
            current_time_est: chrono::Local::now().timestamp_millis(),
            tick_count: 0,
        }
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        html! {
            <>
                <p>{"Estimated Time: "} {self.current_time_est}</p>
                <p>{"Actual Time: "} {Local::now().timestamp_millis()}</p>
                <p>{"Error over real time: "} {((Local::now().timestamp_millis()-self.current_time_est) as f32)/((Local::now().timestamp_millis()-self.start_time) as f32)}</p>
            </>
        }
    }

    fn update(&mut self, _ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TimeDisplayMessage::Tick => {
                self.tick_count += 1;
                self.current_time_est += MILLISEC_PER_UPDATE as i64;
                true
            }
        }
    }
}
