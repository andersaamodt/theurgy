pub mod product_runtime {
    use std::error::Error;
    use std::fmt;

    use serde_json::Value;

    pub const PRODUCT_IR_SCHEMA: &str = "theurgy-product-ir/v1";
    pub const DESKTOP_SURFACE_IR_SCHEMA: &str = "theurgy-desktop-surface-ir/v1";
    pub const MOBILE_SURFACE_IR_SCHEMA: &str = "theurgy-mobile-surface-ir/v1";
    pub const ACTION_IR_SCHEMA: &str = "theurgy-action-ir/v1";
    pub const STATE_SNAPSHOT_SCHEMA: &str = "theurgy-state-snapshot/v1";
    pub const RUNTIME_STATUS_SCHEMA: &str = "theurgy-runtime-status/v1";
    pub const RUNTIME_ACTION_PROTOCOL: &str = "theurgy-runtime-action/v1";
    pub const RUNTIME_ACTION_REQUEST_SCHEMA: &str = "theurgy-runtime-action-request/v1";
    pub const RUNTIME_ACTION_RESULT_SCHEMA: &str = "theurgy-runtime-action-result/v1";
    pub const OPERATION_STATUS_SCHEMA: &str = "theurgy-operation-status/v1";
    pub const OPERATION_HISTORY_SCHEMA: &str = "theurgy-operation-history/v1";
    pub const RUNTIME_MANIFEST_SCHEMA: &str = "theurgy-runtime-manifest/v1";
    pub const GENERATED_RUNTIME_SCHEMA: &str = "theurgy-generated-runtime/v1";

    pub const DESKTOP_ADAPTER_TRANSPORT: &str = "local-process-json";
    pub const MOBILE_ADAPTER_TRANSPORT: &str = "external-json-abi";

    pub fn adapter_runtime_transport(target: &str) -> &'static str {
        if is_desktop_target(target) {
            DESKTOP_ADAPTER_TRANSPORT
        } else {
            MOBILE_ADAPTER_TRANSPORT
        }
    }

    pub fn is_desktop_target(target: &str) -> bool {
        matches!(target, "macos" | "linux")
    }

    pub fn is_mobile_target(target: &str) -> bool {
        matches!(target, "ios" | "android")
    }

    pub fn surface_family_for_target(target: &str) -> Option<&'static str> {
        if is_desktop_target(target) {
            Some("desktop")
        } else if is_mobile_target(target) {
            Some("mobile")
        } else {
            None
        }
    }

    pub fn surface_schema_for_target(target: &str) -> Option<&'static str> {
        if is_desktop_target(target) {
            Some(DESKTOP_SURFACE_IR_SCHEMA)
        } else if is_mobile_target(target) {
            Some(MOBILE_SURFACE_IR_SCHEMA)
        } else {
            None
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct StateSnapshot {
        pub app_id: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RuntimeStatus {
        pub app_id: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RuntimeActionRequest {
        pub app_id: String,
        pub action_id: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RuntimeActionResult {
        pub app_id: String,
        pub action_id: String,
        pub operation_id: String,
        pub long_running: bool,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct OperationStatus {
        pub app_id: String,
        pub operation_id: String,
        pub long_running: bool,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct OperationHistory {
        pub app_id: String,
        pub entries: usize,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RuntimeManifest {
        pub app_id: String,
        pub product_ir: String,
        pub desktop_surface_ir: Option<String>,
        pub mobile_surface_ir: Option<String>,
        pub legacy_native_desktop_ir: Option<String>,
        pub protocol: String,
        pub compatibility: RuntimeCompatibility,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct RuntimeCompatibility {
        pub wizardry_apps_shell_first_still_supported: bool,
        pub theurgy_required_for_legacy_wizardry_apps: bool,
    }

    impl RuntimeCompatibility {
        pub fn shell_first_default() -> Self {
            Self {
                wizardry_apps_shell_first_still_supported: true,
                theurgy_required_for_legacy_wizardry_apps: false,
            }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct ContractError {
        message: String,
    }

    impl ContractError {
        fn new(message: impl Into<String>) -> Self {
            Self {
                message: message.into(),
            }
        }
    }

    impl fmt::Display for ContractError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.message)
        }
    }

    impl Error for ContractError {}

    pub type ContractResult<T> = std::result::Result<T, ContractError>;

    pub fn validate_state_snapshot_value(value: &Value) -> ContractResult<StateSnapshot> {
        expect_value_string(value, "schema", STATE_SNAPSHOT_SCHEMA)?;
        let app_id = value_string(value, "app")
            .filter(|id| valid_slug(id))
            .ok_or_else(|| ContractError::new("state snapshot app must be a lowercase slug"))?;
        value_string(value, "generatedAt")
            .filter(|generated_at| !generated_at.is_empty())
            .ok_or_else(|| ContractError::new("state snapshot generatedAt required"))?;
        value_object(value, "data")?;
        Ok(StateSnapshot { app_id })
    }

    pub fn validate_runtime_status_value(value: &Value) -> ContractResult<RuntimeStatus> {
        expect_value_string(value, "schema", RUNTIME_STATUS_SCHEMA)?;
        let app_id = value_string(value, "app")
            .filter(|id| valid_slug(id))
            .ok_or_else(|| ContractError::new("runtime status app must be a lowercase slug"))?;
        value_string(value, "generatedAt")
            .filter(|generated_at| !generated_at.is_empty())
            .ok_or_else(|| ContractError::new("runtime status generatedAt required"))?;
        value
            .get("state_ready")
            .and_then(Value::as_bool)
            .ok_or_else(|| ContractError::new("runtime status state_ready must be boolean"))?;
        Ok(RuntimeStatus { app_id })
    }

    pub fn validate_runtime_action_request_value(
        value: &Value,
    ) -> ContractResult<RuntimeActionRequest> {
        expect_value_string(value, "protocol", RUNTIME_ACTION_PROTOCOL)?;
        let app_id = value_string(value, "app")
            .filter(|id| valid_slug(id))
            .ok_or_else(|| {
                ContractError::new("runtime action request app must be a lowercase slug")
            })?;
        let action_id = value_string(value, "action")
            .filter(|id| valid_action_id(id))
            .ok_or_else(|| {
                ContractError::new("runtime action request action must be a stable action id")
            })?;
        value_object(value, "params")?;
        Ok(RuntimeActionRequest { app_id, action_id })
    }

    pub fn validate_runtime_action_result_value(
        value: &Value,
    ) -> ContractResult<RuntimeActionResult> {
        expect_value_string(value, "protocol", RUNTIME_ACTION_PROTOCOL)?;
        let app_id = value_string(value, "app")
            .filter(|id| valid_slug(id))
            .ok_or_else(|| {
                ContractError::new("runtime action result app must be a lowercase slug")
            })?;
        let action_id = value_string(value, "action")
            .filter(|id| valid_action_id(id))
            .ok_or_else(|| {
                ContractError::new("runtime action result action must be a stable action id")
            })?;
        let operation = value_object(value, "operation")?;
        let (operation_id, long_running) = validate_operation_record(operation)?;
        if value.get("result").is_none() {
            return Err(ContractError::new("runtime action result result required"));
        }
        Ok(RuntimeActionResult {
            app_id,
            action_id,
            operation_id,
            long_running,
        })
    }

    pub fn validate_operation_status_value(value: &Value) -> ContractResult<OperationStatus> {
        expect_value_string(value, "schema", OPERATION_STATUS_SCHEMA)?;
        let app_id = value_string(value, "app")
            .filter(|id| valid_slug(id))
            .ok_or_else(|| ContractError::new("operation status app must be a lowercase slug"))?;
        value_string(value, "generatedAt")
            .filter(|generated_at| !generated_at.is_empty())
            .ok_or_else(|| ContractError::new("operation status generatedAt required"))?;
        let operation = value_object(value, "operation")?;
        let (operation_id, long_running) = validate_operation_record(operation)?;
        Ok(OperationStatus {
            app_id,
            operation_id,
            long_running,
        })
    }

    pub fn validate_operation_history_value(value: &Value) -> ContractResult<OperationHistory> {
        expect_value_string(value, "schema", OPERATION_HISTORY_SCHEMA)?;
        let app_id = value_string(value, "app")
            .filter(|id| valid_slug(id))
            .ok_or_else(|| ContractError::new("operation history app must be a lowercase slug"))?;
        value_string(value, "generatedAt")
            .filter(|generated_at| !generated_at.is_empty())
            .ok_or_else(|| ContractError::new("operation history generatedAt required"))?;
        let entries = value_array(value, "data")?;
        Ok(OperationHistory {
            app_id,
            entries: entries.len(),
        })
    }

    pub fn validate_runtime_manifest_value(value: &Value) -> ContractResult<RuntimeManifest> {
        expect_value_string(value, "version", RUNTIME_MANIFEST_SCHEMA)?;
        let app_id = value_string(value, "app")
            .filter(|id| valid_slug(id))
            .ok_or_else(|| ContractError::new("runtime manifest app must be a lowercase slug"))?;
        let product_ir = value_string(value, "productIr")
            .filter(|path| !path.is_empty())
            .ok_or_else(|| ContractError::new("runtime manifest productIr required"))?;
        let (desktop_surface_ir, mobile_surface_ir, legacy_native_desktop_ir) =
            runtime_manifest_surface_paths(value)?;
        let compatibility = validate_runtime_manifest_compatibility(value)?;
        let runtime = value_object(value, "runtime")?;
        let state_command = value_string_array(runtime, "stateCommand")?;
        let action_command = value_string_array(runtime, "actionCommand")?;
        if state_command.is_empty() || action_command.is_empty() {
            return Err(ContractError::new(
                "runtime manifest commands must be non-empty arrays",
            ));
        }
        let subscribe_status_command = optional_string_array(
            runtime,
            "subscribeStatusCommand",
            "runtime manifest subscribeStatusCommand",
        )?;
        if runtime.get("subscribeStatusCommand").is_some() && subscribe_status_command.is_empty() {
            return Err(ContractError::new(
                "runtime manifest subscribeStatusCommand must be non-empty",
            ));
        }
        let operation_status_command = optional_string_array(
            runtime,
            "operationStatusCommand",
            "runtime manifest operationStatusCommand",
        )?;
        if runtime.get("operationStatusCommand").is_some() && operation_status_command.is_empty() {
            return Err(ContractError::new(
                "runtime manifest operationStatusCommand must be non-empty",
            ));
        }
        let protocol = value_string(runtime, "protocol")
            .filter(|protocol| !protocol.is_empty())
            .ok_or_else(|| ContractError::new("runtime manifest protocol required"))?;
        Ok(RuntimeManifest {
            app_id,
            product_ir,
            desktop_surface_ir,
            mobile_surface_ir,
            legacy_native_desktop_ir,
            protocol,
            compatibility,
        })
    }

    fn runtime_manifest_surface_paths(
        value: &Value,
    ) -> ContractResult<(Option<String>, Option<String>, Option<String>)> {
        let Some(surfaces) = value.get("surfaces") else {
            return Ok((None, None, None));
        };
        if !surfaces.is_object() {
            return Err(ContractError::new(
                "runtime manifest surfaces must be an object",
            ));
        }
        Ok((
            optional_nonempty_string(surfaces, "desktop", "runtime manifest surfaces.desktop")?,
            optional_nonempty_string(surfaces, "mobile", "runtime manifest surfaces.mobile")?,
            optional_nonempty_string(
                surfaces,
                "legacyNativeDesktop",
                "runtime manifest surfaces.legacyNativeDesktop",
            )?,
        ))
    }

    fn validate_runtime_manifest_compatibility(
        value: &Value,
    ) -> ContractResult<RuntimeCompatibility> {
        let Some(compatibility) = value.get("compatibility") else {
            return Ok(RuntimeCompatibility::shell_first_default());
        };
        let compatibility = compatibility.as_object().ok_or_else(|| {
            ContractError::new("runtime manifest compatibility must be an object")
        })?;
        let defaults = RuntimeCompatibility::shell_first_default();
        Ok(RuntimeCompatibility {
            wizardry_apps_shell_first_still_supported: optional_object_bool(
                compatibility,
                "wizardryAppsShellFirstStillSupported",
                "runtime manifest compatibility.wizardryAppsShellFirstStillSupported",
            )?
            .unwrap_or(defaults.wizardry_apps_shell_first_still_supported),
            theurgy_required_for_legacy_wizardry_apps: optional_object_bool(
                compatibility,
                "theurgyRequiredForLegacyWizardryApps",
                "runtime manifest compatibility.theurgyRequiredForLegacyWizardryApps",
            )?
            .unwrap_or(defaults.theurgy_required_for_legacy_wizardry_apps),
        })
    }

    fn validate_operation_record(value: &Value) -> ContractResult<(String, bool)> {
        let id = value_string(value, "id")
            .filter(|id| !id.is_empty())
            .ok_or_else(|| ContractError::new("runtime operation.id required"))?;
        let status = value_string(value, "status")
            .ok_or_else(|| ContractError::new("runtime operation.status required"))?;
        if !matches!(
            status.as_str(),
            "accepted" | "running" | "completed" | "failed" | "cancelled"
        ) {
            return Err(ContractError::new("runtime operation.status invalid"));
        }
        let progress = value
            .get("progress")
            .and_then(Value::as_u64)
            .ok_or_else(|| ContractError::new("runtime operation.progress integer required"))?;
        if progress > 100 {
            return Err(ContractError::new(
                "runtime operation.progress must be 0..100",
            ));
        }
        let long_running = value
            .get("longRunning")
            .and_then(Value::as_bool)
            .ok_or_else(|| ContractError::new("runtime operation.longRunning boolean required"))?;
        Ok((id, long_running))
    }

    fn expect_value_string(value: &Value, key: &str, expected: &str) -> ContractResult<()> {
        match value_string(value, key) {
            Some(actual) if actual == expected => Ok(()),
            _ => Err(ContractError::new(format!("expected {key} = {expected}"))),
        }
    }

    fn value_string(value: &Value, key: &str) -> Option<String> {
        value.get(key)?.as_str().map(String::from)
    }

    fn value_object<'a>(value: &'a Value, key: &str) -> ContractResult<&'a Value> {
        value
            .get(key)
            .filter(|candidate| candidate.is_object())
            .ok_or_else(|| ContractError::new(format!("missing JSON object key: {key}")))
    }

    fn value_array<'a>(value: &'a Value, key: &str) -> ContractResult<&'a Vec<Value>> {
        value
            .get(key)
            .and_then(Value::as_array)
            .ok_or_else(|| ContractError::new(format!("missing JSON array key: {key}")))
    }

    fn value_string_array(value: &Value, key: &str) -> ContractResult<Vec<String>> {
        let array = value_array(value, key)?;
        let mut values = Vec::new();
        for item in array {
            let Some(string) = item.as_str() else {
                return Err(ContractError::new(format!(
                    "JSON array key {key} must contain strings"
                )));
            };
            values.push(string.to_string());
        }
        Ok(values)
    }

    fn optional_string_array(value: &Value, key: &str, label: &str) -> ContractResult<Vec<String>> {
        let Some(raw) = value.get(key) else {
            return Ok(Vec::new());
        };
        let Some(array) = raw.as_array() else {
            return Err(ContractError::new(format!("{label} must be an array")));
        };
        let mut values = Vec::new();
        for item in array {
            let Some(text) = item.as_str().filter(|text| !text.is_empty()) else {
                return Err(ContractError::new(format!(
                    "{label} must contain non-empty strings"
                )));
            };
            values.push(text.to_string());
        }
        Ok(values)
    }

    fn optional_nonempty_string(
        value: &Value,
        key: &str,
        label: &str,
    ) -> ContractResult<Option<String>> {
        let Some(raw) = value.get(key) else {
            return Ok(None);
        };
        let Some(text) = raw.as_str().filter(|text| !text.is_empty()) else {
            return Err(ContractError::new(format!(
                "{label} must be a non-empty string"
            )));
        };
        Ok(Some(text.to_string()))
    }

    fn optional_object_bool(
        object: &serde_json::Map<String, Value>,
        key: &str,
        label: &str,
    ) -> ContractResult<Option<bool>> {
        object
            .get(key)
            .map(|value| {
                value
                    .as_bool()
                    .ok_or_else(|| ContractError::new(format!("{label} must be boolean")))
            })
            .transpose()
    }

    fn valid_slug(value: &str) -> bool {
        !value.is_empty()
            && value.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_')
            })
    }

    fn valid_action_id(value: &str) -> bool {
        !value.is_empty()
            && value.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'-' | b'_' | b'.')
            })
    }
}

