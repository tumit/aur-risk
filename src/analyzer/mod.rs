#[derive(Debug)]
pub struct Finding {
  pub rule: String,
  pub message: String,
}

pub trait Analyzer {
  fn name(&self) -> &str;
  fn analyze(&self, pkgbuild: &PKGBuild) -> Vec<Finding>;
}

mod dangerous_command;
pub use dangerous_command::DangerousCommandAnalyzer;

mod suspicious_source;
pub use suspicious_source::SuspiciousSourceAnalyzer;

mod checksum;
pub use checksum::ChecksumAnalyzer;

mod install_script;
pub use install_script::InstallScriptAnalyzer;

use crate::pkgbuild::PKGBuild;

// pub fn analyze_all(
//   content: &str,
//   analyzers: &[Box<dyn Analyzer>],
// ) -> Vec<Finding> {
//   analyzers
//     .iter()
//     .flat_map(|analyzer| {
//       analyzer.analyze(content)
//     })
//     .collect()

// }