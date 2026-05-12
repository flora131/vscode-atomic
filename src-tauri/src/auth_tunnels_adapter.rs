use std::{
    env,
    path::{Path, PathBuf},
    sync::Arc,
};

use async_trait::async_trait;
use serde_json::{json, Value};

use crate::{
    contracts::ChannelCallRequest,
    service_registry::{
        AuthService, PlatformService, ServiceError, ServiceId, ServiceRegistry, TunnelService,
    },
};

#[derive(Debug, Clone)]
pub struct AuthTunnelStateConfig {
    root: PathBuf,
}

impl AuthTunnelStateConfig {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn token_file(&self) -> PathBuf {
        self.root.join("token.json")
    }
}

impl Default for AuthTunnelStateConfig {
    fn default() -> Self {
        Self::new(default_cli_state_root())
    }
}

pub fn default_cli_state_root() -> PathBuf {
    default_cli_state_root_for_home(env::var("HOME").ok().filter(|value| !value.is_empty()))
}

fn default_cli_state_root_for_home(home: Option<String>) -> PathBuf {
    home.map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("~"))
        .join(cli::constants::DEFAULT_DATA_PARENT_DIR)
        .join("cli")
}

pub fn register_auth_tunnel_services(
    registry: &mut ServiceRegistry,
    config: AuthTunnelStateConfig,
) {
    registry.register_auth_service(Arc::new(CliAuthStateService::new(config.clone())));
    registry.register_tunnel_service(Arc::new(CliTunnelStateService::new(config)));
}

pub fn registry_with_auth_tunnel_services(config: AuthTunnelStateConfig) -> ServiceRegistry {
    let mut registry = ServiceRegistry::new();
    register_auth_tunnel_services(&mut registry, config);
    registry
}

#[derive(Debug, Clone)]
pub struct CliAuthStateService {
    config: AuthTunnelStateConfig,
}

impl CliAuthStateService {
    pub fn new(config: AuthTunnelStateConfig) -> Self {
        Self { config }
    }

    fn defaults(&self) -> Value {
        json!({
            "applicationName": cli::constants::APPLICATION_NAME,
            "productNameLong": cli::constants::PRODUCT_NAME_LONG,
            "quality": cli::constants::QUALITY,
            "userAgent": cli::constants::get_default_user_agent(),
            "providers": [
                provider_defaults(cli::auth::AuthProvider::Microsoft),
                provider_defaults(cli::auth::AuthProvider::Github),
            ],
        })
    }

    fn state_paths(&self) -> Value {
        json!({
            "root": path_string(self.config.root()),
            "tokenFile": path_string(self.config.token_file()),
            "namespacedTokenPattern": "token-{namespace}.json",
            "keyringPrefix": "vscode-cli",
            "namespacedKeyringPrefixPattern": "vscode-cli-{namespace}",
        })
    }
}

#[async_trait]
impl PlatformService for CliAuthStateService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Auth
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        match request.command.as_str() {
            "defaults" | "getDefaults" => Ok(self.defaults()),
            "statePaths" | "getStatePaths" => Ok(self.state_paths()),
            "current" | "getCurrent" => Ok(json!({
                "authenticated": false,
                "provider": null,
                "credential": null,
                "reason": "auth adapter wiring does not read keychain state",
            })),
            command => Err(ServiceError::unsupported(ServiceId::Auth, command)),
        }
    }
}

impl AuthService for CliAuthStateService {}

#[derive(Debug, Clone)]
pub struct CliTunnelStateService {
    config: AuthTunnelStateConfig,
}

impl CliTunnelStateService {
    pub fn new(config: AuthTunnelStateConfig) -> Self {
        Self { config }
    }

    fn defaults(&self) -> Value {
        json!({
            "protocolVersion": cli::constants::PROTOCOL_VERSION,
            "protocolVersionTag": cli::constants::PROTOCOL_VERSION_TAG,
            "controlPort": cli::constants::CONTROL_PORT,
            "agentHostPort": cli::constants::AGENT_HOST_PORT,
            "activityName": cli::constants::TUNNEL_ACTIVITY_NAME,
            "serviceLogFileName": cli::tunnels::SERVICE_LOG_FILE_NAME,
            "serverFolderName": cli::tunnels::paths::SERVER_FOLDER_NAME,
            "serviceUserAgent": cli::constants::TUNNEL_SERVICE_USER_AGENT.as_str(),
            "methods": {
                "restart": cli::tunnels::protocol::singleton::METHOD_RESTART,
                "shutdown": cli::tunnels::protocol::singleton::METHOD_SHUTDOWN,
                "status": cli::tunnels::protocol::singleton::METHOD_STATUS,
                "log": cli::tunnels::protocol::singleton::METHOD_LOG,
                "logReplyDone": cli::tunnels::protocol::singleton::METHOD_LOG_REPLY_DONE,
            },
        })
    }

    fn state_paths(&self) -> Value {
        let paths = TunnelStatePaths::new(self.config.root().clone());
        json!({
            "root": path_string(paths.root()),
            "tunnelLockfile": path_string(paths.tunnel_lockfile()),
            "forwardingLockfile": path_string(paths.forwarding_lockfile()),
            "agentHostLockfile": path_string(paths.agent_host_lockfile()),
            "serviceLogFile": path_string(paths.service_log_file()),
            "webServerStorage": path_string(paths.web_server_storage()),
        })
    }
}

#[derive(Debug, Clone)]
struct TunnelStatePaths {
    root: PathBuf,
}

