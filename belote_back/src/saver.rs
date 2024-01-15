use serde::{Deserialize, Serialize};

pub trait Savable {
    fn save_score(&mut self, player_name: &str, player_score: i32);
}

/// ## A struct to save the score in the local storage of the browser.
///
/// # Example
///
/// ```json
/// [{"username":"aa","score":10},{"username":"aa","score":30},{"username":"aa","score":32},{"username":"aa","score":-18},{"username":"aa","score":23},{"username":"etienno","score":21},{"username":"etienno","score":-12},{"username":"etienno","score":40},{"username":"totor","score":10},{"username":"totor","score":13},{"username":"totor","score":18}]
/// ```
#[derive(Default)]
pub struct WebLocalStorageSaver {}

#[derive(Serialize, Deserialize)]
pub struct Score {
    username: String,
    score: i32,
}

impl Savable for WebLocalStorageSaver {
    fn save_score(&mut self, player_name: &str, player_score: i32) {
        let window = web_sys::window().expect("no global `window` exists");
        let local_storage = window.local_storage().unwrap().unwrap();
        let scores = match local_storage.get_item("leaderboardData") {
            Ok(Some(scores)) => scores,
            Ok(None) => String::from("[]"),
            Err(_) => String::from("[]"),
        };
        let mut scores: Vec<Score> = serde_json::from_str(&scores).unwrap();
        scores.push(Score {
            username: player_name.to_string(),
            score: player_score,
        });
        let scores = serde_json::to_string(&scores).unwrap();
        local_storage
            .set_item("leaderboardData", &scores)
            .expect("set_item should work");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json() {
        let player_name = String::from("toto");
        let player_score = 10;
        let scores: Vec<Score> = vec![Score {
            username: player_name.to_string(),
            score: player_score,
        }];
        let scores = serde_json::to_string(&scores).unwrap();
        assert_eq!(scores, "[{\"username\":\"toto\",\"score\":10}]".to_string());
    }
}
