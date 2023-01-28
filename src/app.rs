use yew::prelude::*;
#[derive(Clone, Debug, PartialEq)]
struct AppState {
    title: String,
    description: String,
    full_name: String
}

#[function_component(Home)]
pub fn home() -> Html {
    let current_state = use_context::<AppState>().expect("no ctx found");
    println!("current_state {:?}", current_state);

    html! {
        <main>
            <h1>{ current_state.title }</h1>
        </main>
    }
}

#[function_component(Nav)]
pub fn nav() -> Html {

    html! {
        <nav>
        
        </nav>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(
        || AppState {
            title: "This is my first Yew app".to_owned(),
            description: "This is my description".to_owned(),
            full_name: "Elon Aseneka Idiong'o".to_owned(),
        });

    html! {
        <ContextProvider<AppState> context={(*state).clone()}>
        <Home />
        </ContextProvider<AppState>>
    }
}
