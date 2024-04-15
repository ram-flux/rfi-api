//
//  Copyright 2024 Ram Flux, LLC.
//



#[derive(serde::Serialize, serde::Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Response<T> {
    #[serde(skip)]
    status: u16,
    code: u32,
    pub(crate) message: String,
    result: Option<T>,
}

// pub enum ResponseData {}

impl<T> From<Result<T, crate::Error>> for Response<T>
where
    T: serde::Serialize + Sized,
{
    fn from(res: Result<T, crate::Error>) -> Self {
        match res {
            Ok(ok) => ok.into(),
            Err(err) => {
                let (status, code, typ, message) = err.into();
                let message = format!("{}:{}", typ, message);
                Response {
                    status,
                    code,
                    message,
                    result: None,
                }
            }
        }
    }
}

// impl<T> From<Result<T, crate::service::user::api::error::Error>> for Response<T>
// where
//     T: serde::Serialize,
// {
//     fn from(res: Result<T, crate::service::user::api::error::Error>) -> Response<T> {
//         match res {
//             Ok(ok) => ok.into(),
//             Err(err) => {
//                 let (status, code, message) = err.into();
//                 Response {
//                     status,
//                     code,
//                     message,
//                     result: None,
//                 }
//             }
//         }
//     }
// }



impl<T> From<T> for Response<T>
where
    T: serde::Serialize + Sized,
{
    fn from(msg: T) -> Self {
        Self {
            status: 200,
            code: 200,
            message: "success".to_string(),
            result: Some(msg),
        }
    }
}



impl From<crate::Error> for (u16, u32, String, String) {
    fn from(err: crate::Error) -> Self {
        use crate::Error;
        let status = err.get_status_code().as_u16();
        let (code, typ, message) = match err {
            Error::Msg(_) => (203, "error".to_string(), err.to_string()),
        };
        (status, code, typ, message)
    }
}

// impl From<crate::JwtError> for (u16, u32, String, String) {
//     fn from(err: crate::JwtError) -> Self {
//         use crate::JwtError;
//         let (code, typ, message) = match err {
//             JwtError::IllegalAccess(_) => (
//                 err.get_status_code(),
//                 "illegal access".to_string(),
//                 err.to_string(),
//             ),
//             JwtError::TokenExpires => (
//                 err.get_status_code(),
//                 "token expires".to_string(),
//                 err.to_string(),
//             ),
//         };
//         (401, code, typ, message)
//     }
// }

impl<T> axum::response::IntoResponse for Response<T>
where
    T: serde::Serialize + Sized,
{
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::from_u16(self.status)
                .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
            axum::Json(self),
        )
            .into_response()
    }
}
