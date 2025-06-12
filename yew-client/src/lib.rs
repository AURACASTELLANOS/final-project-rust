use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use rand::seq::SliceRandom;

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct Recipe {
    id: i64,
    title: String,
    description: String,
}

#[function_component(App)]
fn app() -> Html {
    let recipe = use_state(|| None::<Recipe>);

    {
        let recipe = recipe.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let fetched: Vec<Recipe> = Request::get("/backend/api/recipes")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                let random = fetched.choose(&mut rand::thread_rng());
                if let Some(r) = random.cloned() {
                    recipe.set(Some(r));
                }
            });
            || ()
        });
    }

    html! {
        <div style="font-family: sans-serif; max-width: 600px; margin: auto; padding: 2rem;">
            <h1>{ "Receta Aleatoria 🍽️" }</h1>
            {
                if let Some(r) = &*recipe {
                    html! {
                        <>
                            <h2>{ &r.title }</h2>
                            <p>{ &r.description }</p>
                        </>
                    }
                } else {
                    html! { <p>{ "Cargando receta..." }</p> }
                }
            }
        </div>
    }
}

