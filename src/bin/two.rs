#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn rounds(&self) -> &[Round] {
        &self.rounds
    }

    pub fn push_round(&mut self, round: Round) {
        self.rounds.push(round);
    }

    pub fn new(id: u32) -> Self {
        Self {
            id,
            rounds: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
struct Round {
    amounts: Amounts,
}

impl Round {
    pub fn amounts(&self) -> Amounts {
        self.amounts
    }

    pub fn amounts_mut(&mut self) -> &mut Amounts {
        &mut self.amounts
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Amounts {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl Amounts {
    pub fn set(&mut self, name: &str, amount: u32) {
        match name {
            "red" => {
                self.red = amount;
            }
            "blue" => {
                self.blue = amount;
            }
            "green" => {
                self.green = amount;
            }
            _ => {
                unreachable!("unknown color {name} w/ amount {amount}");
            }
        }
    }

    pub fn max_per_color(self, other: Amounts) -> Amounts {
        Amounts {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
        }
    }

    pub fn power(self) -> u64 {
        self.red as u64 * self.green as u64 * self.blue as u64
    }

    pub fn check_validity(self, limit: Amounts) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }
}

fn process_line(text: &str) -> Game {
    let mut split = text.split(':');
    let game_and_id = split.next().unwrap();
    let mut second_split = game_and_id.split_whitespace();
    second_split.next(); // Game

    let id = second_split.next().unwrap(); // id
    let id = id.parse::<u32>().unwrap();

    let mut game = Game::new(id);

    let rounds_content = split.next().unwrap();
    for round_str in rounds_content.split(';') {
        let mut round = Round::default();
        for amount in round_str.split(',') {
            let amount_str = amount.trim();
            let mut split = amount_str.split_whitespace();
            let num = split.next().unwrap().parse().unwrap();
            let name = split.next().unwrap();
            round.amounts_mut().set(name, num);
        }
        game.push_round(round)
    }

    game
}

fn check_validity(game: &Game, limits: Amounts) -> bool {
    let valid = game
        .rounds()
        .iter()
        .all(|r| r.amounts().check_validity(limits));

    println!("game {} is valid: {}", game.id(), valid);
    valid
}

fn main() {
    let input = std::fs::read_to_string("input/two_training.txt").unwrap();
    let input = std::fs::read_to_string("input/two.txt").unwrap();

    let limits = Amounts {
        red: 12,
        blue: 14,
        green: 13,
    };

    let games = input.split('\n').map(process_line).collect::<Vec<_>>();

    let mut sum = 0;
    let mut power_sum = 0;
    for game in &games {
        if check_validity(game, limits) {
            sum += game.id();
        }

        let min_amount = game
            .rounds()
            .iter()
            .map(Round::amounts)
            .reduce(Amounts::max_per_color)
            .unwrap();
        power_sum += min_amount.power();
    }

    println!("total sum of valid game ids: {sum}");
    println!("total power sum of all games: {power_sum}");
}
