pub struct Player {
    name: String,
    score: i32,
    steps: i32,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            score: 0,
            steps: 0,
        }
    }
}

