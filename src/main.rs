
use yew::prelude::*;
use gloo_net::http::Request;

mod exo_info;
use exo_info::*;

//https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=select+count(pl_name)+from+ps+where+default_flag=1&format=json
//https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=select+pl_name,pl_bmassj,pl_rade,pl_insol+from+ps+where+default_flag=1&format=json


#[function_component(App)]
pub fn app() -> Html {
    
    let planets = use_state(|| vec![]);
    {
        let planets = planets.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move{
                let fetched_planets: Vec<Planet> = Request::get("TAP/sync?query=select+pl_name,pl_bmassj,pl_rade,pl_insol+from+ps+where+default_flag=1&format=json")
                .header("Access-Control-Allow-Origin", "*")
                .header("mode", "no-cors")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            planets.set(fetched_planets);
            });
            || ()
        });
    }

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <PlanetList planets = {(*planets).clone()} />
        </main>
    }
}



fn main() {
    yew::Renderer::<App>::new().render();
}
