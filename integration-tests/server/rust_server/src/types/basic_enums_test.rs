// Created by Fen v0.5.0 at 16:59:07 on 2025-03-04
// Do not manually modify this file as it is automatically generated

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Input {
    Happy,
    Sad,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Output {
    Foo,
    Bar,
    Baz,
}