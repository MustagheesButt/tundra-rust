use yew::{html, function_component, use_state_eq, use_effect_with_deps, Callback};
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlInputElement, InputEvent, Event};
use crate::comps::Layout;
use crate::comps::ProductsList;
use crate::product::Product;

fn get_value_from_input_event(e: InputEvent) -> String {
  let event: Event = e.dyn_into().unwrap_throw();
  let event_target = event.target().unwrap_throw();
  let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
  web_sys::console::log_1(&target.value().into());
  target.value()
}

#[function_component(Shop)]
pub fn shop() -> Html {
  let products = use_state_eq(|| vec![]);

  let mut p = vec![];
  for i in 1..1000 {
    p.push(
      Product {
        id: i,
        name: format!("Hornet {}", i),
        short_description: "Hornet mini description".to_string(),
        description: "Hornet full description".to_string(),
        sku: format!("HR{}", i),
        images: vec!["asdf".to_owned()],
        permalink: "asdf".to_owned()
      });
  }

  {
    let products = products.clone();
    let p = p.clone();
    use_effect_with_deps(move |_| {
      products.set(p);

      || ()
    }, ());
  }

  let on_filter = {
    let p = p.clone();
    let products = products.clone();
    Callback::from(move |input_event: InputEvent| {
      let value = get_value_from_input_event(input_event.clone());
      let filtered = p.clone().iter().filter(|x| x.name.contains(&value)).map(|x| x.clone()).collect::<Vec<_>>();

      products.set(filtered)
    })
  };

  html! {
    <Layout>
      <section style="height: 80vh;">
        <div class="slide flex flex-col justify-center items-center h-full bg-gradient-to-r from-violet-500 to-fuchsia-500 bg-cover">
          <h1 class="text-white text-6xl"> {"Shop Till You Drop"} </h1>
          <h2 class="text-gray-200 text-3xl mt-5">{ "Everything You Need, In One Place" }</h2>
        </div>
      </section>

      <section class="p-5">
        <input oninput={on_filter} placeholder="Filter by name/id" type="text" class="appearance-none block w-full bg-gray-200 text-gray-700 border rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white" />
      </section>

      <section>
      if products.len() > 0 {
        <ProductsList products={(*products).clone()} />
      } else {
        <p class="p-10 text-4xl text-gray-400 text-center">{ "Nothing to see here..." }</p>
      }
      </section>
    </Layout>
  }
}