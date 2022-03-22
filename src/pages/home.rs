use yew::{html, function_component, use_state, use_effect_with_deps, Callback};
use reqwasm::http::Request;
use crate::product::Product;
use crate::comps::ProductsList;
use crate::comps::Layout;

#[function_component(Home)]
pub fn home() -> Html {
  let products = use_state(|| vec![]);
  {
    let products = products.clone();
    use_effect_with_deps(move |_| {
      let products = products.clone();
      wasm_bindgen_futures::spawn_local(async move {
        let fetched_products: Vec<Product> = Request::get("http://admin.goodjoblahore.com/wp-json/wc/store/products")
          .send()
          .await
          .unwrap()
          .json()
          .await
          .unwrap();
        products.set(fetched_products.clone());
        // log::info!("{}", fetched_products.clone().iter().map(|x| x.clone().name).collect::<Vec<_>>().join(","));
      });
      || ()
    }, ());
  }

  let _on_add_to_cart = {
    Callback::from(move |product: Product| {
      log::info!("hi {}", product.id)
    })
  };

  html! {
    <Layout>
      <section style="height: 80vh;">
        <div class="slide h-full bg-[url(pexels-alena-darmel-7862590.jpg)] bg-cover">
          <h1>{ "Hello World" }</h1>
        </div>
      </section>
      <ProductsList products={(*products).clone()} />
    </Layout>
  }
}