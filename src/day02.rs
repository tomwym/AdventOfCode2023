use crate::util;

#[derive(Debug)]
struct GameScore {
    red: u32,
    green: u32,
    blue: u32,
}

const MAX_CUBES: GameScore = GameScore {
    red: 12,
    green: 13, 
    blue: 14,
};

impl GameScore {
    fn valid_game_iteration(&self) -> bool {
        if self.red <= MAX_CUBES.red &&
           self.green <= MAX_CUBES.green &&
           self.blue <= MAX_CUBES.blue
        {
            true
        }
        else {
            false
        }
    }

    fn raise_max(&mut self, other: Self) -> &mut Self {
        if self.red < other.red {
            self.red = other.red;
        }
        if self.green < other.green {
            self.green = other.green;
        }
        if self.blue < other.blue {
            self.blue = other.blue;
        }
        self
    }

    fn accumulate(&self) -> u32 {
        return self.red * self.green * self.blue;
    }

    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

fn score_from_instance(game_instance: &str) -> GameScore {
    let mut score = GameScore::new();
    let cube_pull: Vec<&str> = game_instance.split(",").map(|v| v.trim_start()).collect();
    for cube_color in cube_pull {
        let pull_vec: Vec<&str> = cube_color.split(" ").collect();
        let count = pull_vec[0].parse::<u32>().unwrap();
        let color = pull_vec[1];
        match color {
            "red" => score.red += count,
            "green" => score.green += count,
            "blue" => score.blue += count,
            _ => ()
        }
    }
    score
}

fn valid_game(game: &str) -> bool {
    let game_session: Vec<&str> = game.split(";").collect();
    for game_instance in game_session {
        if !score_from_instance(game_instance).valid_game_iteration() {
            return false;
        }
    }
    true
}

pub fn part1() -> u32 {
    let vec = util::input_as_vector_string(util::file_path("02"), false);
    let mut sum = 0;

    for test in &vec {
        let as_vector: Vec<&str> = test.split(":").collect();
        let game_header = as_vector[0];
        let game_content = as_vector[1];

        let header: Vec<&str> = game_header.split(" ").collect();
        let game_number = &header[1].parse::<u32>().unwrap();
    
        if valid_game(&game_content) {
            sum += game_number;
        }
    }
    // println!("{sum}");
    sum
}

fn min_score(game: &str) -> GameScore {
    let mut minimum_cubes = GameScore {
        red: 0,
        green: 0,
        blue: 0,
    };
    let game_session: Vec<&str> = game.split(";").collect();
    for game_instance in game_session {
        minimum_cubes.raise_max(score_from_instance(game_instance));
    }
    minimum_cubes
}


pub fn part2() -> u32 {
    let vec = util::input_as_vector_string(util::file_path("02"), false);
    let mut sum = 0;

    for test in &vec {
        let as_vector: Vec<&str> = test.split(":").collect();
        let game_content = as_vector[1];    
        sum += min_score(&game_content).accumulate();
    }
    // println!("{sum}");
    sum
}