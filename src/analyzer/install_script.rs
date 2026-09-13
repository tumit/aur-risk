use crate::pkgbuild::{PKGBuild};

use super::{Analyzer, Finding};

pub struct InstallScriptAnalyzer;

impl Analyzer for InstallScriptAnalyzer {
    fn name(&self) -> &str {
        "InstallScript"
    }

    fn analyze(&self, pkgbuild: &PKGBuild) -> Vec<Finding> {

        // let mut findings = Vec::new();

        // let mut inside_package = false;
        // let mut brace_depth = 0;

        let dangerous = ["sudo", "systemctl", "curl", "wget", "rm -rf"];

        pkgbuild
            .package_lines
            .iter()
            .filter(|line| {
                dangerous
                    .iter()
                    .any(|cmd| line.contains(cmd))
            })
            .map(|line| Finding {
                rule: self.name().to_string(),
                message: format!(
                    "Suspicious command in package(): {}",
                    line.trim()
                ),
            })
            .collect()

        // for line in pkgbuild.content.lines() {
        //     let line_trimmed = line.trim();

        //     if line_trimmed.starts_with("package()") {
        //         inside_package = true;
        //     }

        //     if inside_package {

        //         if dangerous
        //             .iter()
        //             .any(|command| line_trimmed.contains(command))
        //         {
        //             findings.push(Finding {
        //                 rule: self.name().to_string(),
        //                 message: format!(
        //                     "Suspicious command in package(): {}",
        //                     line_trimmed
        //                 ),
        //             });
        //         }
        //     }

        //     brace_depth += line_trimmed.matches('{').count();
        //     brace_depth -= line_trimmed.matches('}').count();

        //     if inside_package && brace_depth == 0 {
        //         inside_package = false;
        //     }

        // }

        // findings
    }
}
