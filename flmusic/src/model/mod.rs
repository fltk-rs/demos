use {
    serde::{Deserialize, Serialize},
    std::{env, fs},
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Model {
    pub list: Vec<String>,
    pub curr: usize,
    pub duration: f64,
    pub time: f64,
}

impl Model {
    pub fn default() -> Self {
        if let Ok(value) = fs::read(file()) {
            if let Ok(value) = rmp_serde::from_slice::<Self>(&value) {
                return value;
            }
        }
        Self {
            curr: 0,
            list: Vec::new(),
            duration: 0.0,
            time: 0.0,
        }
    }
    pub fn choice(&mut self, idx: usize) {
        if idx < self.list.len() {
            self.curr = idx;
        }
    }
    pub fn save(&self) {
        fs::write(file(), rmp_serde::to_vec(&self).unwrap()).unwrap();
    }
    pub fn inc(&mut self) {
        if !self.list.is_empty() {
            match self.curr < self.list.len() - 1 {
                true => self.curr += 1,
                false => self.curr = 0,
            };
            self.choice(self.curr);
        }
    }
    pub fn dec(&mut self) {
        if !self.list.is_empty() {
            match self.curr > 0 {
                true => self.curr -= 1,
                false => self.curr = self.list.len() - 1,
            };
            self.choice(self.curr);
        }
    }
    pub fn remove(&mut self, permanent: bool) {
        if permanent {
            if fs::remove_file(&self.list[self.curr]).is_ok() {
                self.list.remove(self.curr);
                self.inc();
            }
        } else {
            self.list.remove(self.curr);
            self.inc();
        }
    }
}

fn file() -> String {
    env::var("HOME").unwrap() + "/.config/" + crate::NAME
}
