enum WinCondition {
    Win,
    Loss,
    Draw,
}

// For fun
impl WinCondition {
    fn value(string: &str) -> WinCondition {
        match string {
            "win" => WinCondition::Win,
            "loss" => WinCondition::Loss,
            "draw" => WinCondition::Draw,
            _ => panic!("Unknown win condition: {}", string),
        }
    }
}

struct Team {
    name: String,
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

impl Team {
    fn new(string: &str) -> Team {
        Team {
            name: string.to_string(),
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        }
    }

    fn winner(&mut self) {
        self.matches_played += 1;
        self.wins += 1;
        self.points += 3;
    }

    fn loser(&mut self) {
        self.matches_played += 1;
        self.losses += 1;
    }

    fn draw(&mut self) {
        self.matches_played += 1;
        self.draws += 1;
        self.points += 1;
    }

    fn print(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("{:<30}", self.name));
        output.push_str(&format!(" |  {}", self.matches_played));
        output.push_str(&format!(" |  {}", self.wins));
        output.push_str(&format!(" |  {}", self.draws));
        output.push_str(&format!(" |  {}", self.losses));
        output.push_str(&format!(" |  {}", self.points));
        output
    }
}

pub fn tally(match_results: &str) -> String {
    use std::collections::HashMap;
    let header = "Team                           | MP |  W |  D |  L |  P";
    let mut teams: HashMap<String, Team> = HashMap::new();

    match_results.lines().for_each(|line| {
        let part_lines: Vec<&str> = line.split(";").collect();
        let result = part_lines[2];
        let team_1: &mut Team;
        let team_2: &mut Team;

        // Maybe this can improve
        team_1 = teams
            .entry(part_lines[0].to_string())
            .or_insert(Team::new(part_lines[0]));

        set_win_condition(result, team_1, false);

        team_2 = teams
            .entry(part_lines[1].to_string())
            .or_insert(Team::new(part_lines[1]));

        set_win_condition(result, team_2, true);
    });

    // This can improve
    let mut hash_vec: Vec<(&String, &Team)> = teams.iter().collect();
    hash_vec.sort_by(|a, b| b.1.points.cmp(&a.1.points).then(a.1.name.cmp(&b.1.name)));
    let hash_vec_team = hash_vec.iter().map(|x| x.1).collect::<Vec<&Team>>();

    let mut output = String::new();
    output.push_str(header);

    hash_vec_team.iter().for_each(|t| {
        output.push_str("\n");
        output.push_str(&format!("{}", t.print()));
    });

    output
}

fn set_win_condition(result: &str, team: &mut Team, is_team_2: bool) {
    // This can improve
    match WinCondition::value(result) {
        WinCondition::Win => {
            if is_team_2 {
                team.loser();
            } else {
                team.winner();
            }
        }
        WinCondition::Loss => {
            if is_team_2 {
                team.winner();
            } else {
                team.loser();
            }
        }
        WinCondition::Draw => {
            team.draw();
        }
    }
}
