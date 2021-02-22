use tokio;
use warp::Filter;
use warp_prometheus::PrometheusMetrics;
#[tokio::main]
async fn main() {
    let p = PrometheusMetrics::new();
    print!("{}",p.text());
    let metrics = warp::path("metrics").map(move || p.text() );
    warp::serve(metrics).run(([0,0,0,0], 3030)).await;
}