impl TunnelStatePaths {
    fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn tunnel_lockfile(&self) -> PathBuf {
        self.root.join(format!(
            "tunnel-{}.lock",
            cli::constants::VSCODE_CLI_QUALITY.unwrap_or("oss")
        ))
    }

    fn forwarding_lockfile(&self) -> PathBuf {
        self.root.join(format!(
            "forwarding-{}.lock",
            cli::constants::VSCODE_CLI_QUALITY.unwrap_or("oss")
        ))
    }

    fn agent_host_lockfile(&self) -> PathBuf {
        self.root.join(format!(
            "agent-host-{}.lock",
            cli::constants::VSCODE_CLI_QUALITY.unwrap_or("oss")
        ))
    }

    fn service_log_file(&self) -> PathBuf {
        self.root.join(cli::tunnels::SERVICE_LOG_FILE_NAME)
    }

    fn web_server_storage(&self) -> PathBuf {
        self.root.join("serve-web")
    }
}

#[async_trait]
impl PlatformService for CliTunnelStateService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Tunnel
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        match request.command.as_str() {
            "defaults" | "getDefaults" => Ok(self.defaults()),
            "statePaths" | "getStatePaths" => Ok(self.state_paths()),
            "status" | "getStatus" => serde_json::to_value(
                cli::tunnels::protocol::singleton::StatusWithTunnelName::default(),
            )
            .map_err(|error| ServiceError {
                code: "service.serializationFailed",
                message: error.to_string(),
            }),
            command => Err(ServiceError::unsupported(ServiceId::Tunnel, command)),
        }
    }
}

impl TunnelService for CliTunnelStateService {}

fn provider_defaults(provider: cli::auth::AuthProvider) -> Value {
    json!({
        "id": match provider {
            cli::auth::AuthProvider::Microsoft => "microsoft",
            cli::auth::AuthProvider::Github => "github",
        },
        "label": provider.to_string(),
        "clientId": provider.client_id(),
        "codeUri": provider.code_uri(),
        "grantUri": provider.grant_uri(),
        "defaultScopes": provider.get_default_scopes(),
    })
}

fn path_string(path: impl AsRef<std::path::Path>) -> String {
    path.as_ref().to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll, Wake, Waker},
    };

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(future);

        loop {
            match Pin::new(&mut future).poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    fn test_config() -> AuthTunnelStateConfig {
        AuthTunnelStateConfig::new(std::env::temp_dir().join(format!(
            "vscode-atomic-auth-tunnels-test-{}",
            std::process::id()
        )))
    }

    #[test]
    fn default_state_root_reuses_cli_data_parent_dir() {
        let root = default_cli_state_root_for_home(Some("/home/tester".to_string()));
        assert_eq!(
            root,
            PathBuf::from("/home/tester")
                .join(cli::constants::DEFAULT_DATA_PARENT_DIR)
                .join("cli")
        );
        assert!(!path_string(root).starts_with("~/"));
    }

    #[test]
    fn registers_auth_and_tunnel_services() {
        let registry = registry_with_auth_tunnel_services(test_config());

        assert_eq!(
            registry.require(ServiceId::Auth).unwrap().service_id(),
            ServiceId::Auth
        );
        assert_eq!(
            registry.require(ServiceId::Tunnel).unwrap().service_id(),
            ServiceId::Tunnel
        );
    }

    #[test]
    fn auth_adapter_exposes_cli_defaults_without_network() {
        let service = CliAuthStateService::new(test_config());
        let result = block_on(service.call(ChannelCallRequest {
            request_id: "req-auth".to_string(),
            channel: ServiceId::AUTH.to_string(),
            command: "defaults".to_string(),
            args: Vec::new(),
            cancellation_id: None,
        }))
        .unwrap();

        assert_eq!(result["applicationName"], cli::constants::APPLICATION_NAME);
        assert_eq!(result["providers"][0]["id"], "microsoft");
        assert_eq!(
            result["providers"][1]["clientId"],
            cli::auth::AuthProvider::Github.client_id()
        );
    }

    #[test]
    fn tunnel_adapter_exposes_state_paths_from_launcher_paths() {
        let config = test_config();
        let root = config.root().clone();
        let service = CliTunnelStateService::new(config);
        let result = block_on(service.call(ChannelCallRequest {
            request_id: "req-tunnel".to_string(),
            channel: ServiceId::TUNNEL.to_string(),
            command: "statePaths".to_string(),
            args: Vec::new(),
            cancellation_id: None,
        }))
        .unwrap();

        assert_eq!(result["root"], path_string(&root));
        assert_eq!(
            result["serviceLogFile"],
            path_string(root.join(cli::tunnels::SERVICE_LOG_FILE_NAME))
        );
        assert!(result["tunnelLockfile"]
            .as_str()
            .unwrap()
            .ends_with("tunnel-oss.lock"));
    }

    #[test]
    fn tunnel_state_paths_have_no_launcher_migration_side_effects() {
        let root = std::env::temp_dir().join(format!(
            "vscode-atomic-auth-tunnels-side-effects-{}",
            std::process::id()
        ));
        let legacy_server = root.join("server-stable");
        std::fs::create_dir_all(&legacy_server).expect("legacy server dir created");

        let service = CliTunnelStateService::new(AuthTunnelStateConfig::new(root.clone()));
        let _ = block_on(service.call(ChannelCallRequest {
            request_id: "req-tunnel-side-effects".to_string(),
            channel: ServiceId::TUNNEL.to_string(),
            command: "statePaths".to_string(),
            args: Vec::new(),
            cancellation_id: None,
        }))
        .unwrap();

        assert!(legacy_server.exists());
        let _ = std::fs::remove_dir_all(root);
    }
}
