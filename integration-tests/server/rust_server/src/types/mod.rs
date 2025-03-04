// Created by Fen v0.5.0 at 16:59:07 on 2025-03-04
// Do not manually modify this file as it is automatically generated

pub mod dates_test;
pub mod strings_test;
pub mod arrays_test;
pub mod basic_enums_test;
pub mod bools_test;
pub mod structs_with_compound_types_test;
pub mod nested_structs_test;
pub mod enums_with_associated_values_test;
pub mod compound_arrays_and_optionals_test;
pub mod uuids_test;
pub mod optionals_test;
pub mod ints_test;
pub mod composing_structs_and_enums_test;
pub mod basic_structs_test;
pub mod floats_test;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Response<T> {
    Success(SuccessResponse<T>),
    Failure(FailureResponse),
}

impl<T> Response<T> {
    pub const fn success(value: T) -> Self {
        Self::Success(SuccessResponse { value })
    }

    pub fn failure(status: isize, message: &str) -> Self {
        Self::Failure(FailureResponse {
            status,
            message: message.to_string(),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    pub value: T,
}

#[derive(Serialize, Deserialize)]
pub struct FailureResponse {
    pub message: String,
    pub status: isize,
}

pub fn fen_path(path: &str) -> String {
    format!("/_fen_{path}")
}
