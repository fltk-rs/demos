use {
    serde::{Deserialize, Serialize},
    std::{env, fs},
};

#[derive(Deserialize, Serialize, Clone)]
pub struct Model {
    pub prev: f64,
    pub operation: String,
    pub current: String,
    pub output: String,
    pub theme: bool,
}

impl Model {
    pub fn default() -> Self {
        let default = Self {
            prev: 0f64,
            operation: String::new(),
            current: String::from("0"),
            output: String::new(),
            theme: false,
        };
        if let Ok(value) = fs::read(file()) {
            if let Ok(value) = rmp_serde::from_slice(&value) {
                value
            } else {
                default
            }
        } else {
            default
        }
    }
    pub fn save(&mut self) {
        fs::write(file(), rmp_serde::to_vec(&self).unwrap()).unwrap();
    }
    pub fn theme(&mut self) {
        self.theme = !self.theme;
    }
    pub fn click(&mut self, value: &str) {
        match value {
            "/" | "x" | "+" | "-" | "%" => {
                if self.current != "0" {
                    if self.operation.is_empty() {
                        self.prev = self.current.parse().unwrap();
                    } else {
                        self.equil();
                    }
                    self.output.push_str(&format!("{} {}", self.prev, value));
                    self.operation = value.to_string();
                    self.current = String::from("0");
                }
            }
            "=" => {
                if !self.operation.is_empty() {
                    self.equil();
                    self.operation.clear();
                }
            }
            "CE" => {
                self.output.clear();
                self.operation.clear();
                self.current = String::from("0");
                self.prev = 0f64;
            }
            "@<-" => {
                let label = self.current.clone();
                self.current = if label.len() > 1 {
                    String::from(&label[..label.len() - 1])
                } else {
                    String::from("0")
                };
            }
            "C" => self.current = String::from("0"),
            "." => {
                if !self.current.contains('.') {
                    self.current.push('.');
                }
            }
            _ => {
                if self.current == "0" {
                    self.current.clear();
                }
                self.current = self.current.clone() + value;
            }
        };
    }
    fn equil(&mut self) {
        self.output.push_str(&format!(" {}\n", self.current));
        let current: f64 = self.current.parse().unwrap();
        self.prev = match self.operation.as_str() {
            "/" => self.prev / current,
            "x" => self.prev * current,
            "+" => self.prev + current,
            "-" => self.prev - current,
            _ => self.prev / 100.0 * current,
        };
        self.output.push_str(&format!("    = {}\n", self.prev));
        self.current = String::from("0");
    }
}

fn file() -> String {
    env::var("HOME").unwrap() + "/.config/" + crate::NAME
}
