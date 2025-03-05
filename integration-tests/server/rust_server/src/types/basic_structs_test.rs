// Created by Fen v0.5.2 at 13:10:09 on 2025-03-05
// Do not manually modify this file as it is automatically generated

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub name: String,
    pub age: isize,
    pub birthday: DateTime<Utc>,
    pub has_beard: bool,
}