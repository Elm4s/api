use prometheus::*;

lazy_static::lazy_static! {
	pub static ref REGISTRY: Registry = Registry::new_custom(
		Some("rivet".to_string()),
		Some(labels! {
			"service".to_owned() => std::env::var("CHIRP_SERVICE_NAME").unwrap_or_default(),
			"worker_source_hash".to_owned() => std::env::var("RIVET_SOURCE_HASH").unwrap_or_default(),
			"worker_kind".to_owned() => std::env::var("CHIRP_WORKER_KIND").unwrap_or_else(|_| "service".into()),
			"kubernetes_pod_id".to_owned() => std::env::var("KUBERNETES_POD_ID").unwrap_or_default(),
		})).unwrap();
}
