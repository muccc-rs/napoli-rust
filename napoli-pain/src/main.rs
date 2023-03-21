mod service;
mod homepage;

fn main() {
    yew::Renderer::<homepage::Page>::new().render();
}