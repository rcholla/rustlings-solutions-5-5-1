use std::collections::HashMap;

struct Team {
  name: String,
  goals_scored: u8,
  goals_conceded: u8,
}

fn update_team_scores(
  scores: &mut HashMap<String, Team>,
  team_name: &str,
  goals_scored: u8,
  goals_conceded: u8,
) {
  scores
    .entry(team_name.to_owned())
    .and_modify(|team| {
      team.goals_scored += goals_scored;
      team.goals_conceded += goals_conceded;
    })
    .or_insert(Team {
      name: team_name.to_owned(),
      goals_scored,
      goals_conceded,
    });
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
  let mut scores: HashMap<String, Team> = HashMap::new();

  for r in results.lines() {
    let v: Vec<&str> = r.split(',').collect();
    let team_1_name = v[0].to_string();
    let team_1_score: u8 = v[2].parse().unwrap();
    let team_2_name = v[1].to_string();
    let team_2_score: u8 = v[3].parse().unwrap();

    update_team_scores(&mut scores, &team_1_name, team_1_score, team_2_score);
    update_team_scores(&mut scores, &team_2_name, team_2_score, team_1_score);
  }
  scores
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_results() -> String {
    let results = "".to_string()
      + "England,France,4,2\n"
      + "France,Italy,3,1\n"
      + "Poland,Spain,2,0\n"
      + "Germany,England,2,1\n";
    results
  }

  #[test]
  fn build_scores() {
    let scores = build_scores_table(get_results());

    let mut keys: Vec<&String> = scores.keys().collect();
    keys.sort();
    assert_eq!(
      keys,
      vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
    );
  }

  #[test]
  fn validate_team_score_1() {
    let scores = build_scores_table(get_results());
    let team = scores.get("England").unwrap();
    assert_eq!(team.goals_scored, 5);
    assert_eq!(team.goals_conceded, 4);
  }

  #[test]
  fn validate_team_score_2() {
    let scores = build_scores_table(get_results());
    let team = scores.get("Spain").unwrap();
    assert_eq!(team.goals_scored, 0);
    assert_eq!(team.goals_conceded, 2);
  }
}
