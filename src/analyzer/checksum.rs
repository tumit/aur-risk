use crate::pkgbuild::{PKGBuild};

use super::{Analyzer, Finding};

pub struct ChecksumAnalyzer;

impl Analyzer for ChecksumAnalyzer {
  fn name(&self) -> &str {
    "Checksum"
  }

  fn analyze(&self, pkgbuild: &PKGBuild) -> Vec<Finding> {
      pkgbuild.content
          .lines()
          .filter(|line| {
              line.contains("sha256sums=('SKIP')")
                  || line.contains("sha512sums=('SKIP')")
                  || line.contains("b2sums=('SKIP')")
          })
          .map(|line| Finding {
              rule: self.name().to_string(),
              message: format!(
                  "Checksum is SKIP: {}",
                  line.trim()
              ),
          })
          .collect()
  }

}