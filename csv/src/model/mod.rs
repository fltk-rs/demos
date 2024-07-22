use {
    csv::Reader,
    serde::Deserialize,
    std::{collections::HashMap, fs},
};

#[derive(Debug, Deserialize, Clone)]
pub struct Price {
    #[serde(rename = "Date")]
    _date: String,
    #[serde(rename = "Open")]
    pub open: f64,
    #[serde(rename = "High")]
    pub high: f64,
    #[serde(rename = "Low")]
    pub low: f64,
    #[serde(rename = "Close")]
    pub close: f64,
    #[serde(rename = "Volume")]
    _volume: usize,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub cash: HashMap<String, Vec<Price>>,
    pub temp: Vec<String>,
    pub curr: usize,
}

impl Model {
    pub fn default() -> Self {
        let mut default = Self {
            cash: HashMap::new(),
            temp: Vec::new(),
            curr: 0,
        };
        default.init();
        default
    }
    pub fn init(&mut self) {
        for file in std::fs::read_dir("assets/historical_data").unwrap() {
            let entry = file.unwrap().file_name().into_string().unwrap();
            if entry.ends_with(".csv") {
                self.temp
                    .push(entry.strip_suffix(".csv").unwrap().to_string());
            }
            self.choice(0);
        }
    }
    pub fn choice(&mut self, curr: usize) {
        if self.cash.contains_key(&self.temp[curr]) {
            self.curr = curr;
        } else if let Ok(data) = fs::read(format!("assets/historical_data/{}.csv", self.temp[curr]))
        {
            let mut prices: Vec<Price> = Vec::new();
            for result in Reader::from_reader(data.as_slice()).deserialize() {
                prices.push(result.unwrap());
            }
            self.cash.insert(self.temp[curr].clone(), prices);
            self.curr = curr;
        }
    }
}
