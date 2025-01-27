use js_sys::{Date, Function, Promise};
use std::future::Future;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::common::TimeoutError;

#[wasm_bindgen]
extern "C" {

    type Performance;
    #[wasm_bindgen(js_name = "performance")]
    static PERFORMANCE: Option<Performance>;

    #[wasm_bindgen(method)]
    fn now(this: &Performance) -> f64;

    #[wasm_bindgen(js_name = setTimeout)]
    fn set_timeout(closure: &Function, millis: u32) -> f64;

}

pub struct ElapsedTimeCounter {
    start_time: f64,
}

impl ElapsedTimeCounter {
    pub fn start() -> Self {
        Self { start_time: now() }
    }

    pub fn elapsed_time(&self) -> Duration {
        Duration::from_millis((self.start_time - now()) as u64)
    }
}

pub async fn sleep(duration: Duration) {
    let _ = JsFuture::from(Promise::new(&mut |resolve, _reject| {
        set_timeout(&resolve, duration.as_millis() as u32);
    }))
    .await;
}

pub async fn timeout<F>(duration: Duration, future: F) -> Result<F::Output, TimeoutError>
where
    F: Future,
{
    tokio::select! {
        result = future => Ok(result),
        _ = sleep(duration) => Err(TimeoutError)
    }
}

fn now() -> f64 {
    PERFORMANCE.as_ref().map(|p| p.now()).unwrap_or_else(Date::now)
}
