#[macro_export]
macro_rules! error_return {
        (let $fn_name:ident = $return_val:expr) => {
            let ret_val = $return_val;
            if let Err(e) = &ret_val {
                if e.is_client_error() {
                    tracing::debug!("Client error: {:?}", e);
                    return (
                        ::axum::http::StatusCode::BAD_REQUEST,
                        Json(json!({"error": format!("{}", e)})),
                    );
                }


                tracing::error!("Error: {:?}", e);
                return (
                    ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("{}", e)})),
                );
            }

            let $fn_name = ret_val.unwrap();

            };

        (let $fn_name:pat = $return_val:expr) => {
            let ret_val = $return_val;
            if let Err(e) = &ret_val {
                if e.is_client_error() {
                    tracing::debug!("Client error: {:?}", e);
                    return (
                        ::axum::http::StatusCode::BAD_REQUEST,
                        Json(json!({"error": format!("{}", e)})),
                    );
                }


                tracing::error!("Error: {:?}", e);
                return (
                    ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("{}", e)})),
                );
            }

            let $fn_name = ret_val.unwrap();

        };

        ($return_val:expr) => {
            let ret_val = $return_val;
            if let Err(e) = &ret_val {
                if e.is_client_error() {
                    tracing::debug!("Client error: {:?}", e);
                    return (
                        ::axum::http::StatusCode::BAD_REQUEST,
                        Json(json!({"error": format!("{}", e)})),
                    );
                }


                tracing::error!("Error: {:?}", e);
                return (
                    ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("{}", e)})),
                );
            }

        }

    }
