use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
    sync::Arc,
};

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use tokio::{fs, sync::Mutex};

use crate::{
    contracts::ChannelCallRequest,
    service_registry::{
        ConfigurationService, PlatformService, ServiceError, ServiceId, ServiceRegistry,
        StorageService,
    },
};

const USER_DATA_DIR_ENV: &str = "CODE_TAURI_USER_DATA_DIR";
const VSCODE_PORTABLE_ENV: &str = "VSCODE_PORTABLE";
const DEFAULT_PRODUCT_NAME_LONG: &str = match option_env!("VSCODE_CLI_NAME_LONG") {
    Some(value) => value,
    None => "Code - OSS",
};

#[derive(Debug, Clone)]
pub struct UserDataPaths {
    root: PathBuf,
}

impl UserDataPaths {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn default_from_environment() -> Self {
        env::var(USER_DATA_DIR_ENV)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .map(Self::new)
            .unwrap_or_else(|| Self::new(default_user_data_root()))
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn user_dir(&self) -> PathBuf {
        self.root.join("User")
    }

    pub fn settings_resource(&self) -> PathBuf {
        self.user_dir().join("settings.json")
    }

    pub fn global_state_resource(&self) -> PathBuf {
        self.user_dir().join("globalState.json")
    }

    pub fn workspace_storage_home(&self) -> PathBuf {
        self.user_dir().join("workspaceStorage")
    }

    pub fn workspace_state_resource(&self, workspace: &str) -> PathBuf {
        self.workspace_storage_home()
            .join(sanitize_workspace_id(workspace))
            .join("state.json")
    }

    fn as_json(&self) -> Value {
        json!({
            "root": path_string(self.root()),
            "userDir": path_string(self.user_dir()),
            "settingsResource": path_string(self.settings_resource()),
            "globalStateResource": path_string(self.global_state_resource()),
            "workspaceStorageHome": path_string(self.workspace_storage_home()),
        })
    }
}

pub fn register_storage_config_services(registry: &mut ServiceRegistry, paths: UserDataPaths) {
    registry.register_configuration_service(Arc::new(JsonConfigurationService::new(paths.clone())));
    registry.register_storage_service(Arc::new(JsonStorageService::new(paths)));
}

#[derive(Debug)]
pub struct JsonConfigurationService {
    paths: UserDataPaths,
    writes: Mutex<()>,
}

impl JsonConfigurationService {
    pub fn new(paths: UserDataPaths) -> Self {
        Self {
            paths,
            writes: Mutex::new(()),
        }
    }

    async fn get_value(&self, request: ConfigGetRequest) -> Result<Value, ServiceError> {
        let settings = read_json_object(&self.paths.settings_resource()).await?;
        Ok(match request.key.as_deref() {
            Some(key) => get_path(&Value::Object(settings), key)
                .cloned()
                .unwrap_or(Value::Null),
            None => Value::Object(settings),
        })
    }

    async fn update_value(&self, request: ConfigUpdateRequest) -> Result<Value, ServiceError> {
        let _guard = self.writes.lock().await;
        let path = self.paths.settings_resource();
        let mut settings = Value::Object(read_json_object(&path).await?);

        match request.value {
            Some(value) => set_path(&mut settings, &request.key, value)?,
            None => remove_path(&mut settings, &request.key),
        }

        write_json_pretty(&path, &settings).await?;
        Ok(Value::Null)
    }
}

#[async_trait]
impl PlatformService for JsonConfigurationService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Configuration
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        match request.command.as_str() {
            "paths" | "statePaths" | "getStatePaths" => Ok(self.paths.as_json()),
            "all" | "getAll" => self.get_value(ConfigGetRequest { key: None }).await,
            "get" | "getValue" => self.get_value(first_arg(request.args)?).await,
            "update" | "updateValue" | "setValue" => {
                self.update_value(first_arg(request.args)?).await
            }
            command => Err(ServiceError::unsupported(ServiceId::Configuration, command)),
        }
    }
}

impl ConfigurationService for JsonConfigurationService {}

#[derive(Debug)]
pub struct JsonStorageService {
    paths: UserDataPaths,
    writes: Mutex<()>,
}

