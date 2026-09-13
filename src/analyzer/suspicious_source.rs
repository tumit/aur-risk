use crate::pkgbuild::PKGBuild;

use super::{Analyzer, Finding};

pub struct SuspiciousSourceAnalyzer;

impl Analyzer for SuspiciousSourceAnalyzer {
  fn name(&self) -> &str {
      "SuspiciousSource"
  }
  fn analyze(&self, pkgbuild: &PKGBuild) -> Vec<Finding> {
    pkgbuild.content
      .lines()
      .filter(|line| line.contains("http://"))
      .map(|line| Finding {
        rule: self.name().to_string(),
        message: format!("Insecure HTTP source: {}", line.trim()),
      })
      .collect()
  }
}