use chrono::{Duration, Utc};
use crate::engine::model;

#[derive(Debug)]
pub struct Stats {
    repo: String,
}

impl Stats {
    pub fn new(repo: String) -> Stats {
        Stats { repo: repo }
    }

    pub fn fetch(&self) -> Result<Vec<model::Root>, reqwest::Error> {
        let a_year_ago = Utc::now().checked_add_signed(Duration::days(-365)).unwrap();
        let url = format!("https://api.github.com/repos/{}/commits", self.repo);
        let http_client = reqwest::blocking::Client::new();

        let response = http_client
            .get(&url)
            .header("user-agent", "my little app")
            .query(&[("since", a_year_ago.to_rfc3339()), ("per_page", "100".to_string())])
            .send()?;
        return response.json::<Vec<model::Root>>();
    }
}
