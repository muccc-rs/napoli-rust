mod homepage;
mod service;

fn main() {
    yew::Renderer::<homepage::Page>::new().render();
}
