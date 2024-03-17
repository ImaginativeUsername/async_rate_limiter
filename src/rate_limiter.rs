use std::{
    sync::Arc,
    time::Duration,
};

use tokio::sync::Semaphore;


pub struct AsyncRateLimiter {
    limit: usize,
    time_period: Duration,
    semaphore: Arc<Semaphore>,
}


impl AsyncRateLimiter {
    pub fn new(
        limit: usize,
        time_period: Duration,
    ) -> Self {
        Self {
            limit,
            time_period,
            semaphore: Arc::new(
                Semaphore::new(limit)
            ),
        }
    }


    pub async fn acquire(&self, amount: usize) -> Result<(), String> {
        if amount > self.limit {
            Err(
                "Requested amount greater than total limit".to_string()
            )
        } else{
            let permit = self.semaphore
                .clone()
                .acquire_many_owned(amount as u32)
                .await
                .map_err(
                    |e| format!("Error acquiring semaphore permit: {e}")
                )?;

            // Create release task
            let sleep_duration = self.time_period;
            tokio::task::spawn(
                async move {
                    tokio::time::sleep(sleep_duration).await;
                    drop(permit);
                }
            );

            Ok(())
        }
    }
}
