#![cfg_attr(not(feature = "runtime"), allow(dead_code))]

#[cfg(feature = "runtime")]
mod auth_tunnels_adapter;
#[cfg(feature = "runtime")]
mod cli_adapter;
#[cfg(feature = "runtime")]
mod commands;
mod contracts;
mod extension_sidecar;
mod file_service;
mod observability;
mod parity_services;
#[cfg(feature = "runtime")]
mod protocols;
#[cfg(feature = "runtime")]
mod runtime;
mod service_registry;
mod storage_config_service;
mod subscription_manager;
mod terminal_service;
#[cfg(feature = "runtime")]
mod workbench_url_resolver;

#[cfg(feature = "runtime")]
fn main() {
    runtime::run();
}

#[cfg(not(feature = "runtime"))]
fn main() {}
