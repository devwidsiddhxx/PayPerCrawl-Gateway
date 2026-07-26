use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

pub fn setup_metrics() -> PrometheusHandle {
    PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to install Prometheus recorder")
}

pub fn record_request(is_bot: bool, blocked: bool) {
    if is_bot {
        if blocked {
            metrics::counter!("requests_blocked_total", "type" => "bot").increment(1);
        } else {
            metrics::counter!("requests_passed_total", "type" => "bot_paid").increment(1);
            metrics::counter!("payment_revenue_total").increment(1);
        }
    } else {
        metrics::counter!("requests_passed_total", "type" => "human").increment(1);
    }
}