#[cfg(test)]
mod tests {
    use super::product_runtime;

    #[test]
    fn product_runtime_declares_cross_platform_contract_ids() {
        assert_eq!(product_runtime::PRODUCT_IR_SCHEMA, "theurgy-product-ir/v1");
        assert_eq!(
            product_runtime::DESKTOP_SURFACE_IR_SCHEMA,
            "theurgy-desktop-surface-ir/v1"
        );
        assert_eq!(
            product_runtime::MOBILE_SURFACE_IR_SCHEMA,
            "theurgy-mobile-surface-ir/v1"
        );
        assert_eq!(
            product_runtime::RUNTIME_ACTION_PROTOCOL,
            "theurgy-runtime-action/v1"
        );
        assert_eq!(
            product_runtime::RUNTIME_MANIFEST_SCHEMA,
            "theurgy-runtime-manifest/v1"
        );
    }

    #[test]
    fn product_runtime_maps_targets_to_surface_families_and_transports() {
        assert_eq!(
            product_runtime::surface_family_for_target("macos"),
            Some("desktop")
        );
        assert_eq!(
            product_runtime::surface_family_for_target("android"),
            Some("mobile")
        );
        assert_eq!(product_runtime::surface_family_for_target("web"), None);
        assert_eq!(
            product_runtime::surface_schema_for_target("linux"),
            Some(product_runtime::DESKTOP_SURFACE_IR_SCHEMA)
        );
        assert_eq!(
            product_runtime::surface_schema_for_target("ios"),
            Some(product_runtime::MOBILE_SURFACE_IR_SCHEMA)
        );
        assert_eq!(
            product_runtime::adapter_runtime_transport("linux"),
            product_runtime::DESKTOP_ADAPTER_TRANSPORT
        );
        assert_eq!(
            product_runtime::adapter_runtime_transport("ios"),
            product_runtime::MOBILE_ADAPTER_TRANSPORT
        );
    }

