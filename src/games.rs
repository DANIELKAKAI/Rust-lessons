pub struct Game {
    name: String,
    score: u32,
    level: u32,
}


impl Game{
    pub fn new(name :String, score :u32, level :u32) -> Self{
        return Game{name, score, level}
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_score(&self) -> u32 {
        return self.score;
    }

    pub fn get_level(&self) -> u32 {
        return self.level;
    }

    pub fn increase_level(&mut self) -> u32{
        self.level += 1;
        return self.level;
    }
}