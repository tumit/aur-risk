use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AurResponse {
  pub resultcount: u32, // cspell:ignore resultcount
  pub results: Vec<AurPackage>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AurPackage {
  #[serde(rename = "Name")]
  pub name: String,

  #[serde(rename = "Maintainer")]
  pub maintainer: Option<String>,

  #[serde(rename = "NumVotes")]
  pub votes: i32,

  #[serde(rename = "Popularity")]
  pub popularity: f64,

  #[serde(rename = "OutOfDate")]
  pub out_of_date: Option<i64>,

}

pub fn get_package(name: &str) -> Result<AurPackage, Box<dyn std::error::Error>> {
  let url = format!(
    "https://aur.archlinux.org/rpc/v5/info/{}",
    name
  );

  let response = reqwest::blocking::get(url)?;

  let data: AurResponse = response.json()?;

  if data.resultcount == 0 {
    return Err("Package not found".into());
  }

  Ok(data.results[0].clone())
}