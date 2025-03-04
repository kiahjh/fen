// Created by Fen v0.5.0 at 16:59:07 on 2025-03-04
// Do not manually modify this file as it is automatically generated

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type Output = Human;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Human {
    pub name: String,
    pub birthday: DateTime<Utc>,
    pub vehicle: Vehicle,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub color: String,
    pub year: isize,
}