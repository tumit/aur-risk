#[derive(Debug)]
pub enum RiskLevel {
  Low, Review, High, Critical
}

impl std::fmt::Display for RiskLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      RiskLevel::Low => write!(f, "Low"),
      RiskLevel::Review => write!(f, "Review"),
      RiskLevel::High => write!(f, "High"),
      RiskLevel::Critical => write!(f, "Critical"),
    }
  }
}

pub fn level(score: u32) -> RiskLevel {

  match score {
    0..=20 => RiskLevel::Low,
    21..=40 => RiskLevel::Review,
    41..=70 => RiskLevel::High,
    _ => RiskLevel::Critical,
  }
}

pub fn calculate(votes: i32, orphan: bool) -> u32 {

  let mut score = 0;

  if orphan {
    score += 30;
  }

  if votes < 10 {
    score += 30;
  } else if votes < 100 {
    score += 15;
  }

  score
}

pub struct AnalysisResult {
  pub score: u32,
  pub level: RiskLevel,
}

pub fn analyze(votes: i32, orphan: bool) -> AnalysisResult {
  let score = calculate(votes, orphan);
  let level = level(score);
  AnalysisResult { score, level }
}