pub fn download(package: &str) -> Result<String, Box<dyn std::error::Error>> {
  let url = format!(
    "https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h={}",
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

pub struct PKGBuild {
  pub content: String,
  pub source_lines: Vec<String>,
  pub checksum_lines: Vec<String>,
  pub prepare_lines: Vec<String>,
  pub build_lines: Vec<String>,
  pub package_lines: Vec<String>,
}

enum Section {
  None,
  Prepare,
  Build,
  Package,
}

pub fn parse(content: String) -> PKGBuild {
  let mut pkgbuild = PKGBuild {
    content: content.clone(),
    source_lines: Vec::new(),
    checksum_lines: Vec::new(),
    prepare_lines: Vec::new(),
    build_lines: Vec::new(),
    package_lines: Vec::new()
  };

  let mut section = Section::None;
  let mut brace_depth = 0;

  for line in content.lines() {
    let trimmed = line.trim();

    if trimmed.starts_with("prepare()") {
      section = Section::Prepare;
    } else if trimmed.starts_with("build()") {
      section = Section::Build;
    } else if trimmed.starts_with("package()"){
      section = Section::Package
    }

    if !matches!(section, Section::None) {
      match section {
        Section::Prepare => {
          pkgbuild.prepare_lines.push(line.to_string());
        }
        Section::Build => {
          pkgbuild.build_lines.push(line.to_string());
        }
        Section::Package => {
          pkgbuild.package_lines.push(line.to_string());
        }
        Section::None => {}
      }
    }

    brace_depth += trimmed.matches('{').count();
    brace_depth -= trimmed.matches('}').count();

    if brace_depth == 0 {
      section = Section::None;
    }

  }

  pkgbuild
}
