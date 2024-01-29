use yew::Renderer;

mod demos;

fn main() {
    Renderer::<demos::constant_loop_demo::TimeDisplay>::new().render();
}
