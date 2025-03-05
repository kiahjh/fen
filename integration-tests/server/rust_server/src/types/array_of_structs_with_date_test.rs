// Created by Fen v0.5.2 at 13:10:09 on 2025-03-05
// Do not manually modify this file as it is automatically generated

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type Output = Vec<Song>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub title: String,
    pub composed: DateTime<Utc>,
}