use {
    fltk::image::SharedImage,
    std::{collections::HashMap, fs},
};

#[derive(Debug, Clone)]
pub struct Model {
    pub cash: HashMap<String, SharedImage>,
    pub temp: Vec<String>,
    pub curr: usize,
    pub size: i32,
}
impl Model {
    pub fn new() -> Self {
        Self {
            cash: HashMap::new(),
            temp: Vec::new(),
            size: 100,
            curr: 0,
        }
    }
    pub fn empty(&self) -> bool {
        self.temp.is_empty()
    }
    pub fn choice(&mut self, curr: usize) {
        if self.cash.contains_key(&self.temp[curr]) {
            self.curr = curr;
        } else if let Ok(image) = SharedImage::load(&self.temp[curr]) {
            self.cash.insert(self.temp[curr].clone(), image.clone());
            self.curr = curr;
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
