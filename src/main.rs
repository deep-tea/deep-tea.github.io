use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div id="underDevelopment">
        <img src="img/underDevelopmentSign.png" alt="" />
        <div>
        <p>{"Welcome to my humble 𝒅𝒐𝒋𝒐 🪷"}</p>
        <p>{"Soon this will form a place filled with 𝒕𝒉𝒐𝒖𝒈𝒉𝒕𝒔 🫖"}</p>
        </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
