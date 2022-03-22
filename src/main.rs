use yew::prelude::*;
use yew_router::prelude::*;

mod product;
mod comps;
mod pages;

use pages::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
  #[at("/")]
  Home,
  #[at("/shop")]
  Shop,
  #[at("/about")]
  About,
  #[not_found]
  #[at("/404")]
  NotFound,
}

fn switch(routes: &Route) -> Html {
  match routes {
    Route::Home => html! { <Home /> },
    Route::Shop => html! { <Shop /> },
    Route::About => html! { <About /> },
    Route::NotFound => html! { <h1>{ "404" }</h1> },
  }
}

#[function_component(App)]
fn app() -> Html {
  // let cart = vec![];

  html! {
    <BrowserRouter>
      <Switch<Route> render={Switch::render(switch)} />
    </BrowserRouter>
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<App>();

  log::info!("Debug mode enabled");
}
