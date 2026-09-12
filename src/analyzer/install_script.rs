use super::{Analyzer, Finding};

pub struct InstallScriptAnalyzer;

impl Analyzer for InstallScriptAnalyzer {
    fn name(&self) -> &str {
        "InstallScript"
    }

    fn analyze(&self, content: &str) -> Vec<Finding> {
        let mut findings = Vec::new();

        let mut inside_package = false;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("package()") {
                inside_package = true;
                continue;
            }

            if inside_package && trimmed == "}" {
                inside_package = false;
                continue;
            }

            if inside_package {
                let dangerous = ["sudo", "systemctl", "curl", "wget", "rm -rf"];

                for command in dangerous {
                    if trimmed.contains(command) {
                        findings.push(Finding {
                            rule: self.name().to_string(),
                            message: format!("Suspicious command in package(): {}", trimmed),
                        });

                        break;
                    }
                }
            }
        }

        findings
    }
}
