use super::{Analyzer, Finding};

pub struct DangerousCommandAnalyzer;

impl Analyzer for DangerousCommandAnalyzer {

  fn name(&self) -> &str {
      "DangerousCommand"
  }

  fn analyze(&self, content: &str) -> Vec<Finding> {
    let dangerous_commands = ["curl", "wget", "sudo", "systemctl"];

    content
      .lines()
      .filter(|line| {
        dangerous_commands
          .iter()
          .any(|cmd| line.contains(cmd))
      })
      .map(|line| Finding {
        rule: self.name().to_string(),
        message: format!("Suspicious command found: {}", line.trim()),
      })
      .collect()
  }
}