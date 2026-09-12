use clap::Parser;

mod aur;
mod score;
mod report;

#[derive(Parser, Debug)]
#[command(name = "aur-risk")]
#[command(version = "0.1.0")]
#[command(about = "Analyze AUR package risk")]
struct Cli {
  package: String,

  #[arg(short, long)]
  verbose: bool,
}

fn main() {
  let cli = Cli::parse();

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

      // let risk_score = score::calculate(
      //   package.votes,
      //   package.maintainer.is_none(),
      // )
      let analysis_result = score::analyze(
        package.votes,
        package.maintainer.is_none()
      );

      report::print(&maintainer, analysis_result.score, analysis_result.level)

    }

    Err(error) => {
      eprintln!("Error: {}", error)
    }
  }
}