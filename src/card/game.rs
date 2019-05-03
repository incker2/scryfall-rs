use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Game {
    Paper,
    Arena,
    Mtgo,
}
