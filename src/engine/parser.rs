use crate::engine::model;
use std::collections::HashMap;
use chrono::Weekday;
use chrono::{DateTime, Datelike};

pub fn most_active_day_commits(objects: Vec<model::Root>) -> (Weekday, u32) {
  let hash = group_by_commits(objects);
  let crap = hash.iter().max_by_key(|&(_x, y)| y);
  let key = crap.unwrap().0;
  let result = hash.get(key).unwrap();
  return (*key, *result);
}

pub fn group_by_commits(objects: Vec<model::Root>) -> HashMap<Weekday, u32> {
  let mut week_hash: HashMap<Weekday, u32> = HashMap::new();
  for x in objects {
    let weekday = parse_weekday(x.commit.author.date);
    let result = match week_hash.get(&weekday) {
      None => 1,
      Some(value) => value + 1,
    };
    week_hash.insert(weekday, result);
  }
  return week_hash;
}

pub fn parse_weekday(date_string: String) -> Weekday {
  let date_time = DateTime::parse_from_rfc3339(&date_string).unwrap();
  return date_time.weekday();
}
