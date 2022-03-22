use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub html: String,
  #[prop_or("div".to_string())]
  pub wrapper: String
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
  let ele = gloo_utils::document().create_element(&props.wrapper.clone()).unwrap();
  ele.set_inner_html(&props.html.clone());

  Html::VRef(ele.into())
}