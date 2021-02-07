#![forbid(unsafe_code)]
use std::{env, time::Instant};
use prometheus::{opts, Encoder, HistogramVec, IntCounterVec, Registry, TextEncoder};
use prometheus;
use warp::{Filter,reject::Rejection};

#[derive(Clone,Debug)]
pub struct PrometheusMetrics {
    http_requests_total: IntCounterVec,
    http_requests_1xx_total: IntCounterVec,
    http_requests_2xx_total: IntCounterVec,
    http_requests_3xx_total: IntCounterVec,
    http_requests_4xx_total: IntCounterVec,
    http_requests_5xx_total: IntCounterVec,
    http_requests_duration_seconds: HistogramVec,
    registry: Registry,
}

impl Default for PrometheusMetrics {
    fn default() -> Self {
        Self::new()
    }
}


impl PrometheusMetrics {
    pub fn new(registry: Registry) -> Self{
        let namespace = "";
        let http_requests_total_opts =opts!("http_requests_total", "Total number of HTTP requests").namespace(namespace.clone());
        let http_requests_total =
        IntCounterVec::new(http_requests_total_opts, &["endpoint", "method", "status"]).unwrap();
        registry .register(Box::new(http_requests_total.clone())).unwrap();
        let http_requests_duration_seconds_opts = opts!("http_requests_duration_seconds","HTTP request duration in seconds for all requests").namespace(namespace);
        let http_requests_duration_seconds = HistogramVec::new(http_requests_duration_seconds_opts.into(),&["endpoint", "method", "status"]).unwrap();
        PrometheusMetrics {
            registry:registry,
            http_requests_total: http_requests_total,
            http_requests_duration_seconds: http_requests_duration_seconds, 
        }
    }
}

fn metrics() -> PrometheusMetrics {
    PrometheusMetrics {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
