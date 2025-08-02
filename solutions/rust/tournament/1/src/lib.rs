use std::{collections::HashMap, fmt::Write};

#[derive(Debug, PartialEq, Eq)]
struct Team {
    name: String,
    wins: u8,
    draws: u8,
    losses: u8,
}

impl Team {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    fn has_won(&mut self) {
        self.wins += 1;
    }

    fn has_drawn(&mut self) {
        self.draws += 1;
    }

    fn has_lost(&mut self) {
        self.losses += 1;
    }

    fn played(&self) -> u8 {
        self.wins + self.draws + self.losses
    }

    fn score(&self) -> u8 {
        self.wins * 3 + self.draws
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score()
            .cmp(&other.score())
            .reverse()
            .then(self.name.cmp(&other.name))
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

trait TeamIndex {
    fn team<'a>(&'a mut self, name: &str) -> &'a mut Team;
}

impl TeamIndex for HashMap<String, Team> {
    fn team<'a>(&'a mut self, name: &str) -> &'a mut Team {
        self.entry(name.into()).or_insert_with(|| Team::new(name))
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, Team> = HashMap::new();

    for result in match_results.split_terminator('\n') {
        match result.split(";").collect::<Vec<&str>>()[..] {
            [a, b, "win"] => {
                teams.team(a).has_won();
                teams.team(b).has_lost();
            }
            [a, b, "draw"] => {
                teams.team(a).has_drawn();
                teams.team(b).has_drawn();
            }
            [a, b, "loss"] => {
                teams.team(a).has_lost();
                teams.team(b).has_won();
            }
            _ => panic!("invalid match result: {:?}", result),
        }
    }

    dbg!(&teams);

    let mut ranking: Vec<&Team> = teams.values().to_owned().collect();
    ranking.sort();

    dbg!(&ranking);

    let mut tab = String::from("Team                           | MP |  W |  D |  L |  P");

    for team in ranking {
        write!(
            &mut tab,
            "\n{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            team.name,
            team.played(),
            team.wins,
            team.draws,
            team.losses,
            team.score()
        )
        .unwrap();
    }

    tab
}