    #[test]
    fn product_runtime_validates_action_request_abi() {
        let value = serde_json::json!({
            "protocol": product_runtime::RUNTIME_ACTION_PROTOCOL,
            "app": "deployments",
            "action": "publish_changes",
            "params": {"deployment": "site-one"}
        });
        let request = product_runtime::validate_runtime_action_request_value(&value).unwrap();
        assert_eq!(request.app_id, "deployments");
        assert_eq!(request.action_id, "publish_changes");

        let invalid = serde_json::json!({
            "protocol": product_runtime::RUNTIME_ACTION_PROTOCOL,
            "app": "Deployments",
            "action": "publish_changes",
            "params": {}
        });
        let error = product_runtime::validate_runtime_action_request_value(&invalid)
            .unwrap_err()
            .to_string();
        assert_eq!(error, "runtime action request app must be a lowercase slug");
    }

    #[test]
    fn product_runtime_validates_state_and_status_abi() {
        let snapshot = serde_json::json!({
            "schema": product_runtime::STATE_SNAPSHOT_SCHEMA,
            "app": "deployments",
            "generatedAt": "2026-06-11T00:00:00Z",
            "data": {"deployments": []}
        });
        let snapshot = product_runtime::validate_state_snapshot_value(&snapshot).unwrap();
        assert_eq!(snapshot.app_id, "deployments");

        let status = serde_json::json!({
            "schema": product_runtime::RUNTIME_STATUS_SCHEMA,
            "app": "deployments",
            "generatedAt": "2026-06-11T00:00:00Z",
            "state_ready": true
        });
        let status = product_runtime::validate_runtime_status_value(&status).unwrap();
        assert_eq!(status.app_id, "deployments");

        let invalid = serde_json::json!({
            "schema": product_runtime::RUNTIME_STATUS_SCHEMA,
            "app": "deployments",
            "generatedAt": "2026-06-11T00:00:00Z",
            "state_ready": "yes"
        });
        let error = product_runtime::validate_runtime_status_value(&invalid)
            .unwrap_err()
            .to_string();
        assert_eq!(error, "runtime status state_ready must be boolean");
    }

