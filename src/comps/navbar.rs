use yew::prelude::{html, Properties, function_component};
use yew_router::prelude::{Link};
use crate::Route;
use crate::comps::Cart;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {

}

#[function_component(Navbar)]
pub fn navbar(NavbarProps {}: &NavbarProps) -> Html {
  let products = vec![];

  html! {
    <nav class="bg-black p-5 text-center text-gray-400">
      <Link<Route> to={Route::Home} classes="mr-5 hover:text-gray-300">{ "Home" }</Link<Route>>
      <Link<Route> to={Route::Shop} classes="mr-5 hover:text-gray-300">{ "Shop" }</Link<Route>>
      <Link<Route> to={Route::About} classes="mr-5 hover:text-gray-300">{ "About" }</Link<Route>>
      <span class="mr-5">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 inline-block" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
      </span>
      <button>
        <Cart products={products} />
      </button>
    </nav>
  }
}