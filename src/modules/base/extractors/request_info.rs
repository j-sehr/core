use axum::extract::{ConnectInfo, FromRequestParts};

#[derive(Debug, Clone)]
pub struct RequestInfoExtractor {
    pub ip_address: String,
    pub user_agent: String,
}

impl FromRequestParts<()> for RequestInfoExtractor {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &(),
    ) -> Result<Self, Self::Rejection> {
        let ip = ConnectInfo::<std::net::SocketAddr>::from_request_parts(parts, _state).await;
        if ip.is_err() {
            return Err(());
        }

        let user_agent = parts
            .headers
            .get("user-agent")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        Ok(RequestInfoExtractor {
            ip_address: ip.unwrap().ip().to_string(),
            user_agent,
        })
    }
}
