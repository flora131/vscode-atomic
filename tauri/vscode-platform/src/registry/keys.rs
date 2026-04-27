//! Built-in registry extension-point key constants.
//!
//! Mirrors the `Extensions` namespace in
//! src/vs/platform/registry/common/platform.ts.

use crate::registry::RegistryKey;

// ── Placeholder value types ───────────────────────────────────────────────────
// These will be replaced with concrete trait objects once the respective
// registries (editor, view, config, dnd) are fully implemented.

/// Marker type for editor factory registrations.
pub struct EditorFactoryContribution;

/// Marker type for editor pane registrations.
pub struct EditorPaneContribution;

/// Marker type for view-container registrations.
pub struct ViewContainerContribution;

/// Marker type for configuration registrations.
pub struct ConfigurationContribution;

/// Marker type for drag-and-drop handler registrations.
pub struct DragAndDropContribution;

// ── Key constants ─────────────────────────────────────────────────────────────

/// Extension point for editor factory contributions.
pub static EDITOR_FACTORIES: RegistryKey<EditorFactoryContribution> =
    RegistryKey::new("vscode.platform.editor.factories");

/// Extension point for editor pane (panel) contributions.
pub static EDITOR_PANES: RegistryKey<EditorPaneContribution> =
    RegistryKey::new("vscode.platform.editor.panes");

/// Extension point for view container contributions (sidebar/panel).
pub static VIEW_CONTAINERS: RegistryKey<ViewContainerContribution> =
    RegistryKey::new("vscode.platform.viewContainers");

/// Extension point for configuration schema contributions.
pub static CONFIGURATION: RegistryKey<ConfigurationContribution> =
    RegistryKey::new("vscode.platform.configuration");

/// Extension point for drag-and-drop handler contributions.
pub static DRAG_AND_DROP: RegistryKey<DragAndDropContribution> =
    RegistryKey::new("vscode.platform.dnd");
