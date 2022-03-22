use yew::{html, function_component};
use crate::comps::Layout;

#[function_component(About)]
pub fn about() -> Html {
  html! {
    <Layout>
      <h1>{ "About Us" }</h1>
    </Layout>
  }
}