    #[test]
    fn product_runtime_validates_action_result_abi() {
        let value = serde_json::json!({
            "protocol": product_runtime::RUNTIME_ACTION_PROTOCOL,
            "app": "deployments",
            "action": "publish_changes",
            "operation": {
                "id": "op-publish",
                "status": "completed",
                "progress": 100,
                "longRunning": true
            },
            "result": {"message": "published"}
        });
        let result = product_runtime::validate_runtime_action_result_value(&value).unwrap();
        assert_eq!(result.app_id, "deployments");
        assert_eq!(result.action_id, "publish_changes");
        assert_eq!(result.operation_id, "op-publish");
        assert!(result.long_running);

        let invalid = serde_json::json!({
            "protocol": product_runtime::RUNTIME_ACTION_PROTOCOL,
            "app": "deployments",
            "action": "publish_changes",
            "operation": {
                "id": "op-publish",
                "status": "completed",
                "progress": 101,
                "longRunning": true
            },
            "result": {}
        });
        let error = product_runtime::validate_runtime_action_result_value(&invalid)
            .unwrap_err()
            .to_string();
        assert_eq!(error, "runtime operation.progress must be 0..100");
    }

    #[test]
    fn product_runtime_validates_operation_status_and_history_abi() {
        let status = serde_json::json!({
            "schema": product_runtime::OPERATION_STATUS_SCHEMA,
            "app": "deployments",
            "generatedAt": "2026-06-11T00:00:00Z",
            "operation": {
                "id": "op-publish",
                "status": "running",
                "progress": 50,
                "longRunning": true
            }
        });
        let status = product_runtime::validate_operation_status_value(&status).unwrap();
        assert_eq!(status.app_id, "deployments");
        assert_eq!(status.operation_id, "op-publish");
        assert!(status.long_running);

        let history = serde_json::json!({
            "schema": product_runtime::OPERATION_HISTORY_SCHEMA,
            "app": "deployments",
            "generatedAt": "2026-06-11T00:00:00Z",
            "data": [
                {"id": "op-one"},
                {"id": "op-two"}
            ]
        });
        let history = product_runtime::validate_operation_history_value(&history).unwrap();
        assert_eq!(history.app_id, "deployments");
        assert_eq!(history.entries, 2);

        let invalid = serde_json::json!({
            "schema": product_runtime::OPERATION_HISTORY_SCHEMA,
            "app": "deployments",
            "generatedAt": "2026-06-11T00:00:00Z",
            "data": {}
        });
        let error = product_runtime::validate_operation_history_value(&invalid)
            .unwrap_err()
            .to_string();
        assert_eq!(error, "missing JSON array key: data");
    }

