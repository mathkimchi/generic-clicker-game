mod demos;

use yew::*;

fn main() {
    Renderer::<demos::local_storage_demo::Loader>::new().render();
}
