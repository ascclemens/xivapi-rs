use crate::models::id::LinkshellId;

use super::LodestonePagination;

use ffxiv_types::World;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult {
  #[serde(flatten)]
  pub pagination: LodestonePagination,
  pub linkshells: Vec<SearchLinkshell>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchLinkshell {
  #[serde(rename = "ID")]
  pub id: LinkshellId,
  pub name: String,
  pub server: World,
}
