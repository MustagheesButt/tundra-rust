use yew::prelude::*;

use crate::product::Product;
use crate::comps::SafeHtml;

#[derive(Properties, PartialEq)]
pub struct ProductsListProps {
  pub products: Vec<Product>,
}

#[function_component(ProductsList)]
pub fn products_list(ProductsListProps { products }: &ProductsListProps) -> Html {
  let products_list: Vec<Html> = products
    .iter()
    .map(|product| {
      let add_to_cart = {
        // let on_add_to_cart = on_add_to_cart.clone();
        let product = product.clone();
        Callback::from(move |_| {
          log::info!("{}", product.id);
          // on_add_to_cart.emit(product.clone())
        })
      };

      html! {
        <div class="p-5 w-1/4">
          <div class="bg-gray-400 p-2">
            <SafeHtml html={ product.name.to_string() } wrapper={ "h3" } />
            <SafeHtml html={ format!("{}", product.short_description) } wrapper={ "p" } />
            <button onclick={add_to_cart} class="p-2 bg-purple-300 hover:bg-purple-400">{ "Add to Cart" }</button>
          </div>
        </div>
      }
    })
    .collect();

    html! {
      <div class="flex flex-wrap">
        { products_list }
      </div>
    }
}