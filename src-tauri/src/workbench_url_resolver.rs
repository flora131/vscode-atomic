use std::{
    ffi::{OsStr, OsString},
    path::{Component, Path, PathBuf},
};

pub(crate) const WORKBENCH_URL_ENV: &str = "CODE_TAURI_WORKBENCH_URL";
pub(crate) const WORKBENCH_PATH_ENV: &str = "CODE_TAURI_WORKBENCH_PATH";
pub(crate) const REQUIRE_REAL_WORKBENCH_ENV: &str = "CODE_TAURI_REQUIRE_REAL_WORKBENCH";
pub(crate) const FALLBACK_APP_ENTRY: &str = "index.html";
pub(crate) const FALLBACK_APP_ENTRY_DISPLAY: &str = "src-tauri/www/index.html";
pub(crate) const DEFAULT_WORKBENCH_ENTRY: &str = "out/vs/code/browser/workbench/workbench.html";
const PACKAGED_APP_DIR: &str = "src-tauri/www";

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ResolvedWorkbenchUrl {
    External(String),
    File(PathBuf),
    App(&'static str),
}

pub(crate) fn resolve_workbench_url_with(
    env_value: impl Fn(&str) -> Option<String>,
    file_exists: impl Fn(&Path) -> bool,
    repo_root: &Path,
) -> ResolvedWorkbenchUrl {
    resolve_workbench_url_with_release_policy(
        env_value,
        file_exists,
        repo_root,
        release_build_requires_real_workbench(),
    )
}

fn resolve_workbench_url_with_release_policy(
    env_value: impl Fn(&str) -> Option<String>,
    file_exists: impl Fn(&Path) -> bool,
    repo_root: &Path,
    release_build_requires_real_workbench: bool,
) -> ResolvedWorkbenchUrl {
    let require_real_workbench =
        release_build_requires_real_workbench || env_flag(&env_value, REQUIRE_REAL_WORKBENCH_ENV);

    if let Some(url) = env_value(WORKBENCH_URL_ENV) {
        if require_real_workbench {
            panic_external_workbench_url_rejected();
        }

        return ResolvedWorkbenchUrl::External(url);
    }

    if let Some(path) = env_value(WORKBENCH_PATH_ENV) {
        let path = resolve_workbench_path(repo_root, PathBuf::from(path));
        if require_real_workbench
            && (is_scaffold_workbench_path(repo_root, &path, &file_exists)
                || !is_real_workbench_path(repo_root, &path, &file_exists))
        {
            panic_real_workbench_required();
        }

        if file_exists(&path) {
            return ResolvedWorkbenchUrl::File(path);
        }
    }

    if file_exists(&packaged_app_workbench_path(repo_root)) {
        return ResolvedWorkbenchUrl::App(DEFAULT_WORKBENCH_ENTRY);
    }

    let default_workbench_path = repo_root.join(DEFAULT_WORKBENCH_ENTRY);
    if file_exists(&default_workbench_path) {
        return ResolvedWorkbenchUrl::File(default_workbench_path);
    }

    if require_real_workbench {
        panic_real_workbench_required();
    }

    ResolvedWorkbenchUrl::App(FALLBACK_APP_ENTRY)
}

#[cfg(not(debug_assertions))]
const fn release_build_requires_real_workbench() -> bool {
    true
}

#[cfg(debug_assertions)]
const fn release_build_requires_real_workbench() -> bool {
    false
}

fn panic_real_workbench_required() -> ! {
    panic!(
        "{REQUIRE_REAL_WORKBENCH_ENV}=1 requires existing {WORKBENCH_PATH_ENV} or bundled workbench {DEFAULT_WORKBENCH_ENTRY}; refusing developer scaffold fallback {FALLBACK_APP_ENTRY_DISPLAY}"
    );
}

fn panic_external_workbench_url_rejected() -> ! {
    panic!(
        "{REQUIRE_REAL_WORKBENCH_ENV}=1 rejects {WORKBENCH_URL_ENV}; bundled workbench {DEFAULT_WORKBENCH_ENTRY} is required"
    );
}

fn resolve_workbench_path(repo_root: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        repo_root.join(path)
    }
}

fn packaged_app_workbench_path(repo_root: &Path) -> PathBuf {
    repo_root
        .join(PACKAGED_APP_DIR)
        .join(DEFAULT_WORKBENCH_ENTRY)
}

fn canonicalize_existing_path(path: &Path) -> Option<PathBuf> {
    path.canonicalize().ok()
}

