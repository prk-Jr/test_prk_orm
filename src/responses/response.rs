use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AppResponse<T, E> {
    Ok(OkResponse<T>),
    Err(ErrResponse<E>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OkResponse<T>(pub T);
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrResponse<E>(pub E);