impl JsonStorageService {
    pub fn new(paths: UserDataPaths) -> Self {
        Self {
            paths,
            writes: Mutex::new(()),
        }
    }

    async fn get_item(&self, request: StorageKeyRequest) -> Result<Value, ServiceError> {
        let state = read_storage_state(&self.storage_resource(&request)?).await?;
        Ok(state.get(&request.key).cloned().unwrap_or(Value::Null))
    }

    async fn get_items(&self, request: StorageKeysRequest) -> Result<Value, ServiceError> {
        let resource = self.storage_resource(&request.key_request())?;
        let state = read_storage_state(&resource).await?;
        let mut values = Map::new();
        for key in request.keys {
            values.insert(key.clone(), state.get(&key).cloned().unwrap_or(Value::Null));
        }
        Ok(Value::Object(values))
    }

    async fn set_item(&self, request: StorageSetRequest) -> Result<Value, ServiceError> {
        let _guard = self.writes.lock().await;
        let resource = self.storage_resource(&request.key_request())?;
        let mut state = read_storage_state(&resource).await?;
        state.insert(request.key, request.value);
        write_json_pretty(&resource, &Value::Object(state.into_iter().collect())).await?;
        Ok(Value::Null)
    }

    async fn set_items(&self, request: StorageSetManyRequest) -> Result<Value, ServiceError> {
        let _guard = self.writes.lock().await;
        let resource = self.storage_resource(&request.key_request())?;
        let mut state = read_storage_state(&resource).await?;
        for (key, value) in request.items {
            state.insert(key, value);
        }
        write_json_pretty(&resource, &Value::Object(state.into_iter().collect())).await?;
        Ok(Value::Null)
    }

    async fn remove_item(&self, request: StorageKeyRequest) -> Result<Value, ServiceError> {
        let _guard = self.writes.lock().await;
        let resource = self.storage_resource(&request)?;
        let mut state = read_storage_state(&resource).await?;
        state.remove(&request.key);
        write_json_pretty(&resource, &Value::Object(state.into_iter().collect())).await?;
        Ok(Value::Null)
    }

    async fn remove_items(&self, request: StorageKeysRequest) -> Result<Value, ServiceError> {
        let _guard = self.writes.lock().await;
        let resource = self.storage_resource(&request.key_request())?;
        let mut state = read_storage_state(&resource).await?;
        for key in request.keys {
            state.remove(&key);
        }
        write_json_pretty(&resource, &Value::Object(state.into_iter().collect())).await?;
        Ok(Value::Null)
    }

    async fn clear(&self, request: StorageScopeRequest) -> Result<Value, ServiceError> {
        let _guard = self.writes.lock().await;
        let resource = self.storage_resource(&StorageKeyRequest {
            scope: request.scope,
            workspace: request.workspace,
            key: String::new(),
        })?;
        write_json_pretty(&resource, &Value::Object(Map::new())).await?;
        Ok(Value::Null)
    }

    async fn keys(&self, request: StorageScopeRequest) -> Result<Value, ServiceError> {
        let resource = self.storage_resource(&StorageKeyRequest {
            scope: request.scope,
            workspace: request.workspace,
            key: String::new(),
        })?;
        let state = read_storage_state(&resource).await?;
        Ok(Value::Array(
            state.keys().cloned().map(Value::String).collect(),
        ))
    }

    fn storage_resource(&self, request: &StorageKeyRequest) -> Result<PathBuf, ServiceError> {
        match request.scope.unwrap_or(StorageScope::Global) {
            StorageScope::Global | StorageScope::Application | StorageScope::Profile => {
                Ok(self.paths.global_state_resource())
            }
            StorageScope::Workspace => {
                let workspace = request.workspace.as_deref().ok_or_else(|| {
                    service_error(
                        "storage.invalidArgument",
                        "workspace storage requires workspace",
                    )
                })?;
                Ok(self.paths.workspace_state_resource(workspace))
            }
        }
    }
}

