mod analyzer;
mod aur;
mod pkgbuild;
mod report;
mod score;

use analyzer::{Analyzer, ChecksumAnalyzer, DangerousCommandAnalyzer, SuspiciousSourceAnalyzer, InstallScriptAnalyzer, };
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "aur-risk")]
#[command(version = "0.1.0")]
#[command(about = "Analyze AUR package risk")]
struct Cli {
    package: String,

    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let content: String = pkgbuild::download(&cli.package)?;

    let pkgbuild = pkgbuild::parse(content);

    println!("Analyzing {}...\n", cli.package);

    let analyzers: Vec<Box<dyn Analyzer>> = vec![
        Box::new(DangerousCommandAnalyzer),
        Box::new(SuspiciousSourceAnalyzer),
        Box::new(ChecksumAnalyzer),
        Box::new(InstallScriptAnalyzer),
    ];

    let mut findings = Vec::new();

    for analyzer in &analyzers {
        let results = analyzer.analyze(&pkgbuild);
        if results.is_empty() {
            println!("✓ {:<20} 0 findings", analyzer.name());
        } else {
            println!("✗ {:<20} {} finding(s)", analyzer.name(), results.len());
        }

        findings.extend(results);
    }

    println!();

    if findings.is_empty() {
        println!("All analyzers passed.");
    } else {
        for finding in &findings {
            println!("[{}] {}", finding.rule, finding.message);
        }

        println!();
        println!("{} finding(s) detected.", findings.len());
    }

    match aur::get_package(&cli.package) {
        Ok(package) => {
            println!("Package: {}", package.name);

            let maintainer = match &package.maintainer {
                Some(name) => name.clone(),
                None => String::from("unknown"),
            };
            println!("Maintainer: {}", maintainer);

            println!("Votes: {}", package.votes);
            println!("Popularity: {}", package.popularity);

            let analysis_result = score::analyze(package.votes, package.maintainer.is_none());

            report::print(&maintainer, analysis_result.score, analysis_result.level)
        }

        Err(error) => {
            eprintln!("Error: {}", error)
        }
    }

    Ok(())
}
