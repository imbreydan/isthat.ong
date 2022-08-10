use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>
            { "Yes" }

            </h1>
            <a id="github" href="https://github.com/imbreydan/isthat.ong/">
                <img src="https://img.shields.io/badge/Rust-101010?style=flat&logo=rust&logoColor=white" />
            </a>
        </main>
    }
}