#[async_trait]
impl PlatformService for JsonStorageService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Storage
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        match request.command.as_str() {
            "paths" | "statePaths" | "getStatePaths" => Ok(self.paths.as_json()),
            "get" | "getItem" => self.get_item(first_arg(request.args)?).await,
            "getItems" | "mget" => self.get_items(first_arg(request.args)?).await,
            "set" | "setItem" | "store" => self.set_item(first_arg(request.args)?).await,
            "setItems" | "mset" => self.set_items(first_arg(request.args)?).await,
            "delete" | "remove" | "removeItem" => self.remove_item(first_arg(request.args)?).await,
            "deleteItems" | "removeItems" | "mdelete" => {
                self.remove_items(first_arg(request.args)?).await
            }
            "clear" | "clearScope" => self.clear(first_arg(request.args)?).await,
            "keys" | "getKeys" => self.keys(first_arg(request.args)?).await,
            command => Err(ServiceError::unsupported(ServiceId::Storage, command)),
        }
    }
}

impl StorageService for JsonStorageService {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigGetRequest {
    #[serde(default)]
    key: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigUpdateRequest {
    key: String,
    #[serde(default)]
    value: Option<Value>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
enum StorageScope {
    Global,
    Application,
    Profile,
    Workspace,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StorageScopeRequest {
    #[serde(default)]
    scope: Option<StorageScope>,
    #[serde(default)]
    workspace: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StorageKeyRequest {
    #[serde(default)]
    scope: Option<StorageScope>,
    #[serde(default)]
    workspace: Option<String>,
    key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StorageKeysRequest {
    #[serde(default)]
    scope: Option<StorageScope>,
    #[serde(default)]
    workspace: Option<String>,
    #[serde(default)]
    keys: Vec<String>,
}

impl StorageKeysRequest {
    fn key_request(&self) -> StorageKeyRequest {
        StorageKeyRequest {
            scope: self.scope,
            workspace: self.workspace.clone(),
            key: String::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StorageSetRequest {
    #[serde(default)]
    scope: Option<StorageScope>,
    #[serde(default)]
    workspace: Option<String>,
    key: String,
    value: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StorageSetManyRequest {
    #[serde(default)]
    scope: Option<StorageScope>,
    #[serde(default)]
    workspace: Option<String>,
    #[serde(default)]
    items: BTreeMap<String, Value>,
}

impl StorageSetManyRequest {
    fn key_request(&self) -> StorageKeyRequest {
        StorageKeyRequest {
            scope: self.scope,
            workspace: self.workspace.clone(),
            key: String::new(),
        }
    }
}

impl StorageSetRequest {
    fn key_request(&self) -> StorageKeyRequest {
        StorageKeyRequest {
            scope: self.scope,
            workspace: self.workspace.clone(),
            key: self.key.clone(),
        }
    }
}

fn first_arg<T>(args: Vec<Value>) -> Result<T, ServiceError>
where
    T: for<'de> Deserialize<'de>,
{
    let arg = args.into_iter().next().unwrap_or_else(|| json!({}));
    serde_json::from_value(arg)
        .map_err(|error| service_error("service.invalidArgument", error.to_string()))
}

async fn read_json_object(path: &Path) -> Result<Map<String, Value>, ServiceError> {
    let bytes = match fs::read(path).await {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Map::new()),
        Err(error) => {
            return Err(service_error(
                "storage.readFailed",
                format!("failed to read {}: {error}", path.display()),
            ))
        }
    };

    if bytes.is_empty() {
        return Ok(Map::new());
    }

    match serde_json::from_slice::<Value>(&bytes).map_err(|error| {
        service_error(
            "storage.invalidJson",
            format!("failed to parse {}: {error}", path.display()),
        )
    })? {
        Value::Object(object) => Ok(object),
        _ => Err(service_error(
            "storage.invalidJson",
            format!("{} must contain a JSON object", path.display()),
        )),
    }
}

async fn read_storage_state(path: &Path) -> Result<BTreeMap<String, Value>, ServiceError> {
    Ok(read_json_object(path).await?.into_iter().collect())
}

async fn write_json_pretty(path: &Path, value: &Value) -> Result<(), ServiceError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.map_err(|error| {
            service_error(
                "storage.writeFailed",
                format!("failed to create {}: {error}", parent.display()),
            )
        })?;
    }

    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| service_error("storage.serializationFailed", error.to_string()))?;
    fs::write(path, bytes).await.map_err(|error| {
        service_error(
            "storage.writeFailed",
            format!("failed to write {}: {error}", path.display()),
        )
    })
}

fn get_path<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    let mut current = value;
    for segment in key.split('.') {
        current = current.get(segment)?;
    }
    Some(current)
}

fn set_path(value: &mut Value, key: &str, new_value: Value) -> Result<(), ServiceError> {
    let segments = key_segments(key)?;
    let mut current = value;

    for segment in &segments[..segments.len() - 1] {
        if !current.is_object() {
            *current = Value::Object(Map::new());
        }
        current = current
            .as_object_mut()
            .expect("object just ensured")
            .entry(segment.to_string())
            .or_insert_with(|| Value::Object(Map::new()));
    }

    if !current.is_object() {
        *current = Value::Object(Map::new());
    }
    current
        .as_object_mut()
        .expect("object just ensured")
        .insert(
            segments.last().expect("validated key").to_string(),
            new_value,
        );
    Ok(())
}

fn remove_path(value: &mut Value, key: &str) {
    let mut current = value;
    let mut segments = key
        .split('.')
        .filter(|segment| !segment.is_empty())
        .peekable();

    while let Some(segment) = segments.next() {
        if segments.peek().is_none() {
            if let Some(object) = current.as_object_mut() {
                object.remove(segment);
            }
            return;
        }

        match current.get_mut(segment) {
            Some(next) => current = next,
            None => return,
        }
    }
}

fn key_segments(key: &str) -> Result<Vec<&str>, ServiceError> {
    let segments = key
        .split('.')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    if segments.is_empty() {
        Err(service_error(
            "config.invalidKey",
            "configuration key is required",
        ))
    } else {
        Ok(segments)
    }
}

fn default_user_data_root() -> PathBuf {
    let portable = env::var(VSCODE_PORTABLE_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);
    let home = env::var("HOME").ok().filter(|value| !value.is_empty());
    let xdg_config_home = env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|value| !value.is_empty());
    let appdata = env::var("APPDATA").ok().filter(|value| !value.is_empty());
    default_user_data_root_for_environment(
        portable.as_deref(),
        home.as_deref(),
        xdg_config_home.as_deref(),
        appdata.as_deref(),
    )
}

fn default_user_data_root_for_environment(
    portable: Option<&Path>,
    home: Option<&str>,
    xdg_config_home: Option<&str>,
    appdata: Option<&str>,
) -> PathBuf {
    #[cfg(not(all(unix, not(target_os = "macos"))))]
    let _ = xdg_config_home;
    #[cfg(not(target_os = "windows"))]
    let _ = appdata;

    if let Some(portable) = portable {
        return portable.join("user-data");
    }

    #[cfg(target_os = "windows")]
    {
        appdata
            .map(PathBuf::from)
            .or_else(|| home.map(PathBuf::from))
            .unwrap_or_else(env::temp_dir)
            .join(DEFAULT_PRODUCT_NAME_LONG)
    }

    #[cfg(target_os = "macos")]
    {
        home.map(PathBuf::from)
            .unwrap_or_else(env::temp_dir)
            .join("Library")
            .join("Application Support")
            .join(DEFAULT_PRODUCT_NAME_LONG)
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        xdg_config_home
            .map(PathBuf::from)
            .or_else(|| home.map(|value| PathBuf::from(value).join(".config")))
            .unwrap_or_else(env::temp_dir)
            .join(DEFAULT_PRODUCT_NAME_LONG)
    }

    #[cfg(not(any(unix, target_os = "windows")))]
    {
        home.map(PathBuf::from)
            .unwrap_or_else(env::temp_dir)
            .join(DEFAULT_PRODUCT_NAME_LONG)
    }
}

fn sanitize_workspace_id(workspace: &str) -> String {
    let sanitized = workspace
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => character,
            _ => '_',
        })
        .collect::<String>();

    if sanitized.is_empty() {
        "default".to_string()
    } else {
        sanitized
    }
}

fn path_string(path: impl AsRef<Path>) -> String {
    path.as_ref().to_string_lossy().into_owned()
}

fn service_error(code: &'static str, message: impl Into<String>) -> ServiceError {
    ServiceError {
        code,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_paths(name: &str) -> UserDataPaths {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        UserDataPaths::new(env::temp_dir().join(format!("vscode-atomic-{name}-{now}")))
    }

    fn request(channel: &str, command: &str, arg: Value) -> ChannelCallRequest {
        ChannelCallRequest {
            request_id: "req-1".to_string(),
            channel: channel.to_string(),
            command: command.to_string(),
            args: vec![arg],
            cancellation_id: None,
        }
    }

    #[tokio::test]
    async fn configuration_update_and_get_roundtrip_settings_json() {
        let paths = test_paths("config");
        let service = JsonConfigurationService::new(paths.clone());

        service
            .call(request(
                "configuration",
                "updateValue",
                json!({ "key": "editor.fontSize", "value": 14 }),
            ))
            .await
            .expect("update settings");

        let value = service
            .call(request(
                "configuration",
                "getValue",
                json!({ "key": "editor.fontSize" }),
            ))
            .await
            .expect("read setting");

        assert_eq!(value, json!(14));
        let persisted = fs::read_to_string(paths.settings_resource())
            .await
            .expect("settings persisted");
        assert!(persisted.contains("fontSize"));
    }

    #[tokio::test]
    async fn storage_preserves_global_and_workspace_state() {
        let paths = test_paths("storage");
        let service = JsonStorageService::new(paths.clone());

        service
            .call(request(
                "storage",
                "setItem",
                json!({ "key": "welcome", "value": true }),
            ))
            .await
            .expect("set global state");
        service
            .call(request(
                "storage",
                "setItem",
                json!({ "scope": "workspace", "workspace": "file:///repo", "key": "recent", "value": ["a.rs"] }),
            ))
            .await
            .expect("set workspace state");

        let global = service
            .call(request("storage", "getItem", json!({ "key": "welcome" })))
            .await
            .expect("get global state");
        let workspace = service
            .call(request(
                "storage",
                "getItem",
                json!({ "scope": "workspace", "workspace": "file:///repo", "key": "recent" }),
            ))
            .await
            .expect("get workspace state");

        assert_eq!(global, json!(true));
        assert_eq!(workspace, json!(["a.rs"]));
        assert!(paths.global_state_resource().exists());
        assert!(paths.workspace_state_resource("file:///repo").exists());
    }

    #[test]
    fn register_storage_config_services_installs_both_traits() {
        let mut registry = ServiceRegistry::new();
        register_storage_config_services(&mut registry, test_paths("registry"));

        assert!(registry.get(ServiceId::Configuration).is_some());
        assert!(registry.get(ServiceId::Storage).is_some());
    }

    #[test]
    fn default_user_data_root_matches_vscode_locations() {
        let portable = PathBuf::from("/portable/code");
        assert_eq!(
            default_user_data_root_for_environment(
                Some(&portable),
                Some("/home/tester"),
                Some("/config"),
                Some("C:/Users/tester/AppData/Roaming"),
            ),
            portable.join("user-data")
        );

        #[cfg(all(unix, not(target_os = "macos")))]
        assert_eq!(
            default_user_data_root_for_environment(
                None,
                Some("/home/tester"),
                Some("/config"),
                None,
            ),
            PathBuf::from("/config").join(DEFAULT_PRODUCT_NAME_LONG)
        );

        #[cfg(all(unix, not(target_os = "macos")))]
        assert_eq!(
            default_user_data_root_for_environment(None, Some("/home/tester"), None, None),
            PathBuf::from("/home/tester")
                .join(".config")
                .join(DEFAULT_PRODUCT_NAME_LONG)
        );

        #[cfg(target_os = "macos")]
        assert_eq!(
            default_user_data_root_for_environment(None, Some("/Users/tester"), None, None),
            PathBuf::from("/Users/tester")
                .join("Library")
                .join("Application Support")
                .join(DEFAULT_PRODUCT_NAME_LONG)
        );

        #[cfg(target_os = "windows")]
        assert_eq!(
            default_user_data_root_for_environment(
                None,
                Some("C:/Users/tester"),
                None,
                Some("C:/Users/tester/AppData/Roaming"),
            ),
            PathBuf::from("C:/Users/tester/AppData/Roaming").join(DEFAULT_PRODUCT_NAME_LONG)
        );
    }
}
