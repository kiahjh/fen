use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Response<T> {
    Success(SuccessResponse<T>),
    Failure(FailureResponse),
}

impl<T> Response<T> {
    pub const fn success(data: T) -> Self {
        Self::Success(SuccessResponse { data })
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
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct FailureResponse {
    pub message: String,
    pub status: isize,
}

pub fn fen_path(path: &str) -> String {
    format!("/_fen_{path}")
}
