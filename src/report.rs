use crate::score::RiskLevel;

pub fn print(
  name: &str,
  score: u32,
  level: RiskLevel,
) {
  println!("Package : {}", name);
  println!("Score : {}", score);
  println!("Level : {}", level);
}