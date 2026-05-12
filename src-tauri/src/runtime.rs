use std::{env, path::PathBuf, time::Instant};

use serde_json::json;

use crate::observability::{
    metric_event, record_metric, MetricUnit, TraceId, BRIDGE_SERVICE_BOOT_MS_METRIC,
    WORKBENCH_READY_MS_METRIC,
};

use crate::workbench_url_resolver::{
    resolve_workbench_url_with, ResolvedWorkbenchUrl, WORKBENCH_URL_ENV,
};

pub fn run() {
    crate::observability::init_tracing();
    let boot_started = Instant::now();

    let _cli_protocol_version = crate::cli_adapter::protocol_version();
    let services_started = Instant::now();
    let mut service_registry = crate::service_registry::ServiceRegistry::new();
    crate::auth_tunnels_adapter::register_auth_tunnel_services(
        &mut service_registry,
        crate::auth_tunnels_adapter::AuthTunnelStateConfig::default(),
    );
    crate::terminal_service::register_terminal_services(&mut service_registry);
    crate::file_service::register_file_watcher_services(&mut service_registry);
    crate::storage_config_service::register_storage_config_services(
        &mut service_registry,
        crate::storage_config_service::UserDataPaths::default_from_environment(),
    );
    crate::parity_services::register_platform_parity_services(&mut service_registry);
    service_registry.register_extension_host_service(std::sync::Arc::new(
        crate::extension_sidecar::ExtensionSidecarServiceImpl::default(),
    ));
    record_metric(&metric_event(
        TraceId::new("bridge-service-boot"),
        BRIDGE_SERVICE_BOOT_MS_METRIC,
        services_started.elapsed().as_secs_f64() * 1000.0,
        MetricUnit::Milliseconds,
        Some(json!({ "phase": "serviceRegistryReady" })),
    ));

    // Workbench loading is intentionally minimal for T06:
    // - CODE_TAURI_WORKBENCH_URL may point at a dev server or hosted workbench.
    // - CODE_TAURI_WORKBENCH_PATH may point at a generated workbench.html.
    // - bundled out/vs/code/browser/workbench/workbench.html is preferred for packaged builds.
    // - out/vs/code/browser/workbench/workbench.html is auto-detected for source builds.
    // - CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 rejects the developer-only scaffold for release validation.
    // - src-tauri/www/index.html remains the developer-only fallback scaffold.
    crate::protocols::register_custom_protocols(tauri::Builder::default())
        .manage(service_registry)
        .manage(crate::cancellation_manager::CancellationManager::default())
        .manage(crate::subscription_manager::SubscriptionManager::default())
        .invoke_handler(tauri::generate_handler![
            crate::commands::channel_call,
            crate::commands::channel_listen,
            crate::commands::channel_dispose,
            crate::commands::cancel_request,
            crate::commands::fs_stat,
            crate::commands::fs_read_file,
            crate::commands::fs_write_file,
            crate::commands::fs_delete,
            crate::commands::fs_mkdir,
            crate::commands::fs_readdir,
            crate::commands::fs_watch
        ])
        .setup(|app| {
            let workbench_started = Instant::now();
            tauri::WebviewWindowBuilder::new(app, "main", resolve_workbench_url())
                .title("VS Code Atomic")
                .inner_size(1200.0, 800.0)
                .build()?;
            record_metric(&metric_event(
                TraceId::new("workbench-boot"),
                WORKBENCH_READY_MS_METRIC,
                boot_started.elapsed().as_secs_f64() * 1000.0,
                MetricUnit::Milliseconds,
                Some(json!({
                    "phase": "webviewWindowBuilt",
                    "windowBuildMs": workbench_started.elapsed().as_secs_f64() * 1000.0,
                })),
            ));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run VS Code Atomic Tauri runtime");
}

fn resolve_workbench_url() -> tauri::WebviewUrl {
    match resolve_workbench_url_with(env_value, |path| path.exists(), &repo_root()) {
        ResolvedWorkbenchUrl::External(url) => tauri::WebviewUrl::External(
            url.parse()
                .unwrap_or_else(|error| panic!("invalid {WORKBENCH_URL_ENV}: {error}")),
        ),
        ResolvedWorkbenchUrl::File(path) => tauri::WebviewUrl::External(file_url(path)),
        ResolvedWorkbenchUrl::App(path) => tauri::WebviewUrl::App(path.into()),
    }
}

fn env_value(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn file_url(path: PathBuf) -> tauri::Url {
    let path = path.canonicalize().unwrap_or_else(|error| {
        panic!(
            "failed to resolve workbench path {}: {error}",
            path.display()
        )
    });
    tauri::Url::from_file_path(&path).unwrap_or_else(|_| {
        panic!(
            "failed to convert workbench path {} to file URL",
            path.display()
        )
    })
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri has repository parent")
        .to_path_buf()
}