fn is_scaffold_workbench_path(
    repo_root: &Path,
    path: &Path,
    file_exists: impl Fn(&Path) -> bool,
) -> bool {
    let scaffold_path = repo_root.join(FALLBACK_APP_ENTRY_DISPLAY);
    path_matches_workbench_candidate(path, &scaffold_path, &file_exists)
}

fn is_real_workbench_path(
    repo_root: &Path,
    path: &Path,
    file_exists: impl Fn(&Path) -> bool,
) -> bool {
    let source_workbench_path = repo_root.join(DEFAULT_WORKBENCH_ENTRY);
    let packaged_workbench_path = packaged_app_workbench_path(repo_root);
    path_matches_workbench_candidate(path, &source_workbench_path, &file_exists)
        || path_matches_workbench_candidate(path, &packaged_workbench_path, &file_exists)
}

fn path_matches_workbench_candidate(
    path: &Path,
    candidate: &Path,
    file_exists: &impl Fn(&Path) -> bool,
) -> bool {
    if lexical_normalize_path(path) == lexical_normalize_path(candidate) {
        return true;
    }

    if !file_exists(path) {
        return false;
    }

    let Some(path) = canonicalize_existing_path(path) else {
        return false;
    };
    let Some(candidate) = canonicalize_existing_path(candidate) else {
        return false;
    };

    path == candidate
}

fn lexical_normalize_path(path: &Path) -> PathBuf {
    let mut prefix = None;
    let mut has_root = false;
    let mut parts = Vec::<OsString>::new();

    for component in path.components() {
        match component {
            Component::Prefix(path_prefix) => {
                prefix = Some(path_prefix.as_os_str().to_os_string());
                parts.clear();
            }
            Component::RootDir => {
                has_root = true;
                parts.clear();
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if parts.last().is_some_and(|part| part != OsStr::new("..")) {
                    parts.pop();
                } else if !has_root {
                    parts.push(OsString::from(".."));
                }
            }
            Component::Normal(part) => parts.push(part.to_os_string()),
        }
    }

    let mut normalized = PathBuf::new();
    if let Some(prefix) = prefix {
        normalized.push(prefix);
    }
    if has_root {
        normalized.push(Component::RootDir.as_os_str());
    }
    for part in parts {
        normalized.push(part);
    }

    normalized
}

