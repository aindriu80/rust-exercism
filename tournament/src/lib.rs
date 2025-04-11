pub fn tally(match_results: &str) -> String {
    let table = String::new();
    let header = "Team                           | MP |  W |  D |  L |  P";

    if match_results.is_empty() {
        return String::from(header);
    }

    use std::collections::HashMap;
    let mut team_stats: HashMap<String, (u32, u32, u32, u32, u32)> = HashMap::new();

    for line in match_results.lines() {
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 3 {
            continue;
        }

        let team1 = parts[0];
        let team2 = parts[1];
        let result = parts[2];

        // Initialize teams if they don't exist
        if !team_stats.contains_key(team1) {
            team_stats.insert(team1.to_string(), (0, 0, 0, 0, 0));
        }

        if !team_stats.contains_key(team2) {
            team_stats.insert(team2.to_string(), (0, 0, 0, 0, 0));
        }

        match result {
            "win" => {
                // Team1 wins, Team2 loses
                let mut team1_stats = *team_stats.get(team1).unwrap();
                let mut team2_stats = *team_stats.get(team2).unwrap();

                // Update matches played
                team1_stats.0 += 1;
                team2_stats.0 += 1;

                // Update wins/losses
                team1_stats.1 += 1; // Win
                team2_stats.3 += 1; // Loss

                // Update points
                team1_stats.4 += 3; // 3 points for win

                team_stats.insert(team1.to_string(), team1_stats);
                team_stats.insert(team2.to_string(), team2_stats);
            }
            "loss" => {
                // Team1 loses, Team2 wins
                let mut team1_stats = *team_stats.get(team1).unwrap();
                let mut team2_stats = *team_stats.get(team2).unwrap();

                // Update matches played
                team1_stats.0 += 1;
                team2_stats.0 += 1;

                // Update losses/wins
                team1_stats.3 += 1; // Loss
                team2_stats.1 += 1; // Win

                // Update points
                team2_stats.4 += 3; // 3 points for win

                team_stats.insert(team1.to_string(), team1_stats);
                team_stats.insert(team2.to_string(), team2_stats);
            }
            draw => {
                // Both teams draw
                let mut team1_stats = *team_stats.get(team1).unwrap();
                let mut team2_stats = *team_stats.get(team2).unwrap();

                // Update matches played
                team1_stats.0 += 1;
                team2_stats.0 += 1;

                // Update draws
                team1_stats.2 += 1;
                team2_stats.2 += 1;

                // Update points
                team1_stats.4 += 1; // 1 point for draw
                team2_stats.4 += 1; // 1 point for draw

                team_stats.insert(team1.to_string(), team1_stats);
                team_stats.insert(team2.to_string(), team2_stats);
            }
            _ => continue, //Skip invalid results
        }
    }
    // Sort teams by points (descending) and then by name (ascending)
    let mut teams: Vec<(String, (u32, u32, u32, u32, u32))> = team_stats.into_iter().collect();
    teams.sort_by(|a, b| {
        // First sort by points (descending)
        let points_comparison = b.1.4.cmp(&a.1.4);
        // If points are equal, sort by name (ascending)
        if points_comparison == std::cmp::Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            points_comparison
        }
    });

    // Format the table
    let mut table = String::from(header);

    // Format each team's stats
    for (team, stats) in teams {
        let (mp, w, d, l, p) = stats;
        table.push_str(&format!(
            "\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team, mp, w, d, l, p
        ));
    }
    table
}
