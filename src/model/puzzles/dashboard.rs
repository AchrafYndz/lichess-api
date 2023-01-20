use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Default, Clone, Debug, Serialize)]
pub struct Query;

pub type GetRequest = crate::model::Request<Query>;

impl GetRequest {
    pub fn new(days: u32) -> Self {
        Self {
            method: http::Method::GET,
            path: format!("/api/puzzle/dashboard/{}", days),
            query: Default::default(),
            body: Default::default()
        }
    }
}

pub type Dashboard = PuzzleDashboardJson;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleDashboardJson {
    days: i64,
    global: Results,
    themes: HashMap<String, Theme>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Theme {
    results: Results,
    theme: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Results {
    #[serde(rename = "firstWins")]
    first_wins: i64,
    nb: i64,
    performance: i64,
    #[serde(rename = "puzzleRatingAvg")]
    puzzle_rating_avg: i64,
    #[serde(rename = "replayWins")]
    replay_wins: i64,
}
