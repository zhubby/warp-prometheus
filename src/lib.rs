#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::{env, time::Instant};

use prometheus::{opts, Encoder, HistogramVec, IntCounterVec, Registry, TextEncoder};

use prometheus;

#[derive(Clone)]
#[must_use = "must be attached and mounted to a Warp instance"]
pub struct PrometheusMetrics {
    http_requests_total: IntCounterVec,
    http_requests_duration_seconds: HistogramVec,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