fn env_flag(env_value: &impl Fn(&str) -> Option<String>, key: &str) -> bool {
    env_value(key).is_some_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        fs,
        path::Path,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn uses_scaffold_fallback_when_real_workbench_not_required() {
        assert_eq!(
            resolve_for(&[], &[]),
            ResolvedWorkbenchUrl::App(FALLBACK_APP_ENTRY)
        );
    }

    #[test]
    fn packaged_app_asset_wins_over_source_checkout_workbench() {
        let packaged_workbench = packaged_present_path(DEFAULT_WORKBENCH_ENTRY);

        assert_eq!(
            resolve_for(&[], &[packaged_workbench.as_str(), DEFAULT_WORKBENCH_ENTRY],),
            ResolvedWorkbenchUrl::App(DEFAULT_WORKBENCH_ENTRY)
        );
    }

    #[test]
    fn packaged_app_asset_resolves_to_default_workbench_entry() {
        let packaged_workbench = packaged_present_path(DEFAULT_WORKBENCH_ENTRY);

        assert_eq!(
            resolve_for(&[], &[packaged_workbench.as_str()]),
            ResolvedWorkbenchUrl::App(DEFAULT_WORKBENCH_ENTRY)
        );
    }

    #[test]
    fn default_resolver_does_not_use_source_checkout_when_packaged_asset_exists() {
        let packaged_workbench = packaged_present_path(DEFAULT_WORKBENCH_ENTRY);

        assert_eq!(
            resolve_for(&[], &[packaged_workbench.as_str(), DEFAULT_WORKBENCH_ENTRY]),
            ResolvedWorkbenchUrl::App(DEFAULT_WORKBENCH_ENTRY)
        );
    }

    #[test]
    fn release_resolver_does_not_use_source_checkout_when_packaged_asset_exists() {
        let packaged_workbench = packaged_present_path(DEFAULT_WORKBENCH_ENTRY);
        assert_eq!(
            resolve_for_release_policy(
                &[],
                &[packaged_workbench.as_str(), DEFAULT_WORKBENCH_ENTRY],
                true,
            ),
            ResolvedWorkbenchUrl::App(DEFAULT_WORKBENCH_ENTRY)
        );
    }

    #[test]
    #[should_panic(
        expected = "CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 requires existing CODE_TAURI_WORKBENCH_PATH or bundled workbench out/vs/code/browser/workbench/workbench.html; refusing developer scaffold fallback src-tauri/www/index.html"
    )]
    fn rejects_scaffold_fallback_when_real_workbench_required() {
        resolve_for(&[(REQUIRE_REAL_WORKBENCH_ENV, "1")], &[]);
    }

    #[test]
    #[should_panic(
        expected = "CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 requires existing CODE_TAURI_WORKBENCH_PATH or bundled workbench out/vs/code/browser/workbench/workbench.html; refusing developer scaffold fallback src-tauri/www/index.html"
    )]
    fn rejects_scaffold_fallback_when_release_requires_real_workbench() {
        resolve_for_release_policy(&[], &[], true);
    }

    #[test]
    #[should_panic(
        expected = "CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 requires existing CODE_TAURI_WORKBENCH_PATH or bundled workbench out/vs/code/browser/workbench/workbench.html; refusing developer scaffold fallback src-tauri/www/index.html"
    )]
    fn rejects_scaffold_fallback_asset_when_release_requires_real_workbench() {
        resolve_for_release_policy(&[], &[FALLBACK_APP_ENTRY_DISPLAY], true);
    }

    #[test]
    fn uses_packaged_workbench_asset_when_release_requires_real_workbench() {
        let packaged_workbench = packaged_present_path(DEFAULT_WORKBENCH_ENTRY);

        assert_eq!(
            resolve_for_release_policy(&[], &[packaged_workbench.as_str()], true),
            ResolvedWorkbenchUrl::App(DEFAULT_WORKBENCH_ENTRY)
        );
    }

    #[test]
    #[should_panic(
        expected = "CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 requires existing CODE_TAURI_WORKBENCH_PATH or bundled workbench out/vs/code/browser/workbench/workbench.html; refusing developer scaffold fallback src-tauri/www/index.html"
    )]
    fn rejects_explicit_scaffold_path_when_real_workbench_required() {
        resolve_for(
            &[
                (REQUIRE_REAL_WORKBENCH_ENV, "1"),
                (WORKBENCH_PATH_ENV, FALLBACK_APP_ENTRY_DISPLAY),
            ],
            &[FALLBACK_APP_ENTRY_DISPLAY],
        );
    }

    #[test]
    #[should_panic(
        expected = "CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 requires existing CODE_TAURI_WORKBENCH_PATH or bundled workbench out/vs/code/browser/workbench/workbench.html; refusing developer scaffold fallback src-tauri/www/index.html"
    )]
    fn rejects_explicit_scaffold_path_when_release_requires_real_workbench() {
        resolve_for_release_policy(
            &[(WORKBENCH_PATH_ENV, FALLBACK_APP_ENTRY_DISPLAY)],
            &[FALLBACK_APP_ENTRY_DISPLAY],
            true,
        );
    }

    #[test]
    #[should_panic(
        expected = "CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 requires existing CODE_TAURI_WORKBENCH_PATH or bundled workbench out/vs/code/browser/workbench/workbench.html; refusing developer scaffold fallback src-tauri/www/index.html"
    )]
    fn rejects_normalized_explicit_scaffold_path_when_real_workbench_required() {
        resolve_for(
            &[
                (REQUIRE_REAL_WORKBENCH_ENV, "1"),
                (
                    WORKBENCH_PATH_ENV,
                    "./src-tauri/../src-tauri/www/index.html",
                ),
            ],
            &[FALLBACK_APP_ENTRY_DISPLAY],
        );
    }

    #[test]
    fn allows_explicit_scaffold_path_when_real_workbench_not_required() {
        assert_eq!(
            resolve_for(
                &[(WORKBENCH_PATH_ENV, FALLBACK_APP_ENTRY_DISPLAY)],
                &[FALLBACK_APP_ENTRY_DISPLAY],
            ),
            ResolvedWorkbenchUrl::File(PathBuf::from("/repo/src-tauri/www/index.html"))
        );
    }

    #[test]
    #[should_panic(
        expected = "CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 rejects CODE_TAURI_WORKBENCH_URL; bundled workbench out/vs/code/browser/workbench/workbench.html is required"
    )]
    fn rejects_explicit_url_when_real_workbench_required() {
        resolve_for(
            &[
                (REQUIRE_REAL_WORKBENCH_ENV, "1"),
                (WORKBENCH_URL_ENV, "https://example.test/workbench"),
            ],
            &[],
        );
    }

    #[test]
    #[should_panic(
        expected = "CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 rejects CODE_TAURI_WORKBENCH_URL; bundled workbench out/vs/code/browser/workbench/workbench.html is required"
    )]
    fn rejects_explicit_url_when_release_requires_real_workbench() {
        resolve_for_release_policy(
            &[(WORKBENCH_URL_ENV, "https://example.test/workbench")],
            &[],
            true,
        );
    }

    #[test]
    fn allows_explicit_url_in_debug_when_real_workbench_not_required() {
        assert_eq!(
            resolve_for_release_policy(
                &[(WORKBENCH_URL_ENV, "https://example.test/workbench")],
                &[],
                false,
            ),
            ResolvedWorkbenchUrl::External("https://example.test/workbench".to_string())
        );
    }

    #[test]
    fn explicit_path_wins_over_detected_workbench() {
        assert_eq!(
            resolve_for(
                &[(WORKBENCH_PATH_ENV, "/custom/workbench.html")],
                &["/custom/workbench.html", DEFAULT_WORKBENCH_ENTRY],
            ),
            ResolvedWorkbenchUrl::File(PathBuf::from("/custom/workbench.html"))
        );
    }

    #[test]
    fn relative_explicit_path_resolves_against_repo_root_even_when_cwd_differs() {
        let repo_root = Path::new("/repo");
        let cwd = Path::new("/different/cwd");
        let envs = HashMap::from([(
            WORKBENCH_PATH_ENV.to_string(),
            DEFAULT_WORKBENCH_ENTRY.to_string(),
        )]);

        assert_eq!(
            resolve_workbench_url_with(
                |key| envs.get(key).cloned(),
                |path| {
                    assert_ne!(path, &cwd.join(DEFAULT_WORKBENCH_ENTRY));
                    path == repo_root.join(DEFAULT_WORKBENCH_ENTRY)
                },
                repo_root,
            ),
            ResolvedWorkbenchUrl::File(PathBuf::from(
                "/repo/out/vs/code/browser/workbench/workbench.html"
            ))
        );
    }

    #[test]
    fn missing_explicit_path_falls_through_to_detected_workbench() {
        assert_eq!(
            resolve_for(
                &[(WORKBENCH_PATH_ENV, "/missing/workbench.html")],
                &[DEFAULT_WORKBENCH_ENTRY],
            ),
            ResolvedWorkbenchUrl::File(PathBuf::from(
                "/repo/out/vs/code/browser/workbench/workbench.html"
            ))
        );
    }

    #[test]
    fn rejects_non_workbench_explicit_path_when_real_workbench_required() {
        let result = std::panic::catch_unwind(|| {
            resolve_for(
                &[
                    (REQUIRE_REAL_WORKBENCH_ENV, "on"),
                    (WORKBENCH_PATH_ENV, "/custom/workbench.html"),
                ],
                &["/custom/workbench.html"],
            );
        });

        assert!(result.is_err());
    }

    #[test]
    fn allows_source_explicit_path_when_real_workbench_required() {
        assert_eq!(
            resolve_for(
                &[
                    (REQUIRE_REAL_WORKBENCH_ENV, "on"),
                    (WORKBENCH_PATH_ENV, DEFAULT_WORKBENCH_ENTRY),
                ],
                &[DEFAULT_WORKBENCH_ENTRY],
            ),
            ResolvedWorkbenchUrl::File(PathBuf::from(
                "/repo/out/vs/code/browser/workbench/workbench.html"
            ))
        );
    }

    #[test]
    fn explicit_workbench_path_still_wins_for_source_ci_when_packaged_asset_exists() {
        let packaged_workbench = packaged_present_path(DEFAULT_WORKBENCH_ENTRY);

        assert_eq!(
            resolve_for(
                &[(WORKBENCH_PATH_ENV, DEFAULT_WORKBENCH_ENTRY)],
                &[packaged_workbench.as_str(), DEFAULT_WORKBENCH_ENTRY],
            ),
            ResolvedWorkbenchUrl::File(PathBuf::from(
                "/repo/out/vs/code/browser/workbench/workbench.html"
            ))
        );
    }

    #[test]
    fn rejects_non_workbench_explicit_path_when_release_requires_real_workbench() {
        let result = std::panic::catch_unwind(|| {
            resolve_for_release_policy(
                &[(WORKBENCH_PATH_ENV, "/custom/workbench.html")],
                &["/custom/workbench.html"],
                true,
            );
        });

        assert!(result.is_err());
    }

    #[test]
    #[should_panic(
        expected = "CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 requires existing CODE_TAURI_WORKBENCH_PATH or bundled workbench out/vs/code/browser/workbench/workbench.html; refusing developer scaffold fallback src-tauri/www/index.html"
    )]
    fn rejects_missing_explicit_path_when_release_requires_real_workbench() {
        resolve_for_release_policy(
            &[(WORKBENCH_PATH_ENV, "/missing/workbench.html")],
            &[],
            true,
        );
    }

    #[test]
    fn allows_source_explicit_path_when_release_requires_real_workbench() {
        assert_eq!(
            resolve_for_release_policy(
                &[(WORKBENCH_PATH_ENV, DEFAULT_WORKBENCH_ENTRY)],
                &[DEFAULT_WORKBENCH_ENTRY],
                true,
            ),
            ResolvedWorkbenchUrl::File(PathBuf::from(
                "/repo/out/vs/code/browser/workbench/workbench.html"
            ))
        );
    }

    #[test]
    fn allows_packaged_explicit_path_when_release_requires_real_workbench() {
        let packaged_workbench = packaged_present_path(DEFAULT_WORKBENCH_ENTRY);

        assert_eq!(
            resolve_for_release_policy(
                &[(WORKBENCH_PATH_ENV, packaged_workbench.as_str())],
                &[packaged_workbench.as_str()],
                true,
            ),
            ResolvedWorkbenchUrl::File(PathBuf::from(
                "/repo/src-tauri/www/out/vs/code/browser/workbench/workbench.html"
            ))
        );
    }

    #[test]
    #[should_panic(
        expected = "CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 requires existing CODE_TAURI_WORKBENCH_PATH or bundled workbench out/vs/code/browser/workbench/workbench.html; refusing developer scaffold fallback src-tauri/www/index.html"
    )]
    fn rejects_explicit_scaffold_path_when_release_requires_real_workbench_even_if_detected_exists()
    {
        resolve_for_release_policy(
            &[(WORKBENCH_PATH_ENV, FALLBACK_APP_ENTRY_DISPLAY)],
            &[FALLBACK_APP_ENTRY_DISPLAY, DEFAULT_WORKBENCH_ENTRY],
            true,
        );
    }

    #[test]
    fn allows_detected_workbench_when_real_workbench_required() {
        assert_eq!(
            resolve_for(
                &[(REQUIRE_REAL_WORKBENCH_ENV, "true")],
                &[DEFAULT_WORKBENCH_ENTRY],
            ),
            ResolvedWorkbenchUrl::File(PathBuf::from(
                "/repo/out/vs/code/browser/workbench/workbench.html"
            ))
        );
    }

    #[test]
    fn allows_detected_workbench_when_release_requires_real_workbench() {
        assert_eq!(
            resolve_for_release_policy(&[], &[DEFAULT_WORKBENCH_ENTRY], true),
            ResolvedWorkbenchUrl::File(PathBuf::from(
                "/repo/out/vs/code/browser/workbench/workbench.html"
            ))
        );
    }

    #[test]
    fn detects_scaffold_workbench_path_for_direct_and_normalized_paths() {
        let repo = TempWorkbenchRepo::new("direct-normalized-scaffold");
        repo.write_file(FALLBACK_APP_ENTRY_DISPLAY);

        assert!(is_scaffold_workbench_path(
            repo.root(),
            &repo.root().join(FALLBACK_APP_ENTRY_DISPLAY),
            Path::exists,
        ));
        assert!(is_scaffold_workbench_path(
            repo.root(),
            &repo.root().join("src-tauri/../src-tauri/www/./index.html"),
            Path::exists,
        ));
    }

    #[test]
    fn rejects_symlink_to_scaffold_when_real_workbench_required() {
        let repo = TempWorkbenchRepo::new("scaffold-symlink");
        repo.write_file(FALLBACK_APP_ENTRY_DISPLAY);
        if !repo.write_symlink_file(FALLBACK_APP_ENTRY_DISPLAY, "linked-workbench.html") {
            return;
        }

        assert!(is_scaffold_workbench_path(
            repo.root(),
            &repo.root().join("linked-workbench.html"),
            Path::exists,
        ));

        let envs = HashMap::from([
            (REQUIRE_REAL_WORKBENCH_ENV.to_string(), "1".to_string()),
            (
                WORKBENCH_PATH_ENV.to_string(),
                "linked-workbench.html".to_string(),
            ),
        ]);

        let result = std::panic::catch_unwind(|| {
            resolve_workbench_url_with(|key| envs.get(key).cloned(), Path::exists, repo.root());
        });

        assert!(result.is_err());
    }

    #[test]
    fn accepts_symlink_to_real_generated_workbench_when_real_workbench_required() {
        let repo = TempWorkbenchRepo::new("real-workbench-symlink");
        repo.write_file(FALLBACK_APP_ENTRY_DISPLAY);
        repo.write_file(DEFAULT_WORKBENCH_ENTRY);
        if !repo.write_symlink_file(DEFAULT_WORKBENCH_ENTRY, "linked-workbench.html") {
            return;
        }

        let linked_workbench = repo.root().join("linked-workbench.html");

        assert!(!is_scaffold_workbench_path(
            repo.root(),
            &linked_workbench,
            Path::exists,
        ));
        assert_eq!(
            resolve_workbench_url_with(
                |key| match key {
                    REQUIRE_REAL_WORKBENCH_ENV => Some("1".to_string()),
                    WORKBENCH_PATH_ENV => Some("linked-workbench.html".to_string()),
                    _ => None,
                },
                Path::exists,
                repo.root(),
            ),
            ResolvedWorkbenchUrl::File(linked_workbench)
        );
    }

    #[test]
    fn detects_real_workbench_path_for_source_and_packaged_paths() {
        let repo = TempWorkbenchRepo::new("real-source-packaged");
        repo.write_file(DEFAULT_WORKBENCH_ENTRY);
        repo.write_file(&packaged_present_path(DEFAULT_WORKBENCH_ENTRY));

        assert!(is_real_workbench_path(
            repo.root(),
            &repo.root().join(DEFAULT_WORKBENCH_ENTRY),
            Path::exists,
        ));
        assert!(is_real_workbench_path(
            repo.root(),
            &repo
                .root()
                .join(packaged_present_path(DEFAULT_WORKBENCH_ENTRY)),
            Path::exists,
        ));
        assert!(!is_real_workbench_path(
            repo.root(),
            &repo.root().join(FALLBACK_APP_ENTRY_DISPLAY),
            Path::exists,
        ));
    }

    fn resolve_for(envs: &[(&str, &str)], present_paths: &[&str]) -> ResolvedWorkbenchUrl {
        resolve_for_release_policy(envs, present_paths, release_build_requires_real_workbench())
    }

    fn resolve_for_release_policy(
        envs: &[(&str, &str)],
        present_paths: &[&str],
        release_build_requires_real_workbench: bool,
    ) -> ResolvedWorkbenchUrl {
        let envs = envs
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect::<HashMap<_, _>>();
        let repo_root = Path::new("/repo");

        resolve_workbench_url_with_release_policy(
            |key| envs.get(key).cloned(),
            |path| {
                present_paths.iter().any(|present_path| {
                    path == resolve_workbench_path(repo_root, PathBuf::from(*present_path))
                })
            },
            repo_root,
            release_build_requires_real_workbench,
        )
    }

    fn packaged_present_path(path: &str) -> String {
        format!("{PACKAGED_APP_DIR}/{path}")
    }

    struct TempWorkbenchRepo {
        root: PathBuf,
    }

    impl TempWorkbenchRepo {
        fn new(name: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time must be after unix epoch")
                .as_nanos();
            let root = std::env::temp_dir().join(format!(
                "vscode-atomic-workbench-url-resolver-{name}-{unique}"
            ));

            fs::create_dir_all(&root).expect("create temporary workbench repo");

            Self { root }
        }

        fn root(&self) -> &Path {
            &self.root
        }

        fn write_file(&self, path: &str) {
            let path = self.root.join(path);
            fs::create_dir_all(path.parent().expect("test path must have parent"))
                .expect("create test file parent directory");
            fs::write(path, b"workbench").expect("write test file");
        }

        fn write_symlink_file(&self, target: &str, link: &str) -> bool {
            create_file_symlink(&self.root.join(target), &self.root.join(link)).is_ok()
        }
    }

    impl Drop for TempWorkbenchRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[cfg(unix)]
    fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
        std::os::unix::fs::symlink(target, link)
    }

    #[cfg(windows)]
    fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
        std::os::windows::fs::symlink_file(target, link)
    }

    #[cfg(not(any(unix, windows)))]
    fn create_file_symlink(_target: &Path, _link: &Path) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file symlinks are unsupported on this platform",
        ))
    }
}
