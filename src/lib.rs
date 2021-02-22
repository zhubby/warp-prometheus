#![forbid(unsafe_code)]
use std::{env, time::Instant};
use prometheus::{opts, Encoder, HistogramVec, IntCounterVec, Registry, TextEncoder};
use prometheus;
use warp::{Filter,reject::Rejection};

#[derive(Clone,Debug)]
pub struct PrometheusMetrics {
    http_requests_total: IntCounterVec,
    // http_requests_1xx_total: IntCounterVec,
    // http_requests_2xx_total: IntCounterVec,
    // http_requests_3xx_total: IntCounterVec,
    // http_requests_4xx_total: IntCounterVec,
    // http_requests_5xx_total: IntCounterVec,
    http_requests_duration_seconds: HistogramVec,
    registry: Registry,
}

impl Default for PrometheusMetrics {
    fn default() -> Self {
        Self::new()
    }
}

async fn metrics() {

}

impl PrometheusMetrics {
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn new() -> Self{
        let label = ["endpoint", "method", "status"];
        let namespace = "";
        let registry = Registry::new();
        let http_requests_total_opts =opts!("http_requests_total", "Total number of HTTP requests").namespace(namespace.clone());
        let http_requests_total =
        IntCounterVec::new(http_requests_total_opts, &label).unwrap();
        registry .register(Box::new(http_requests_total.clone())).unwrap();
        let http_requests_duration_seconds_opts = opts!("http_requests_duration_seconds","HTTP request duration in seconds for all requests").namespace(namespace);
        let http_requests_duration_seconds = HistogramVec::new(http_requests_duration_seconds_opts.into(),&label).unwrap();
        PrometheusMetrics {
            registry:registry,
            http_requests_total: http_requests_total,
            http_requests_duration_seconds: http_requests_duration_seconds, 
        }
    }

    pub fn text(&self)-> String {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        encoder.encode(&self.registry.gather(), &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}


// pub fn metrics() -> impl Filter<Extract = PrometheusMetrics, Error = Rejection> + Copy {
//     warp::any().and(warp::any().map(move ||
//         {
//             let p = PrometheusMetrics::new();
//             p.clone()
//         }

//     )).and_then(|pm : PrometheusMetrics| async move {
//         Ok::<_, Rejection>(PrometheusMetrics::new())
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    #[tokio::test]
    async fn test_server() {
        let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
    }
}
