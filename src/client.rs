use tonic::transport::{Channel, Endpoint};

use crate::protobuf::rate_limiter::{
    rate_limiter_client::RateLimiterClient as RateLimiterClientGrpc,
    AcquireRequest,
    AcquireResponse,
};


pub struct RateLimiterClient {
    client: RateLimiterClientGrpc<Channel>
}


impl RateLimiterClient {
    pub fn try_new(host: &str, port: u16) -> Result<Self, String> {
        Ok(
            Self {
                client: RateLimiterClientGrpc::new(
                    Endpoint::from_shared(
                        format!("http://{host}:{port}")
                    )
                        .map_err(
                            |e| format!("Error creating tonic Endpoint: {e}")
                        )?
                        .connect_lazy()
                )
            }
        )
    }


    pub async fn acquire(&self, amount: usize) -> Result<(), String> {
        match self.client
            .clone()
            .acquire(
                AcquireRequest {
                    amount: amount as u64
                }
            )
            .await
            .map(
                |response| response.into_inner()
            )
        {
            Ok(
                AcquireResponse { success: true, .. }
            ) => Ok(()),
            Ok(
                AcquireResponse { success: false, error_message }
            ) => Err(
                error_message
                    .unwrap_or(
                        "No error message provided".to_string()
                    )
            ),
            Err(e) => Err(e.to_string()),
        }
    }
}
