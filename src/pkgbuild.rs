pub fn download(package: &str) -> Result<String, Box<dyn std::error::Error>> {
  let url = format!(
    "https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h{}",
    package
  );

  let response = reqwest::blocking::get(url)?;

  if !response.status().is_success() {
    return Err(
      format!("Failed to download PKGBUILD: {}", response.status()).into()
    )
  }

  Ok(response.text()?)
}