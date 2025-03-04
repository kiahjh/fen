// Created by Fen v0.5.0 at 16:59:07 on 2025-03-04
// Do not manually modify this file as it is automatically generated

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Output = Person;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub name: String,
    pub birthday: DateTime<Utc>,
    pub id: Uuid,
    pub car: Car,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub color: String,
    pub gear: Gear,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Gear {
    Park,
    Neutral,
    Reverse,
    Drive(Speed),
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Speed {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}