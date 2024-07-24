use {
    serde::{Deserialize, Serialize},
    std::{env, fs},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Model {
    pub temp: Vec<String>,
    pub curr: usize,
    pub size: i32,
}
impl Model {
    pub fn default() -> Self {
        if let Ok(value) = fs::read(file()) {
            if let Ok(value) = rmp_serde::from_slice::<Self>(&value) {
                return value;
            }
        }
        Self {
            temp: Vec::new(),
            size: 100,
            curr: 0,
        }
    }
    pub fn save(&self) {
        fs::write(file(), rmp_serde::to_vec(&self).unwrap()).unwrap();
    }
    pub fn curr(&self) -> String {
        self.temp[self.curr].clone()
    }
    pub fn choice(&mut self, idx: usize) {
        if idx < self.temp.len() {
            self.curr = idx;
        }
    }
    pub fn remove(&mut self, permanent: bool) {
        if permanent {
            if fs::remove_file(&self.temp[self.curr]).is_ok() {
                self.temp.remove(self.curr);
                self.inc();
            }
        } else {
            self.temp.remove(self.curr);
            self.inc();
        }
    }
    pub fn inc(&mut self) {
        if !self.temp.is_empty() {
            match self.curr < self.temp.len() - 1 {
                true => self.curr += 1,
                false => self.curr = 0,
            };
            self.choice(self.curr);
        }
    }
    pub fn dec(&mut self) {
        if !self.temp.is_empty() {
            match self.curr > 0 {
                true => self.curr -= 1,
                false => self.curr = self.temp.len() - 1,
            };
            self.choice(self.curr);
        }
    }
}

fn file() -> String {
    env::var("HOME").unwrap() + "/.config/" + crate::NAME
}
