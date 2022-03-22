use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Product {
  pub id: usize,
  pub sku: String,
  pub name: String,
  pub short_description: String,
  pub description: String,
  pub images: Vec<String>,
  pub permalink: String,
}