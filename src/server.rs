use std::{
    error::Error,
    time::Duration,
};

use tonic::{
    transport::Server,
    Request,
    Response,
    Status,
};

use crate::{
    protobuf::rate_limiter::{
        rate_limiter_server::{
            RateLimiter,
            RateLimiterServer as RateLimiterServerGrpc,
        },
        AcquireRequest,
        AcquireResponse,
    },
    rate_limiter::AsyncRateLimiter,
};


pub struct RateLimiterServer {
    rate_limiter: AsyncRateLimiter,
}


#[tonic::async_trait]
impl RateLimiter for RateLimiterServer {
    async fn acquire(
        &self,
        request: Request<AcquireRequest>,
    ) -> Result<Response<AcquireResponse>, Status> {
        let response_message = match self.rate_limiter
            .acquire(
                request.into_inner().amount as usize
            )
            .await
        {
            Ok(()) => AcquireResponse {
                success: true,
                error_message: None,
            },
            Err(e) => AcquireResponse {
                success: false,
                error_message: Some(e),
            },
        };

        Ok(
            Response::new(response_message)
        )
    }
}


impl RateLimiterServer {
    pub fn new(
        limit: usize,
        time_period: Duration,
    ) -> Self {
        Self {
            rate_limiter: AsyncRateLimiter::new(limit, time_period),
        }
    }


    pub async fn serve(self, port: u16) -> Result<(), Box<dyn Error>> {
        Server::builder()
            .add_service(
                RateLimiterServerGrpc::new(self)
            )
            .serve(
                format!("0.0.0.0:{}", port).parse()?
            )
            .await?;

        Ok(())
    }
}