    #[test]
    fn product_runtime_validates_runtime_manifest_contract() {
        let manifest = serde_json::json!({
            "version": product_runtime::RUNTIME_MANIFEST_SCHEMA,
            "app": "deployments",
            "productIr": "app-blueprint/product.ir.json",
            "surfaces": {
                "desktop": "app-blueprint/desktop.surface.ir.json",
                "mobile": "app-blueprint/mobile.surface.ir.json",
                "legacyNativeDesktop": "app-blueprint/app.ir.yaml"
            },
            "runtime": {
                "protocol": product_runtime::RUNTIME_ACTION_PROTOCOL,
                "stateCommand": ["deployments-core", "runtime-state"],
                "actionCommand": ["deployments-core", "runtime-action"],
                "subscribeStatusCommand": ["deployments-core", "runtime-status"],
                "operationStatusCommand": ["deployments-core", "runtime-operation-status"]
            }
        });
        let manifest = product_runtime::validate_runtime_manifest_value(&manifest).unwrap();
        assert_eq!(manifest.app_id, "deployments");
        assert_eq!(manifest.product_ir, "app-blueprint/product.ir.json");
        assert_eq!(
            manifest.desktop_surface_ir.as_deref(),
            Some("app-blueprint/desktop.surface.ir.json")
        );
        assert_eq!(
            manifest.mobile_surface_ir.as_deref(),
            Some("app-blueprint/mobile.surface.ir.json")
        );
        assert_eq!(
            manifest.legacy_native_desktop_ir.as_deref(),
            Some("app-blueprint/app.ir.yaml")
        );
        assert_eq!(manifest.protocol, product_runtime::RUNTIME_ACTION_PROTOCOL);
        assert!(
            manifest
                .compatibility
                .wizardry_apps_shell_first_still_supported
        );
        assert!(
            !manifest
                .compatibility
                .theurgy_required_for_legacy_wizardry_apps
        );

        let invalid = serde_json::json!({
            "version": product_runtime::RUNTIME_MANIFEST_SCHEMA,
            "app": "deployments",
            "productIr": "app-blueprint/product.ir.json",
            "runtime": {
                "protocol": product_runtime::RUNTIME_ACTION_PROTOCOL,
                "stateCommand": [],
                "actionCommand": ["deployments-core", "runtime-action"]
            }
        });
        let error = product_runtime::validate_runtime_manifest_value(&invalid)
            .unwrap_err()
            .to_string();
        assert_eq!(error, "runtime manifest commands must be non-empty arrays");
    }
}
