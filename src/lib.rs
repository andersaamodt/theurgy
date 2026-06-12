pub mod product_runtime {
    use std::collections::{BTreeMap, BTreeSet};
    use std::error::Error;
    use std::fmt;
    use std::fs;
    use std::path::Path;

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

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RuntimeBridge {
        pub app_id: String,
        pub protocol: String,
        pub product_ir: String,
        pub runtime_manifest: String,
        pub source_surface_ir: String,
        pub legacy_native_desktop_ir: Option<String>,
        pub compatibility: RuntimeCompatibility,
        pub state_command: Vec<String>,
        pub status_command: Vec<String>,
        pub subscribe_status_command: Vec<String>,
        pub operation_status_command: Vec<String>,
        pub action_command: Vec<String>,
        pub history_command: Vec<String>,
        pub daemon_command: Vec<String>,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct GeneratedNativeFile {
        pub path: String,
        pub contents: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct GeneratedRuntime {
        pub app_id: String,
        pub target: String,
        pub release_target: String,
        pub release_artifact: String,
        pub state_snapshot_schema: String,
        pub persistence_truth: String,
        pub adapter_runtime_transport: String,
        pub runtime_status_schema: String,
        pub runtime_action_request_schema: String,
        pub runtime_action_result_schema: String,
        pub operation_status_schema: String,
        pub operation_history_schema: String,
        pub surface_schema: String,
        pub surface_target: String,
        pub actions: usize,
        pub product_actions: usize,
        pub surface_actions: usize,
        pub surface_action_contracts: usize,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ProductActionContract {
        pub id: String,
        pub label: String,
        pub effect: String,
        pub safe: bool,
        pub mutating: bool,
        pub long_running: bool,
        pub privileged: bool,
        pub command: Vec<String>,
        pub input_keys: Vec<String>,
        pub output_keys: Vec<String>,
        pub failure_keys: Vec<String>,
        pub input_shape: BTreeMap<String, String>,
        pub output_shape: BTreeMap<String, String>,
        pub failure_shape: BTreeMap<String, String>,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ActionIr {
        pub action_ids: Vec<String>,
        pub action_contracts: Vec<ProductActionContract>,
        pub actions: usize,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ProductIr {
        pub app_id: String,
        pub app_name: String,
        pub targets: Vec<String>,
        pub desktop_surface_ir: Option<String>,
        pub mobile_surface_ir: Option<String>,
        pub capabilities: Vec<String>,
        pub permissions: Vec<String>,
        pub domain_object_ids: Vec<String>,
        pub state_snapshot_schema: String,
        pub state_command: Vec<String>,
        pub state_status_command: Vec<String>,
        pub persistence_truth: String,
        pub persistence_root_ids: Vec<String>,
        pub background_jobs: Vec<BackgroundJob>,
        pub background_job_ids: Vec<String>,
        pub release_targets: Vec<ReleaseTarget>,
        pub audit_keys: Vec<String>,
        pub action_contracts: Vec<ProductActionContract>,
        pub action_ids: Vec<String>,
        pub actions: usize,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct BackgroundJob {
        pub id: String,
        pub command: Vec<String>,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ReleaseTarget {
        pub id: String,
        pub target: String,
        pub surface: String,
        pub artifact: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct SurfaceIr {
        pub schema: String,
        pub product: String,
        pub target: String,
        pub action_ids: Vec<String>,
        pub roles: Vec<String>,
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

    pub fn validate_runtime_action_params_text(
        action_id: &str,
        params: &str,
        contracts: &[ProductActionContract],
    ) -> ContractResult<()> {
        let value: Value = serde_json::from_str(params)
            .map_err(|error| ContractError::new(format!("invalid JSON: {error}")))?;
        validate_runtime_action_params_value(action_id, &value, contracts)
    }

    pub fn validate_runtime_action_params_value(
        action_id: &str,
        params: &Value,
        contracts: &[ProductActionContract],
    ) -> ContractResult<()> {
        let contract = runtime_action_contract(action_id, contracts)?;
        let Some(object) = params.as_object() else {
            return Err(ContractError::new(format!(
                "runtime action params must be a JSON object for Product IR action: {action_id}"
            )));
        };
        for key in object.keys() {
            if !contract
                .input_keys
                .iter()
                .any(|declared_key| declared_key == key)
            {
                return Err(ContractError::new(format!(
                    "runtime action param not declared in Product IR input for {action_id}: {key}"
                )));
            }
        }
        validate_shape_object(
            &contract.input_shape,
            object,
            "runtime action param",
            action_id,
        )
    }

    pub fn validate_runtime_action_result_contract_value(
        action_id: &str,
        result: &Value,
        contracts: &[ProductActionContract],
    ) -> ContractResult<()> {
        let contract = runtime_action_contract(action_id, contracts)?;
        let Some(object) = result.get("result").and_then(Value::as_object) else {
            return Err(ContractError::new(format!(
                "runtime action result must be a JSON object for Product IR action: {action_id}"
            )));
        };
        for key in object.keys() {
            if !contract
                .output_keys
                .iter()
                .any(|declared_key| declared_key == key)
            {
                return Err(ContractError::new(format!(
                    "runtime action result key not declared in Product IR output for {action_id}: {key}"
                )));
            }
        }
        validate_shape_object(
            &contract.output_shape,
            object,
            "runtime action result",
            action_id,
        )
    }

    pub fn validate_runtime_action_operation_contract(
        action_id: &str,
        actual_long_running: bool,
        contracts: &[ProductActionContract],
    ) -> ContractResult<()> {
        let contract = runtime_action_contract(action_id, contracts)?;
        if actual_long_running != contract.long_running {
            return Err(ContractError::new(format!(
                "runtime action operation.longRunning mismatch for {action_id}: expected {}, got {}",
                contract.long_running, actual_long_running
            )));
        }
        Ok(())
    }

    pub fn validate_runtime_action_failure_contract_text(
        action_id: &str,
        output: &str,
        contracts: &[ProductActionContract],
    ) -> ContractResult<()> {
        let Ok(value) = serde_json::from_str::<Value>(output) else {
            return Ok(());
        };
        validate_runtime_action_failure_contract_value(action_id, &value, contracts)
    }

    pub fn validate_runtime_action_failure_contract_value(
        action_id: &str,
        output: &Value,
        contracts: &[ProductActionContract],
    ) -> ContractResult<()> {
        if output.get("success").and_then(Value::as_bool) != Some(false) {
            return Ok(());
        }
        let contract = runtime_action_contract(action_id, contracts)?;
        let Some(object) = output.as_object() else {
            return Ok(());
        };
        for key in object.keys().filter(|key| key.as_str() != "success") {
            if !contract
                .failure_keys
                .iter()
                .any(|declared_key| declared_key == key)
            {
                return Err(ContractError::new(format!(
                    "runtime action failure key not declared in Product IR failure for {action_id}: {key}"
                )));
            }
        }
        let failure_object = object
            .iter()
            .filter(|(key, _)| key.as_str() != "success")
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<serde_json::Map<_, _>>();
        validate_shape_object(
            &contract.failure_shape,
            &failure_object,
            "runtime action failure",
            action_id,
        )
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

    pub fn load_runtime_manifest(path: impl AsRef<Path>) -> ContractResult<RuntimeManifest> {
        let path = path.as_ref();
        let text = fs::read_to_string(path)
            .map_err(|error| ContractError::new(format!("could not read JSON: {error}")))?;
        let value: Value = serde_json::from_str(&text)
            .map_err(|error| ContractError::new(format!("invalid JSON: {error}")))?;
        validate_runtime_manifest_value(&value)
    }

    pub fn runtime_bridge_from_manifest_text(text: &str) -> ContractResult<RuntimeBridge> {
        let value: Value = serde_json::from_str(text)
            .map_err(|error| ContractError::new(format!("invalid JSON: {error}")))?;
        runtime_bridge_from_manifest_value(&value)
    }

    pub fn runtime_bridge_from_manifest_value(value: &Value) -> ContractResult<RuntimeBridge> {
        let manifest = validate_runtime_manifest_value(value)?;
        let runtime = value_object(value, "runtime")?;
        Ok(RuntimeBridge {
            app_id: manifest.app_id,
            protocol: manifest.protocol,
            product_ir: manifest.product_ir,
            runtime_manifest: "theurgy-runtime.manifest.json".to_string(),
            source_surface_ir: "theurgy-surface.json".to_string(),
            legacy_native_desktop_ir: manifest.legacy_native_desktop_ir,
            compatibility: manifest.compatibility,
            state_command: value_string_array(runtime, "stateCommand")?,
            status_command: optional_string_array(
                runtime,
                "statusCommand",
                "runtime manifest statusCommand",
            )?,
            subscribe_status_command: optional_string_array(
                runtime,
                "subscribeStatusCommand",
                "runtime manifest subscribeStatusCommand",
            )?,
            operation_status_command: optional_string_array(
                runtime,
                "operationStatusCommand",
                "runtime manifest operationStatusCommand",
            )?,
            action_command: value_string_array(runtime, "actionCommand")?,
            history_command: optional_string_array(
                runtime,
                "historyCommand",
                "runtime manifest historyCommand",
            )?,
            daemon_command: optional_string_array(
                runtime,
                "daemonCommand",
                "runtime manifest daemonCommand",
            )?,
        })
    }

    pub fn validate_generated_runtime_text(text: &str) -> ContractResult<GeneratedRuntime> {
        let value: Value = serde_json::from_str(text)
            .map_err(|error| ContractError::new(format!("invalid JSON: {error}")))?;
        validate_generated_runtime_value(&value)
    }

    pub fn validate_generated_runtime_value(value: &Value) -> ContractResult<GeneratedRuntime> {
        expect_value_string(value, "version", GENERATED_RUNTIME_SCHEMA)?;
        let app_id = value_string(value, "app")
            .filter(|id| valid_slug(id))
            .ok_or_else(|| ContractError::new("generated runtime app must be a lowercase slug"))?;
        let target = value_string(value, "target")
            .filter(|target| matches!(target.as_str(), "macos" | "linux" | "ios" | "android"))
            .ok_or_else(|| {
                ContractError::new("generated runtime target must be macos, linux, ios, or android")
            })?;
        value_string(value, "protocol")
            .filter(|protocol| !protocol.is_empty())
            .ok_or_else(|| ContractError::new("generated runtime protocol required"))?;
        let runtime_status_schema =
            expect_and_return_value_string(value, "runtimeStatusSchema", RUNTIME_STATUS_SCHEMA)?;
        let runtime_action_request_schema = expect_and_return_value_string(
            value,
            "runtimeActionRequestSchema",
            RUNTIME_ACTION_REQUEST_SCHEMA,
        )?;
        let runtime_action_result_schema = expect_and_return_value_string(
            value,
            "runtimeActionResultSchema",
            RUNTIME_ACTION_RESULT_SCHEMA,
        )?;
        let operation_status_schema = expect_and_return_value_string(
            value,
            "operationStatusSchema",
            OPERATION_STATUS_SCHEMA,
        )?;
        let operation_history_schema = expect_and_return_value_string(
            value,
            "operationHistorySchema",
            OPERATION_HISTORY_SCHEMA,
        )?;
        value_string(value, "productIr")
            .filter(|path| !path.is_empty())
            .ok_or_else(|| ContractError::new("generated runtime productIr required"))?;
        value_string(value, "runtimeManifest")
            .filter(|path| !path.is_empty())
            .ok_or_else(|| ContractError::new("generated runtime runtimeManifest required"))?;
        value_string(value, "sourceSurfaceIr")
            .filter(|path| !path.is_empty())
            .ok_or_else(|| ContractError::new("generated runtime sourceSurfaceIr required"))?;
        optional_nonempty_string(
            value,
            "legacyNativeDesktopIr",
            "generated runtime legacyNativeDesktopIr",
        )?;
        value_bool(value, "compatibilityWizardryAppsShellFirstStillSupported").ok_or_else(|| {
            ContractError::new(
                "generated runtime compatibilityWizardryAppsShellFirstStillSupported boolean required",
            )
        })?;
        value_bool(value, "compatibilityTheurgyRequiredForLegacyWizardryApps").ok_or_else(|| {
            ContractError::new(
                "generated runtime compatibilityTheurgyRequiredForLegacyWizardryApps boolean required",
            )
        })?;
        let state_snapshot_schema = value_string(value, "productStateSnapshotSchema")
            .filter(|schema| !schema.is_empty())
            .ok_or_else(|| {
                ContractError::new("generated runtime productStateSnapshotSchema required")
            })?;
        let persistence_truth = value_string(value, "productPersistenceTruth")
            .filter(|truth| !truth.is_empty())
            .ok_or_else(|| {
                ContractError::new("generated runtime productPersistenceTruth required")
            })?;
        let adapter_runtime_transport =
            value_string(value, "adapterRuntimeTransport").ok_or_else(|| {
                ContractError::new("generated runtime adapterRuntimeTransport required")
            })?;
        match (target.as_str(), adapter_runtime_transport.as_str()) {
            ("macos" | "linux", DESKTOP_ADAPTER_TRANSPORT) => {}
            ("ios" | "android", MOBILE_ADAPTER_TRANSPORT) => {}
            _ => {
                return Err(ContractError::new(
                    "generated runtime adapterRuntimeTransport must match target family",
                ))
            }
        }
        let target_release_target = value_string(value, "targetReleaseTarget")
            .filter(|release_target| valid_action_id(release_target))
            .ok_or_else(|| ContractError::new("generated runtime targetReleaseTarget required"))?;
        let target_release_artifact = value_string(value, "targetReleaseArtifact")
            .filter(|artifact| !artifact.is_empty())
            .ok_or_else(|| {
                ContractError::new("generated runtime targetReleaseArtifact required")
            })?;
        for key in ["stateCommand", "subscribeStatusCommand", "actionCommand"] {
            if value_string_array(value, key)?.is_empty() {
                return Err(ContractError::new(format!(
                    "generated runtime {key} must be non-empty"
                )));
            }
        }
        let operation_status_command = optional_string_array(
            value,
            "operationStatusCommand",
            "generated runtime operationStatusCommand",
        )?;
        for key in [
            "statusCommand",
            "historyCommand",
            "daemonCommand",
            "productTargets",
            "productActions",
            "productCapabilities",
            "productPermissions",
            "productDomainObjects",
            "productPersistenceRoots",
            "productBackgroundJobs",
            "productReleaseTargets",
            "productAuditKeys",
            "surfaceActions",
            "surfaceRoles",
        ] {
            optional_string_array(value, key, &format!("generated runtime {key}"))?;
        }
        let product_release_targets = value_string_array(value, "productReleaseTargets")?;
        if !product_release_targets
            .iter()
            .any(|release_target| release_target == &target_release_target)
        {
            return Err(ContractError::new(
                "generated runtime targetReleaseTarget must be listed in productReleaseTargets",
            ));
        }
        let product_actions = value_string_array(value, "productActions")?;
        if product_actions.is_empty() {
            return Err(ContractError::new(
                "generated runtime productActions required",
            ));
        }
        let surface_actions = value_string_array(value, "surfaceActions")?;
        for action_id in &surface_actions {
            if !product_actions
                .iter()
                .any(|product_action| product_action == action_id)
            {
                return Err(ContractError::new(format!(
                    "generated runtime surface action not declared in productActions: {action_id}"
                )));
            }
        }
        let contracts = value_array(value, "productActionContracts")?;
        if contracts.len() != product_actions.len() {
            return Err(ContractError::new(
                "generated runtime productActionContracts must match productActions length",
            ));
        }
        let mut contract_ids = Vec::new();
        let mut has_long_running_action = false;
        for contract in contracts {
            contract_ids.push(validate_generated_action_contract(contract)?);
            if contract
                .get("longRunning")
                .and_then(Value::as_bool)
                .unwrap_or(false)
            {
                has_long_running_action = true;
            }
        }
        if has_long_running_action && operation_status_command.is_empty() {
            return Err(ContractError::new(
                "generated runtime operationStatusCommand required for long-running actions",
            ));
        }
        if contract_ids != product_actions {
            return Err(ContractError::new(
                "generated runtime productActionContracts order must match productActions",
            ));
        }
        let surface_contracts = value_array(value, "surfaceActionContracts")?;
        if surface_contracts.len() != surface_actions.len() {
            return Err(ContractError::new(
                "generated runtime surfaceActionContracts must match surfaceActions length",
            ));
        }
        let mut surface_contract_ids = Vec::new();
        for contract in surface_contracts {
            surface_contract_ids.push(validate_generated_action_contract(contract)?);
        }
        if surface_contract_ids != surface_actions {
            return Err(ContractError::new(
                "generated runtime surfaceActionContracts order must match surfaceActions",
            ));
        }
        for surface_contract in surface_contracts {
            let surface_id = surface_contract
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let Some(product_contract) = contracts.iter().find(|contract| {
                contract
                    .get("id")
                    .and_then(Value::as_str)
                    .map(|id| id == surface_id)
                    .unwrap_or(false)
            }) else {
                return Err(ContractError::new(format!(
                    "generated runtime surfaceActionContracts action missing from productActionContracts: {surface_id}"
                )));
            };
            if product_contract != surface_contract {
                return Err(ContractError::new(format!(
                    "generated runtime surfaceActionContracts must match productActionContracts for {surface_id}"
                )));
            }
        }
        value_string(value, "surface")
            .filter(|surface| !surface.is_empty())
            .ok_or_else(|| ContractError::new("generated runtime surface required"))?;
        let surface_schema = value_string(value, "surfaceSchema")
            .ok_or_else(|| ContractError::new("generated runtime surfaceSchema required"))?;
        if !matches!(
            surface_schema.as_str(),
            DESKTOP_SURFACE_IR_SCHEMA | MOBILE_SURFACE_IR_SCHEMA
        ) {
            return Err(ContractError::new(
                "generated runtime surfaceSchema invalid",
            ));
        }
        let expected_surface_schema = surface_schema_for_target(target.as_str())
            .ok_or_else(|| ContractError::new("generated runtime target invalid"))?;
        if surface_schema != expected_surface_schema {
            return Err(ContractError::new(
                "generated runtime surfaceSchema invalid for target",
            ));
        }
        let surface_target = value_string(value, "surfaceTarget")
            .ok_or_else(|| ContractError::new("generated runtime surfaceTarget required"))?;
        let expected_surface_target = if matches!(target.as_str(), "macos" | "linux") {
            "desktop"
        } else {
            "mobile"
        };
        if surface_target != target && surface_target != expected_surface_target {
            return Err(ContractError::new(
                "generated runtime surfaceTarget invalid for target",
            ));
        }
        Ok(GeneratedRuntime {
            app_id,
            target,
            release_target: target_release_target,
            release_artifact: target_release_artifact,
            state_snapshot_schema,
            persistence_truth,
            adapter_runtime_transport,
            runtime_status_schema,
            runtime_action_request_schema,
            runtime_action_result_schema,
            operation_status_schema,
            operation_history_schema,
            surface_schema,
            surface_target,
            actions: product_actions.len(),
            product_actions: product_actions.len(),
            surface_actions: surface_actions.len(),
            surface_action_contracts: surface_contracts.len(),
        })
    }

    pub fn validate_product_ir_value(value: &Value) -> ContractResult<ProductIr> {
        expect_value_string(value, "version", PRODUCT_IR_SCHEMA)?;
        expect_value_string(value, "format", "json")?;
        let app = value_object(value, "app")?;
        let app_id = value_string(app, "id")
            .filter(|id| valid_slug(id))
            .ok_or_else(|| ContractError::new("product IR app.id must be a lowercase slug"))?;
        let app_name = value_string(app, "name")
            .filter(|name| !name.is_empty())
            .ok_or_else(|| ContractError::new("product IR app.name required"))?;
        let targets = value_string_array(app, "targets")?;
        if targets.is_empty() {
            return Err(ContractError::new("product IR app.targets required"));
        }
        for target in &targets {
            if !matches!(target.as_str(), "macos" | "linux" | "ios" | "android") {
                return Err(ContractError::new(
                    "product IR target must be macos, linux, ios, or android",
                ));
            }
        }
        let capabilities =
            optional_string_array(app, "capabilities", "product IR app.capabilities")?;
        let permissions = optional_string_array(app, "permissions", "product IR app.permissions")?;
        let (desktop_surface_ir, mobile_surface_ir) = product_surface_paths(value)?;
        let domain_object_ids = optional_object_id_array(
            value.get("domain").unwrap_or(&Value::Null),
            "objects",
            "product IR domain.objects",
        )?;
        let action_values = value_array(value, "actions")?;
        if action_values.is_empty() {
            return Err(ContractError::new("product IR actions required"));
        }
        let mut action_ids = Vec::new();
        let mut action_contracts = Vec::new();
        for action in action_values {
            let contract = validate_product_action_contract(action)?;
            action_ids.push(contract.id.clone());
            action_contracts.push(contract);
        }
        let state = value_object(value, "state")?;
        let state_snapshot_schema = value_string(state, "snapshotSchema")
            .filter(|schema| !schema.is_empty())
            .ok_or_else(|| ContractError::new("product IR state.snapshotSchema required"))?;
        let state_command = optional_string_array(state, "command", "product IR state.command")?;
        let state_status_command =
            optional_string_array(state, "statusCommand", "product IR state.statusCommand")?;
        let persistence_root_ids =
            optional_object_id_array(state, "roots", "product IR state.roots")?;
        let persistence_truth = validate_product_persistence(value)?;
        let background_jobs =
            validate_product_background_jobs(value, "backgroundJobs", "product IR backgroundJobs")?;
        let background_job_ids = background_jobs.iter().map(|job| job.id.clone()).collect();
        let release_targets = validate_product_release_targets(
            value,
            "releaseTargets",
            "product IR releaseTargets",
            &targets,
        )?;
        let audit_keys = optional_object_keys(value, "audit")?;
        Ok(ProductIr {
            app_id,
            app_name,
            targets,
            desktop_surface_ir,
            mobile_surface_ir,
            capabilities,
            permissions,
            domain_object_ids,
            state_snapshot_schema,
            state_command,
            state_status_command,
            persistence_truth,
            persistence_root_ids,
            background_jobs,
            background_job_ids,
            release_targets,
            audit_keys,
            action_contracts,
            actions: action_values.len(),
            action_ids,
        })
    }

    pub fn validate_surface_ir_value(value: &Value) -> ContractResult<SurfaceIr> {
        let schema = value_string(value, "version")
            .ok_or_else(|| ContractError::new("surface IR version required"))?;
        if !matches!(
            schema.as_str(),
            DESKTOP_SURFACE_IR_SCHEMA | MOBILE_SURFACE_IR_SCHEMA
        ) {
            return Err(ContractError::new("surface IR version invalid"));
        }
        expect_value_string(value, "format", "json")?;
        let product = value_string(value, "product")
            .filter(|id| valid_slug(id))
            .ok_or_else(|| ContractError::new("surface IR product must be a lowercase slug"))?;
        let target = value_string(value, "target")
            .filter(|target| !target.is_empty())
            .ok_or_else(|| ContractError::new("surface IR target required"))?;
        let action_ids = surface_action_ids(value)?;
        let mut roles = Vec::new();
        if schema == DESKTOP_SURFACE_IR_SCHEMA {
            if !matches!(target.as_str(), "desktop" | "macos" | "linux") {
                return Err(ContractError::new("desktop surface IR target invalid"));
            }
            let window = value_object(value, "window")?;
            collect_surface_roles(window, &mut roles);
        } else {
            if !matches!(target.as_str(), "mobile" | "ios" | "android") {
                return Err(ContractError::new("mobile surface IR target invalid"));
            }
            for screen in value_array(value, "screens")? {
                if let Ok(node) = value_object(screen, "node") {
                    collect_surface_roles(node, &mut roles);
                }
            }
        }
        roles.sort();
        roles.dedup();
        Ok(SurfaceIr {
            schema,
            product,
            target,
            action_ids,
            roles,
        })
    }

    pub fn project_surface_from_product_text(
        product: &str,
        target: &str,
    ) -> ContractResult<String> {
        let value: Value = serde_json::from_str(product)
            .map_err(|error| ContractError::new(format!("invalid JSON: {error}")))?;
        let product = validate_product_ir_value(&value)?;
        let surface = project_surface_from_product(&product, target)?;
        serde_json::to_string_pretty(&surface)
            .map(|json| format!("{json}\n"))
            .map_err(|error| ContractError::new(format!("could not serialize surface IR: {error}")))
    }

    pub fn project_surface_from_product(
        product: &ProductIr,
        target: &str,
    ) -> ContractResult<Value> {
        if !product
            .targets
            .iter()
            .any(|candidate| candidate.as_str() == target)
        {
            return Err(ContractError::new(format!(
                "product IR does not declare target: {target}"
            )));
        }
        let actions = string_vec_value(&product.action_ids);
        let surface = if is_desktop_target(target) {
            serde_json::json!({
                "version": DESKTOP_SURFACE_IR_SCHEMA,
                "format": "json",
                "product": product.app_id,
                "target": target,
                "actions": actions,
                "window": {
                    "id": "window.main",
                    "type": "Window",
                    "title": product.app_name,
                    "role": "native-product-root",
                    "child": {
                        "id": "split.main",
                        "type": "SplitPane",
                        "role": "left-list-detail",
                        "children": [
                            {
                                "id": "list.primary",
                                "type": "TreeList",
                                "role": "product-navigation"
                            },
                            {
                                "id": "detail.primary",
                                "type": "Detail",
                                "role": "product-detail"
                            }
                        ]
                    }
                }
            })
        } else if is_mobile_target(target) {
            serde_json::json!({
                "version": MOBILE_SURFACE_IR_SCHEMA,
                "format": "json",
                "product": product.app_id,
                "target": target,
                "actions": actions,
                "screens": [
                    {
                        "id": "overview",
                        "title": product.app_name,
                        "node": {
                            "id": "screen.overview",
                            "type": "NavigationStack",
                            "role": "status-overview"
                        }
                    },
                    {
                        "id": "detail",
                        "title": "Detail",
                        "node": {
                            "id": "screen.detail",
                            "type": "Screen",
                            "role": "focused-action-detail"
                        }
                    }
                ]
            })
        } else {
            return Err(ContractError::new(
                "target must be macos, linux, ios, or android",
            ));
        };
        validate_surface_ir_value(&surface)?;
        Ok(surface)
    }

    pub fn generated_runtime_metadata(
        product: &ProductIr,
        runtime: &RuntimeBridge,
        target: &str,
        surface: &SurfaceIr,
    ) -> ContractResult<String> {
        let release_target = release_target_for_target(product, target).ok_or_else(|| {
            ContractError::new(format!(
                "product IR release target missing for target: {target}"
            ))
        })?;
        let mut object = serde_json::Map::new();
        object.insert(
            "version".to_string(),
            Value::String(GENERATED_RUNTIME_SCHEMA.to_string()),
        );
        object.insert("app".to_string(), Value::String(runtime.app_id.clone()));
        object.insert("target".to_string(), Value::String(target.to_string()));
        object.insert(
            "productIr".to_string(),
            Value::String(runtime.product_ir.clone()),
        );
        object.insert(
            "runtimeManifest".to_string(),
            Value::String(runtime.runtime_manifest.clone()),
        );
        object.insert(
            "sourceSurfaceIr".to_string(),
            Value::String(runtime.source_surface_ir.clone()),
        );
        if let Some(legacy_native_desktop_ir) = &runtime.legacy_native_desktop_ir {
            object.insert(
                "legacyNativeDesktopIr".to_string(),
                Value::String(legacy_native_desktop_ir.clone()),
            );
        }
        object.insert(
            "compatibilityWizardryAppsShellFirstStillSupported".to_string(),
            Value::Bool(
                runtime
                    .compatibility
                    .wizardry_apps_shell_first_still_supported,
            ),
        );
        object.insert(
            "compatibilityTheurgyRequiredForLegacyWizardryApps".to_string(),
            Value::Bool(
                runtime
                    .compatibility
                    .theurgy_required_for_legacy_wizardry_apps,
            ),
        );
        object.insert(
            "productTargets".to_string(),
            string_vec_value(&product.targets),
        );
        object.insert(
            "productActions".to_string(),
            string_vec_value(&product.action_ids),
        );
        object.insert(
            "productActionContracts".to_string(),
            action_contracts_value(&product.action_contracts),
        );
        object.insert(
            "productCapabilities".to_string(),
            string_vec_value(&product.capabilities),
        );
        object.insert(
            "productPermissions".to_string(),
            string_vec_value(&product.permissions),
        );
        object.insert(
            "productDomainObjects".to_string(),
            string_vec_value(&product.domain_object_ids),
        );
        object.insert(
            "productStateSnapshotSchema".to_string(),
            Value::String(product.state_snapshot_schema.clone()),
        );
        object.insert(
            "productPersistenceRoots".to_string(),
            string_vec_value(&product.persistence_root_ids),
        );
        object.insert(
            "productPersistenceTruth".to_string(),
            Value::String(product.persistence_truth.clone()),
        );
        object.insert(
            "adapterRuntimeTransport".to_string(),
            Value::String(adapter_runtime_transport(target).to_string()),
        );
        object.insert(
            "productBackgroundJobs".to_string(),
            string_vec_value(&product.background_job_ids),
        );
        object.insert(
            "productReleaseTargets".to_string(),
            string_vec_value(&release_target_ids(product)),
        );
        object.insert(
            "targetReleaseTarget".to_string(),
            Value::String(release_target.id.clone()),
        );
        object.insert(
            "targetReleaseArtifact".to_string(),
            Value::String(release_target.artifact.clone()),
        );
        object.insert(
            "productAuditKeys".to_string(),
            string_vec_value(&product.audit_keys),
        );
        object.insert(
            "protocol".to_string(),
            Value::String(runtime.protocol.clone()),
        );
        object.insert(
            "runtimeStatusSchema".to_string(),
            Value::String(RUNTIME_STATUS_SCHEMA.to_string()),
        );
        object.insert(
            "runtimeActionRequestSchema".to_string(),
            Value::String(RUNTIME_ACTION_REQUEST_SCHEMA.to_string()),
        );
        object.insert(
            "runtimeActionResultSchema".to_string(),
            Value::String(RUNTIME_ACTION_RESULT_SCHEMA.to_string()),
        );
        object.insert(
            "operationStatusSchema".to_string(),
            Value::String(OPERATION_STATUS_SCHEMA.to_string()),
        );
        object.insert(
            "operationHistorySchema".to_string(),
            Value::String(OPERATION_HISTORY_SCHEMA.to_string()),
        );
        object.insert(
            "stateCommand".to_string(),
            string_vec_value(&runtime.state_command),
        );
        if !runtime.status_command.is_empty() {
            object.insert(
                "statusCommand".to_string(),
                string_vec_value(&runtime.status_command),
            );
        }
        if !runtime.operation_status_command.is_empty() {
            object.insert(
                "operationStatusCommand".to_string(),
                string_vec_value(&runtime.operation_status_command),
            );
        }
        object.insert(
            "subscribeStatusCommand".to_string(),
            string_vec_value(&effective_subscribe_status_command(runtime)),
        );
        object.insert(
            "actionCommand".to_string(),
            string_vec_value(&runtime.action_command),
        );
        if !runtime.history_command.is_empty() {
            object.insert(
                "historyCommand".to_string(),
                string_vec_value(&runtime.history_command),
            );
        }
        if !runtime.daemon_command.is_empty() {
            object.insert(
                "daemonCommand".to_string(),
                string_vec_value(&runtime.daemon_command),
            );
        }
        object.insert(
            "surface".to_string(),
            Value::String("theurgy-surface.json".to_string()),
        );
        object.insert(
            "surfaceSchema".to_string(),
            Value::String(surface.schema.clone()),
        );
        object.insert(
            "surfaceTarget".to_string(),
            Value::String(surface.target.clone()),
        );
        object.insert(
            "surfaceActions".to_string(),
            string_vec_value(&surface.action_ids),
        );
        object.insert(
            "surfaceActionContracts".to_string(),
            action_contracts_value(&surface_action_contracts(product, surface)),
        );
        object.insert("surfaceRoles".to_string(), string_vec_value(&surface.roles));
        let metadata = Value::Object(object);
        validate_generated_runtime_value(&metadata)?;
        serde_json::to_string_pretty(&metadata)
            .map(|json| format!("{json}\n"))
            .map_err(|error| {
                ContractError::new(format!("could not serialize generated runtime: {error}"))
            })
    }

    pub fn validate_native_compile_contract(
        product: &ProductIr,
        surface: &SurfaceIr,
        target: &str,
    ) -> ContractResult<ReleaseTarget> {
        if !product
            .targets
            .iter()
            .any(|candidate| candidate.as_str() == target)
        {
            return Err(ContractError::new(format!(
                "product IR does not declare target: {target}"
            )));
        }
        let release_target = release_target_for_target(product, target).ok_or_else(|| {
            ContractError::new(format!(
                "product IR release target missing for target: {target}"
            ))
        })?;
        if surface.product != product.app_id {
            return Err(ContractError::new(
                "surface IR product does not match product IR app",
            ));
        }
        let expected_surface_target = surface_family_for_target(target)
            .ok_or_else(|| ContractError::new("unsupported target"))?;
        let expected_surface_schema = surface_schema_for_target(target)
            .ok_or_else(|| ContractError::new("unsupported target"))?;
        if release_target.surface != expected_surface_target {
            return Err(ContractError::new(format!(
                "product IR release target surface for {target} must be {expected_surface_target}"
            )));
        }
        if surface.schema != expected_surface_schema {
            return Err(ContractError::new(format!(
                "surface IR schema for {target} must be {expected_surface_schema}"
            )));
        }
        if surface.target != target && surface.target != expected_surface_target {
            return Err(ContractError::new(format!(
                "surface IR target must be {target} or {expected_surface_target}"
            )));
        }
        for action_id in &surface.action_ids {
            if !product
                .action_ids
                .iter()
                .any(|product_action| product_action == action_id)
            {
                return Err(ContractError::new(format!(
                    "surface IR action not declared in Product IR: {action_id}"
                )));
            }
        }
        Ok(release_target.clone())
    }

    pub fn macos_adapter_files(
        product: &ProductIr,
        surface: &SurfaceIr,
        runtime: &RuntimeBridge,
    ) -> Vec<GeneratedNativeFile> {
        vec![
            GeneratedNativeFile {
                path: "Package.swift".to_string(),
                contents: format!(
                    "// Generated by theurgy-runtime compile-native.\n// swift-tools-version: 6.0\nimport PackageDescription\n\nlet package = Package(name: \"{}\", platforms: [.macOS(.v13)], products: [.executable(name: \"{}\", targets: [\"App\"])], targets: [.executableTarget(name: \"App\", path: \"Sources/App\")])\n",
                    product.app_id, product.app_id
                ),
            },
            GeneratedNativeFile {
                path: "Sources/App/App.swift".to_string(),
                contents: macos_adapter_source(product, surface, runtime),
            },
        ]
    }

    pub fn linux_adapter_files(
        product: &ProductIr,
        surface: &SurfaceIr,
        runtime: &RuntimeBridge,
    ) -> Vec<GeneratedNativeFile> {
        vec![
            GeneratedNativeFile {
                path: "meson.build".to_string(),
                contents: format!(
                    "project('{}', 'c', version: '0.1.0')\ngtk = dependency('gtk4')\njson_glib = dependency('json-glib-1.0')\nexecutable('{}', 'src/main.c', dependencies: [gtk, json_glib], install: true)\n",
                    product.app_id, product.app_id
                ),
            },
            GeneratedNativeFile {
                path: "src/main.c".to_string(),
                contents: linux_adapter_source(product, surface, runtime),
            },
        ]
    }

    pub fn validate_action_ir_value(value: &Value) -> ContractResult<ActionIr> {
        expect_value_string(value, "version", ACTION_IR_SCHEMA)?;
        let action_values = value_array(value, "actions")?;
        if action_values.is_empty() {
            return Err(ContractError::new("action IR actions required"));
        }
        let mut action_ids = Vec::new();
        let mut action_contracts = Vec::new();
        for action in action_values {
            let contract = validate_product_action_contract(action)?;
            action_ids.push(contract.id.clone());
            action_contracts.push(contract);
        }
        Ok(ActionIr {
            actions: action_values.len(),
            action_ids,
            action_contracts,
        })
    }

    pub fn validate_product_action_contract(
        action: &Value,
    ) -> ContractResult<ProductActionContract> {
        let id = value_string(action, "id")
            .filter(|id| valid_action_id(id))
            .ok_or_else(|| ContractError::new("product IR action.id must be a stable action id"))?;
        let label = value_string(action, "label")
            .filter(|label| !label.is_empty())
            .ok_or_else(|| ContractError::new("product IR action.label required"))?;
        let input = value_object(action, "input")
            .map_err(|_| ContractError::new("product IR action.input object required"))?;
        let output = value_object(action, "output")
            .map_err(|_| ContractError::new("product IR action.output object required"))?;
        let failure = value_object(action, "failure")
            .map_err(|_| ContractError::new("product IR action.failure object required"))?;
        let effect = value_string(action, "effect")
            .ok_or_else(|| ContractError::new("product IR action.effect invalid"))?;
        if !matches!(
            effect.as_str(),
            "read" | "write" | "background" | "external" | "release"
        ) {
            return Err(ContractError::new("product IR action.effect invalid"));
        }
        for key in ["safe", "mutating", "longRunning", "privileged"] {
            value_bool(action, key).ok_or_else(|| {
                ContractError::new(format!("product IR action.{key} boolean required"))
            })?;
        }
        let command = optional_string_array(action, "command", "product IR action.command")?;
        if action.get("command").is_some() && command.is_empty() {
            return Err(ContractError::new("product IR action.command required"));
        }
        Ok(ProductActionContract {
            id,
            label,
            effect,
            safe: value_bool(action, "safe").unwrap_or(false),
            mutating: value_bool(action, "mutating").unwrap_or(false),
            long_running: value_bool(action, "longRunning").unwrap_or(false),
            privileged: value_bool(action, "privileged").unwrap_or(false),
            command,
            input_keys: object_keys(input),
            output_keys: object_keys(output),
            failure_keys: object_keys(failure),
            input_shape: object_shape(input, "product IR action.input")?,
            output_shape: object_shape(output, "product IR action.output")?,
            failure_shape: object_shape(failure, "product IR action.failure")?,
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

    fn product_surface_paths(value: &Value) -> ContractResult<(Option<String>, Option<String>)> {
        let Some(surfaces) = value.get("surfaces") else {
            return Ok((None, None));
        };
        if !surfaces.is_object() {
            return Err(ContractError::new("product IR surfaces must be an object"));
        }
        Ok((
            optional_nonempty_string(surfaces, "desktop", "product IR surfaces.desktop")?,
            optional_nonempty_string(surfaces, "mobile", "product IR surfaces.mobile")?,
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

    fn runtime_action_contract<'a>(
        action_id: &str,
        contracts: &'a [ProductActionContract],
    ) -> ContractResult<&'a ProductActionContract> {
        contracts
            .iter()
            .find(|contract| contract.id == action_id)
            .ok_or_else(|| {
                ContractError::new(format!(
                    "runtime action not declared in Product IR: {action_id}"
                ))
            })
    }

    fn validate_shape_object(
        shape: &BTreeMap<String, String>,
        object: &serde_json::Map<String, Value>,
        label: &str,
        action_id: &str,
    ) -> ContractResult<()> {
        for (key, descriptor) in shape {
            if descriptor.ends_with('?') && !object.contains_key(key) {
                continue;
            }
            let Some(value) = object.get(key) else {
                continue;
            };
            if !value_matches_shape(value, descriptor) {
                return Err(ContractError::new(format!(
                    "{label} type mismatch for {action_id}.{key}: expected {descriptor}"
                )));
            }
        }
        Ok(())
    }

    fn value_matches_shape(value: &Value, descriptor: &str) -> bool {
        if value.is_null() {
            return descriptor.ends_with('?');
        }
        let required = descriptor.strip_suffix('?').unwrap_or(descriptor);
        if required.contains('|') {
            return value
                .as_str()
                .map(|actual| required.split('|').any(|variant| variant == actual))
                .unwrap_or(false);
        }
        match required {
            "string" => value.is_string(),
            "boolean" => value.is_boolean(),
            "number" => value.is_number(),
            "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
            "array" => value.is_array(),
            "object" => value.is_object(),
            "json" => true,
            _ => false,
        }
    }

    fn release_target_ids(product: &ProductIr) -> Vec<String> {
        product
            .release_targets
            .iter()
            .map(|release_target| release_target.id.clone())
            .collect()
    }

    fn release_target_for_target<'a>(
        product: &'a ProductIr,
        target: &str,
    ) -> Option<&'a ReleaseTarget> {
        product
            .release_targets
            .iter()
            .find(|release_target| release_target.target == target)
    }

    fn effective_subscribe_status_command(runtime: &RuntimeBridge) -> Vec<String> {
        if !runtime.subscribe_status_command.is_empty() {
            return runtime.subscribe_status_command.clone();
        }
        if !runtime.status_command.is_empty() {
            return runtime.status_command.clone();
        }
        vec![
            "theurgy-runtime".to_string(),
            "subscribe-status".to_string(),
            "--manifest".to_string(),
            "theurgy-runtime.json".to_string(),
            "--once".to_string(),
        ]
    }

    fn surface_action_contracts(
        product: &ProductIr,
        surface: &SurfaceIr,
    ) -> Vec<ProductActionContract> {
        surface
            .action_ids
            .iter()
            .filter_map(|action_id| {
                product
                    .action_contracts
                    .iter()
                    .find(|contract| &contract.id == action_id)
                    .cloned()
            })
            .collect()
    }

    fn action_contracts_value(contracts: &[ProductActionContract]) -> Value {
        Value::Array(
            contracts
                .iter()
                .map(|contract| {
                    let mut object = serde_json::Map::new();
                    object.insert("id".to_string(), Value::String(contract.id.clone()));
                    object.insert("label".to_string(), Value::String(contract.label.clone()));
                    object.insert("effect".to_string(), Value::String(contract.effect.clone()));
                    object.insert("safe".to_string(), Value::Bool(contract.safe));
                    object.insert("mutating".to_string(), Value::Bool(contract.mutating));
                    object.insert(
                        "longRunning".to_string(),
                        Value::Bool(contract.long_running),
                    );
                    object.insert("privileged".to_string(), Value::Bool(contract.privileged));
                    object.insert(
                        "inputKeys".to_string(),
                        string_vec_value(&contract.input_keys),
                    );
                    object.insert(
                        "outputKeys".to_string(),
                        string_vec_value(&contract.output_keys),
                    );
                    object.insert(
                        "failureKeys".to_string(),
                        string_vec_value(&contract.failure_keys),
                    );
                    object.insert("inputShape".to_string(), shape_value(&contract.input_shape));
                    object.insert(
                        "outputShape".to_string(),
                        shape_value(&contract.output_shape),
                    );
                    object.insert(
                        "failureShape".to_string(),
                        shape_value(&contract.failure_shape),
                    );
                    Value::Object(object)
                })
                .collect(),
        )
    }

    fn shape_value(shape: &BTreeMap<String, String>) -> Value {
        let mut object = serde_json::Map::new();
        for (key, type_name) in shape {
            object.insert(key.clone(), Value::String(type_name.clone()));
        }
        Value::Object(object)
    }

    fn linux_adapter_source(
        product: &ProductIr,
        surface: &SurfaceIr,
        runtime: &RuntimeBridge,
    ) -> String {
        let action_contracts = surface_action_contracts(product, surface);
        let action_contracts_json =
            serde_json::to_string(&action_contracts_value(&action_contracts)).unwrap_or_default();
        let subscribe_status_command = effective_subscribe_status_command(runtime);
        let subscribe_status_executable = subscribe_status_command
            .first()
            .cloned()
            .unwrap_or_default();
        let subscribe_status_arguments = c_argv_tail_literal(&subscribe_status_command[1..]);
        let executable = runtime.state_command.first().cloned().unwrap_or_default();
        let arguments = c_argv_tail_literal(&runtime.state_command[1..]);
        let status_executable = runtime.status_command.first().cloned().unwrap_or_default();
        let status_tail = runtime.status_command.get(1..).unwrap_or(&[]);
        let status_arguments = c_argv_tail_literal(status_tail);
        let action_executable = runtime.action_command.first().cloned().unwrap_or_default();
        let action_tail = runtime.action_command.get(1..).unwrap_or(&[]);
        let action_arguments = c_argv_tail_literal(action_tail);
        let default_action_id = action_contracts
            .first()
            .map(|contract| contract.id.clone())
            .unwrap_or_default();
        let action_text = runtime.action_command.join(" ");
        let subscribe_status_text = subscribe_status_command.join(" ");
        let operation_status_text = runtime.operation_status_command.join(" ");
        let history_text = runtime.history_command.join(" ");
        let daemon_text = runtime.daemon_command.join(" ");
        let template = r#"/* Generated by theurgy-runtime compile-native.
 * Runtime: theurgy-runtime.json
 * Surface: theurgy-surface.json
 */
#include <gtk/gtk.h>
#include <json-glib/json-glib.h>

static const char *surface_action_contracts_json = "__ACTION_CONTRACTS_JSON__";

static char *resolve_executable(const char *name) {
  const char *override = g_getenv("THEURGY_RUNTIME_EXECUTABLE");
  if (override != NULL && g_file_test(override, G_FILE_TEST_IS_EXECUTABLE)) {
    return g_strdup(override);
  }
  g_autofree char *self_path = g_file_read_link("/proc/self/exe", NULL);
  if (self_path != NULL) {
    g_autofree char *self_dir = g_path_get_dirname(self_path);
    g_autofree char *beside_exe = g_build_filename(self_dir, name, NULL);
    if (g_file_test(beside_exe, G_FILE_TEST_IS_EXECUTABLE)) {
      return g_strdup(beside_exe);
    }
    g_autofree char *libexec = g_build_filename(self_dir, "..", "libexec", name, NULL);
    if (g_file_test(libexec, G_FILE_TEST_IS_EXECUTABLE)) {
      return g_canonicalize_filename(libexec, NULL);
    }
  }
  return g_strdup(name);
}

static char *run_runtime_command(const char *argv[]);

static char *load_runtime_state(void) {
  g_autofree char *runtime = resolve_executable("__STATE_EXECUTABLE__");
  const char *argv[] = { runtime, __STATE_ARGUMENTS__ NULL };
  return run_runtime_command(argv);
}

static char *load_runtime_status(void) {
  g_autofree char *runtime = resolve_executable("__STATUS_EXECUTABLE__");
  const char *argv[] = { runtime, __STATUS_ARGUMENTS__ NULL };
  return run_runtime_command(argv);
}

static char *subscribe_runtime_status(void) {
  g_autofree char *runtime = resolve_executable("__SUBSCRIBE_STATUS_EXECUTABLE__");
  const char *argv[] = { runtime, __SUBSCRIBE_STATUS_ARGUMENTS__ NULL };
  return run_runtime_command(argv);
}

static char *run_default_action(void) {
  g_autofree char *runtime = resolve_executable("__ACTION_EXECUTABLE__");
  const char *argv[] = { runtime, __ACTION_ARGUMENTS__ "__DEFAULT_ACTION_ID__", "{}", NULL };
  return run_runtime_command(argv);
}

static char *run_runtime_command(const char *argv[]) {
  g_autoptr(GError) error = NULL;
  g_autoptr(GSubprocess) process = g_subprocess_newv(
      argv,
      G_SUBPROCESS_FLAGS_STDOUT_PIPE | G_SUBPROCESS_FLAGS_STDERR_PIPE,
      &error);
  if (process == NULL) {
    return g_strdup(error != NULL ? error->message : "could not start runtime command");
  }
  char *stdout_text = NULL;
  char *stderr_text = NULL;
  if (!g_subprocess_communicate_utf8(process, NULL, NULL, &stdout_text, &stderr_text, &error)) {
    g_free(stdout_text);
    g_free(stderr_text);
    return g_strdup(error != NULL ? error->message : "runtime command failed");
  }
  if (!g_subprocess_get_successful(process)) {
    g_free(stdout_text);
    return stderr_text != NULL ? stderr_text : g_strdup("runtime command exited unsuccessfully");
  }
  g_free(stderr_text);
  return stdout_text;
}

static void refresh_state(GtkButton *button, gpointer user_data) {
  (void)button;
  GtkLabel *label = GTK_LABEL(user_data);
  g_autofree char *state = load_runtime_state();
  gtk_label_set_text(label, state);
}

static void refresh_status(GtkButton *button, gpointer user_data) {
  (void)button;
  GtkLabel *label = GTK_LABEL(user_data);
  g_autofree char *state = load_runtime_status();
  gtk_label_set_text(label, state);
}

static void subscribe_status(GtkButton *button, gpointer user_data) {
  (void)button;
  GtkLabel *label = GTK_LABEL(user_data);
  g_autofree char *state = subscribe_runtime_status();
  gtk_label_set_text(label, state);
}

static void run_action(GtkButton *button, gpointer user_data) {
  (void)button;
  GtkLabel *label = GTK_LABEL(user_data);
  g_autofree char *state = run_default_action();
  gtk_label_set_text(label, state);
}

static void activate(GtkApplication *app, gpointer user_data) {
  (void)user_data;
  GtkWidget *window = gtk_application_window_new(app);
  GtkWidget *box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 8);
  GtkWidget *contract = gtk_label_new("State: __STATE_COMMAND_TEXT__\nStatus: __STATUS_COMMAND_TEXT__\nSubscribe: __SUBSCRIBE_STATUS_COMMAND_TEXT__\nOperation status: __OPERATION_STATUS_COMMAND_TEXT__\nAction: __ACTION_COMMAND_TEXT__\nHistory: __HISTORY_COMMAND_TEXT__\nDaemon: __DAEMON_COMMAND_TEXT__\nSurface action contracts: __ACTION_CONTRACT_IDS__");
  GtkWidget *button_box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 8);
  GtkWidget *button = gtk_button_new_with_label("State");
  GtkWidget *status_button = gtk_button_new_with_label("Status");
  GtkWidget *subscribe_button = gtk_button_new_with_label("Subscribe");
  GtkWidget *action_button = gtk_button_new_with_label("Action");
  GtkWidget *label = gtk_label_new("Runtime state not loaded.");
  gtk_window_set_title(GTK_WINDOW(window), "__APP_NAME__");
  gtk_window_set_default_size(GTK_WINDOW(window), 960, 640);
  gtk_label_set_xalign(GTK_LABEL(contract), 0.0);
  gtk_label_set_wrap(GTK_LABEL(contract), TRUE);
  gtk_label_set_xalign(GTK_LABEL(label), 0.0);
  gtk_label_set_wrap(GTK_LABEL(label), TRUE);
  gtk_box_append(GTK_BOX(button_box), button);
  gtk_box_append(GTK_BOX(button_box), status_button);
  gtk_box_append(GTK_BOX(button_box), subscribe_button);
  gtk_box_append(GTK_BOX(button_box), action_button);
  gtk_box_append(GTK_BOX(box), contract);
  gtk_box_append(GTK_BOX(box), button_box);
  gtk_box_append(GTK_BOX(box), label);
  gtk_window_set_child(GTK_WINDOW(window), box);
  g_signal_connect(button, "clicked", G_CALLBACK(refresh_state), label);
  g_signal_connect(status_button, "clicked", G_CALLBACK(refresh_status), label);
  g_signal_connect(subscribe_button, "clicked", G_CALLBACK(subscribe_status), label);
  g_signal_connect(action_button, "clicked", G_CALLBACK(run_action), label);
  refresh_state(GTK_BUTTON(button), label);
  gtk_window_present(GTK_WINDOW(window));
}

int main(int argc, char **argv) {
  GtkApplication *app = gtk_application_new("app.theurgy.__APP_ID__", G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
  int status = g_application_run(G_APPLICATION(app), argc, argv);
  g_object_unref(app);
  return status;
}
"#;
        template
            .replace("__APP_NAME__", &c_escape(&product.app_name))
            .replace("__APP_ID__", &c_escape(&product.app_id.replace('-', "_")))
            .replace("__STATE_EXECUTABLE__", &c_escape(&executable))
            .replace("__STATE_ARGUMENTS__", &arguments)
            .replace("__STATUS_EXECUTABLE__", &c_escape(&status_executable))
            .replace("__STATUS_ARGUMENTS__", &status_arguments)
            .replace(
                "__SUBSCRIBE_STATUS_EXECUTABLE__",
                &c_escape(&subscribe_status_executable),
            )
            .replace(
                "__SUBSCRIBE_STATUS_ARGUMENTS__",
                &subscribe_status_arguments,
            )
            .replace("__ACTION_EXECUTABLE__", &c_escape(&action_executable))
            .replace("__ACTION_ARGUMENTS__", &action_arguments)
            .replace("__DEFAULT_ACTION_ID__", &c_escape(&default_action_id))
            .replace(
                "__STATE_COMMAND_TEXT__",
                &c_escape(&runtime.state_command.join(" ")),
            )
            .replace(
                "__STATUS_COMMAND_TEXT__",
                &c_escape(&runtime.status_command.join(" ")),
            )
            .replace(
                "__SUBSCRIBE_STATUS_COMMAND_TEXT__",
                &c_escape(&subscribe_status_text),
            )
            .replace(
                "__OPERATION_STATUS_COMMAND_TEXT__",
                &c_escape(&operation_status_text),
            )
            .replace("__ACTION_COMMAND_TEXT__", &c_escape(&action_text))
            .replace("__HISTORY_COMMAND_TEXT__", &c_escape(&history_text))
            .replace("__DAEMON_COMMAND_TEXT__", &c_escape(&daemon_text))
            .replace(
                "__ACTION_CONTRACTS_JSON__",
                &c_escape(&action_contracts_json),
            )
            .replace(
                "__ACTION_CONTRACT_IDS__",
                &c_escape(
                    &action_contracts
                        .iter()
                        .map(|contract| contract.id.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                ),
            )
    }

    fn c_argv_tail_literal(values: &[String]) -> String {
        if values.is_empty() {
            String::new()
        } else {
            format!(
                "{}, ",
                values
                    .iter()
                    .map(|value| format!("\"{}\"", c_escape(value)))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }

    fn c_escape(value: &str) -> String {
        let mut escaped = String::new();
        for character in value.chars() {
            match character {
                '\\' => escaped.push_str("\\\\"),
                '"' => escaped.push_str("\\\""),
                '\n' => escaped.push_str("\\n"),
                '\r' => escaped.push_str("\\r"),
                '\t' => escaped.push_str("\\t"),
                _ => escaped.push(character),
            }
        }
        escaped
    }

    fn macos_adapter_source(
        product: &ProductIr,
        surface: &SurfaceIr,
        runtime: &RuntimeBridge,
    ) -> String {
        let action_contracts = surface_action_contracts(product, surface);
        let subscribe_status_command = effective_subscribe_status_command(runtime);
        let default_action_id = action_contracts
            .first()
            .map(|contract| contract.id.clone())
            .unwrap_or_default();
        let template = r#"// Generated by theurgy-runtime compile-native.
// Runtime: theurgy-runtime.json
// Surface: theurgy-surface.json
import Foundation
import SwiftUI

struct ProductActionContract {
  let id: String
  let label: String
  let effect: String
  let safe: Bool
  let mutating: Bool
  let longRunning: Bool
  let privileged: Bool
  let inputKeys: [String]
  let outputKeys: [String]
  let failureKeys: [String]
  let inputShape: [String: String]
  let outputShape: [String: String]
  let failureShape: [String: String]
}

let runtimeStateCommand = __STATE_COMMAND__
let runtimeStatusCommand = __STATUS_COMMAND__
let runtimeSubscribeStatusCommand = __SUBSCRIBE_STATUS_COMMAND__
let runtimeOperationStatusCommand = __OPERATION_STATUS_COMMAND__
let runtimeActionCommand = __ACTION_COMMAND__
let runtimeHistoryCommand = __HISTORY_COMMAND__
let runtimeDaemonCommand = __DAEMON_COMMAND__
let actionContracts = __ACTION_CONTRACTS__
let defaultActionId = "__DEFAULT_ACTION_ID__"

func command(for action: ProductActionContract, json: String) -> [String] {
  runtimeActionCommand + [action.id, json]
}

struct RuntimeStateView: View {
  @State private var status = "Runtime state not loaded."

  var body: some View {
    VStack(alignment: .leading, spacing: 12) {
      Text("__APP_NAME__")
        .font(.title2)
      VStack(alignment: .leading, spacing: 4) {
        Text("State: \(runtimeStateCommand.joined(separator: " "))")
        Text("Status: \(runtimeStatusCommand.joined(separator: " "))")
        Text("Subscribe: \(runtimeSubscribeStatusCommand.joined(separator: " "))")
        Text("Operation status: \(runtimeOperationStatusCommand.joined(separator: " "))")
        Text("Action: \(runtimeActionCommand.joined(separator: " "))")
        Text("History: \(runtimeHistoryCommand.joined(separator: " "))")
        Text("Daemon: \(runtimeDaemonCommand.joined(separator: " "))")
        Text("Surface actions: \(actionContracts.map { $0.id }.joined(separator: ", "))")
      }
      .font(.system(.caption, design: .monospaced))
      HStack {
        Button("State") {
          status = runRuntimeCommand(runtimeStateCommand)
        }
        if !runtimeStatusCommand.isEmpty {
          Button("Status") {
            status = runRuntimeCommand(runtimeStatusCommand)
          }
        }
        Button("Subscribe") {
          status = runRuntimeCommand(runtimeSubscribeStatusCommand)
        }
        if !runtimeActionCommand.isEmpty && !defaultActionId.isEmpty {
          Button("Action") {
            status = runRuntimeCommand(runtimeActionCommand + [defaultActionId, "{}"])
          }
        }
        if !runtimeHistoryCommand.isEmpty {
          Button("History") {
            status = runRuntimeCommand(runtimeHistoryCommand + ["default", "20"])
          }
        }
      }
      VStack(alignment: .leading, spacing: 6) {
        ForEach(actionContracts, id: \.id) { action in
          Button(action.label) {
            status = runRuntimeCommand(command(for: action, json: "{}"))
          }
          Text(command(for: action, json: "{}").joined(separator: " "))
            .font(.system(.caption2, design: .monospaced))
        }
      }
      Text(status)
        .font(.system(.body, design: .monospaced))
        .textSelection(.enabled)
    }
    .padding()
    .frame(minWidth: 960, minHeight: 640, alignment: .topLeading)
    .onAppear {
      status = runRuntimeCommand(runtimeStateCommand)
    }
  }
}

func runRuntimeCommand(_ command: [String]) -> String {
  guard let executable = command.first else {
    return "runtime command missing"
  }
  let process = Process()
  process.executableURL = resolveExecutable(executable)
  process.arguments = Array(command.dropFirst())
  let output = Pipe()
  let error = Pipe()
  process.standardOutput = output
  process.standardError = error
  do {
    try process.run()
    process.waitUntilExit()
    let data = output.fileHandleForReading.readDataToEndOfFile()
    let errorData = error.fileHandleForReading.readDataToEndOfFile()
    if process.terminationStatus != 0 {
      return String(data: errorData, encoding: .utf8) ?? "runtime command failed"
    }
    let text = String(data: data, encoding: .utf8) ?? "runtime state loaded"
    return String(text.prefix(4000))
  } catch {
    return String(describing: error)
  }
}

func resolveExecutable(_ name: String) -> URL {
  let fileManager = FileManager.default
  let candidates = [
    Bundle.main.executableURL?.deletingLastPathComponent().appendingPathComponent(name),
    Bundle.main.resourceURL?.appendingPathComponent(name),
    URL(fileURLWithPath: name)
  ].compactMap { $0 }
  return candidates.first(where: { fileManager.isExecutableFile(atPath: $0.path) }) ?? candidates.last!
}

@main
struct TheurgyNativeApp: App {
  var body: some Scene {
    WindowGroup("__APP_NAME__") {
      RuntimeStateView()
    }
  }
}
"#;
        template
            .replace("__APP_NAME__", &swift_escape(&product.app_name))
            .replace(
                "__STATE_COMMAND__",
                &swift_string_array_literal(&runtime.state_command),
            )
            .replace(
                "__STATUS_COMMAND__",
                &swift_string_array_literal(&runtime.status_command),
            )
            .replace(
                "__SUBSCRIBE_STATUS_COMMAND__",
                &swift_string_array_literal(&subscribe_status_command),
            )
            .replace(
                "__OPERATION_STATUS_COMMAND__",
                &swift_string_array_literal(&runtime.operation_status_command),
            )
            .replace(
                "__ACTION_COMMAND__",
                &swift_string_array_literal(&runtime.action_command),
            )
            .replace(
                "__HISTORY_COMMAND__",
                &swift_string_array_literal(&runtime.history_command),
            )
            .replace(
                "__DAEMON_COMMAND__",
                &swift_string_array_literal(&runtime.daemon_command),
            )
            .replace(
                "__ACTION_CONTRACTS__",
                &swift_action_contracts_literal(&action_contracts),
            )
            .replace("__DEFAULT_ACTION_ID__", &swift_escape(&default_action_id))
    }

    fn swift_string_array_literal(values: &[String]) -> String {
        let items = values
            .iter()
            .map(|value| format!("\"{}\"", swift_escape(value)))
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{items}]")
    }

    fn swift_shape_literal(shape: &BTreeMap<String, String>) -> String {
        let items = shape
            .iter()
            .map(|(key, value)| format!("\"{}\": \"{}\"", swift_escape(key), swift_escape(value)))
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{items}]")
    }

    fn swift_action_contracts_literal(contracts: &[ProductActionContract]) -> String {
        let items = contracts
            .iter()
            .map(|contract| {
                format!(
                    "ProductActionContract(id: \"{}\", label: \"{}\", effect: \"{}\", safe: {}, mutating: {}, longRunning: {}, privileged: {}, inputKeys: {}, outputKeys: {}, failureKeys: {}, inputShape: {}, outputShape: {}, failureShape: {})",
                    swift_escape(&contract.id),
                    swift_escape(&contract.label),
                    swift_escape(&contract.effect),
                    swift_bool(contract.safe),
                    swift_bool(contract.mutating),
                    swift_bool(contract.long_running),
                    swift_bool(contract.privileged),
                    swift_string_array_literal(&contract.input_keys),
                    swift_string_array_literal(&contract.output_keys),
                    swift_string_array_literal(&contract.failure_keys),
                    swift_shape_literal(&contract.input_shape),
                    swift_shape_literal(&contract.output_shape),
                    swift_shape_literal(&contract.failure_shape)
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{items}]")
    }

    fn swift_bool(value: bool) -> &'static str {
        if value {
            "true"
        } else {
            "false"
        }
    }

    fn swift_escape(value: &str) -> String {
        value.replace('\\', "\\\\").replace('"', "\\\"")
    }

    fn expect_value_string(value: &Value, key: &str, expected: &str) -> ContractResult<()> {
        match value_string(value, key) {
            Some(actual) if actual == expected => Ok(()),
            _ => Err(ContractError::new(format!("expected {key} = {expected}"))),
        }
    }

    fn expect_and_return_value_string(
        value: &Value,
        key: &str,
        expected: &str,
    ) -> ContractResult<String> {
        expect_value_string(value, key, expected)?;
        Ok(expected.to_string())
    }

    fn validate_generated_action_contract(contract: &Value) -> ContractResult<String> {
        let id = value_string(contract, "id")
            .filter(|id| valid_action_id(id))
            .ok_or_else(|| ContractError::new("generated runtime action contract id invalid"))?;
        value_string(contract, "label")
            .filter(|label| !label.is_empty())
            .ok_or_else(|| {
                ContractError::new("generated runtime action contract label required")
            })?;
        let effect = value_string(contract, "effect").ok_or_else(|| {
            ContractError::new("generated runtime action contract effect invalid")
        })?;
        if !matches!(
            effect.as_str(),
            "read" | "write" | "background" | "external" | "release"
        ) {
            return Err(ContractError::new(
                "generated runtime action contract effect invalid",
            ));
        }
        for key in ["safe", "mutating", "longRunning", "privileged"] {
            value_bool(contract, key).ok_or_else(|| {
                ContractError::new(format!(
                    "generated runtime action contract {key} boolean required"
                ))
            })?;
        }
        for (keys_key, shape_key) in [
            ("inputKeys", "inputShape"),
            ("outputKeys", "outputShape"),
            ("failureKeys", "failureShape"),
        ] {
            let keys = value_string_array(contract, keys_key).map_err(|_| {
                ContractError::new(format!(
                    "generated runtime action contract {keys_key} must be a string array"
                ))
            })?;
            let shape = value_object(contract, shape_key).map_err(|_| {
                ContractError::new(format!(
                    "generated runtime action contract {shape_key} object required"
                ))
            })?;
            object_shape(
                shape,
                &format!("generated runtime action contract {shape_key}"),
            )?;
            let mut shape_keys = object_keys(shape);
            let mut sorted_keys = keys;
            sorted_keys.sort();
            shape_keys.sort();
            if sorted_keys != shape_keys {
                return Err(ContractError::new(format!(
                    "generated runtime action contract {keys_key} must match {shape_key} keys"
                )));
            }
        }
        Ok(id)
    }

    fn surface_action_ids(value: &Value) -> ContractResult<Vec<String>> {
        let Some(actions) = value.get("actions") else {
            return Ok(Vec::new());
        };
        let Some(array) = actions.as_array() else {
            return Err(ContractError::new("surface IR actions must be an array"));
        };
        let mut action_ids = Vec::new();
        for item in array {
            let Some(action_id) = item.as_str() else {
                return Err(ContractError::new(
                    "surface IR actions must contain strings",
                ));
            };
            if !valid_action_id(action_id) {
                return Err(ContractError::new(
                    "surface IR action must be a stable action id",
                ));
            }
            action_ids.push(action_id.to_string());
        }
        Ok(action_ids)
    }

    fn collect_surface_roles(node: &Value, roles: &mut Vec<String>) {
        if let Some(role) = value_string(node, "role").filter(|role| !role.is_empty()) {
            roles.push(role);
        }
        match node {
            Value::Object(object) => {
                for child in object.values() {
                    collect_surface_roles(child, roles);
                }
            }
            Value::Array(children) => {
                for child in children {
                    collect_surface_roles(child, roles);
                }
            }
            _ => {}
        }
    }

    fn value_string(value: &Value, key: &str) -> Option<String> {
        value.get(key)?.as_str().map(String::from)
    }

    fn value_bool(value: &Value, key: &str) -> Option<bool> {
        value.get(key)?.as_bool()
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

    fn string_vec_value(values: &[String]) -> Value {
        Value::Array(values.iter().cloned().map(Value::String).collect())
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

    fn optional_object_id_array(
        value: &Value,
        key: &str,
        label: &str,
    ) -> ContractResult<Vec<String>> {
        let Some(raw) = value.get(key) else {
            return Ok(Vec::new());
        };
        let Some(array) = raw.as_array() else {
            return Err(ContractError::new(format!("{label} must be an array")));
        };
        let mut ids = Vec::new();
        for item in array {
            let Some(object) = item.as_object() else {
                return Err(ContractError::new(format!("{label} must contain objects")));
            };
            let Some(id) = object
                .get("id")
                .and_then(Value::as_str)
                .filter(|id| valid_action_id(id))
            else {
                return Err(ContractError::new(format!(
                    "{label} object.id must be stable"
                )));
            };
            ids.push(id.to_string());
        }
        Ok(ids)
    }

    fn validate_product_background_jobs(
        value: &Value,
        key: &str,
        label: &str,
    ) -> ContractResult<Vec<BackgroundJob>> {
        let Some(raw) = value.get(key) else {
            return Ok(Vec::new());
        };
        let Some(array) = raw.as_array() else {
            return Err(ContractError::new(format!("{label} must be an array")));
        };
        let mut jobs = Vec::new();
        for item in array {
            let Some(object) = item.as_object() else {
                return Err(ContractError::new(format!("{label} must contain objects")));
            };
            let id = required_stable_id(item, &format!("{label} object.id"))?;
            required_nonempty_object_string(item, "label", &format!("{label} object.label"))?;
            optional_nonempty_string(item, "state", &format!("{label} object.state"))?;
            let command = if object.get("command").is_some() {
                let command =
                    optional_string_array(item, "command", &format!("{label} object.command"))?;
                if command.is_empty() {
                    return Err(ContractError::new(format!(
                        "{label} object.command required"
                    )));
                }
                command
            } else {
                Vec::new()
            };
            jobs.push(BackgroundJob { id, command });
        }
        Ok(jobs)
    }

    fn validate_product_release_targets(
        value: &Value,
        key: &str,
        label: &str,
        app_targets: &[String],
    ) -> ContractResult<Vec<ReleaseTarget>> {
        let raw = value
            .get(key)
            .ok_or_else(|| ContractError::new(format!("{label} required")))?;
        let Some(array) = raw.as_array() else {
            return Err(ContractError::new(format!("{label} must be an array")));
        };
        if array.is_empty() {
            return Err(ContractError::new(format!("{label} required")));
        }
        let mut release_targets = Vec::new();
        let mut ids = BTreeSet::new();
        let mut target_names = BTreeSet::new();
        for item in array {
            let Some(_object) = item.as_object() else {
                return Err(ContractError::new(format!("{label} must contain objects")));
            };
            let id = required_stable_id(item, &format!("{label} object.id"))?;
            if !ids.insert(id.clone()) {
                return Err(ContractError::new(format!(
                    "{label} object.id duplicated: {id}"
                )));
            }
            let target =
                required_nonempty_object_string(item, "target", &format!("{label} object.target"))?;
            if !matches!(target.as_str(), "macos" | "linux" | "ios" | "android") {
                return Err(ContractError::new(format!(
                    "{label} object.target must be macos, linux, ios, or android"
                )));
            }
            if !app_targets.iter().any(|app_target| app_target == &target) {
                return Err(ContractError::new(format!(
                    "{label} object.target not declared in app.targets: {target}"
                )));
            }
            if !target_names.insert(target.clone()) {
                return Err(ContractError::new(format!(
                    "{label} object.target duplicated: {target}"
                )));
            }
            let surface = required_nonempty_object_string(
                item,
                "surface",
                &format!("{label} object.surface"),
            )?;
            let expected_surface = if is_desktop_target(&target) {
                "desktop"
            } else {
                "mobile"
            };
            if surface != expected_surface {
                return Err(ContractError::new(format!(
                    "{label} object.surface for {target} must be {expected_surface}"
                )));
            }
            let artifact = required_nonempty_object_string(
                item,
                "artifact",
                &format!("{label} object.artifact"),
            )?;
            release_targets.push(ReleaseTarget {
                id,
                target,
                surface,
                artifact,
            });
        }
        for app_target in app_targets {
            if !target_names.contains(app_target) {
                return Err(ContractError::new(format!(
                    "{label} missing release target for app target: {app_target}"
                )));
            }
        }
        Ok(release_targets)
    }

    fn validate_product_persistence(value: &Value) -> ContractResult<String> {
        let Some(raw) = value.get("persistence") else {
            return Ok("file-first".to_string());
        };
        let Some(_object) = raw.as_object() else {
            return Err(ContractError::new(
                "product IR persistence must be an object",
            ));
        };
        let truth = required_nonempty_object_string(raw, "truth", "product IR persistence.truth")?;
        optional_nonempty_string(raw, "database", "product IR persistence.database")?;
        optional_nonempty_string(raw, "history", "product IR persistence.history")?;
        Ok(truth)
    }

    fn required_stable_id(value: &Value, label: &str) -> ContractResult<String> {
        value
            .get("id")
            .and_then(Value::as_str)
            .filter(|id| valid_action_id(id))
            .map(String::from)
            .ok_or_else(|| ContractError::new(format!("{label} must be stable")))
    }

    fn required_nonempty_object_string(
        value: &Value,
        key: &str,
        label: &str,
    ) -> ContractResult<String> {
        value
            .get(key)
            .and_then(Value::as_str)
            .filter(|text| !text.is_empty())
            .map(String::from)
            .ok_or_else(|| ContractError::new(format!("{label} required")))
    }

    fn optional_object_keys(value: &Value, key: &str) -> ContractResult<Vec<String>> {
        let Some(raw) = value.get(key) else {
            return Ok(Vec::new());
        };
        let Some(object) = raw.as_object() else {
            return Err(ContractError::new(format!(
                "product IR {key} must be an object"
            )));
        };
        let mut keys = object.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        Ok(keys)
    }

    fn object_shape(value: &Value, label: &str) -> ContractResult<BTreeMap<String, String>> {
        let Some(object) = value.as_object() else {
            return Err(ContractError::new(format!("{label} must be an object")));
        };
        let mut shape = BTreeMap::new();
        for (key, raw) in object {
            let Some(type_name) = raw.as_str().filter(|type_name| !type_name.is_empty()) else {
                return Err(ContractError::new(format!(
                    "{label}.{key} must be a non-empty type string"
                )));
            };
            validate_shape_descriptor(type_name, &format!("{label}.{key}"))?;
            shape.insert(key.clone(), type_name.to_string());
        }
        Ok(shape)
    }

    fn validate_shape_descriptor(descriptor: &str, label: &str) -> ContractResult<()> {
        let required = descriptor.strip_suffix('?').unwrap_or(descriptor);
        if required.is_empty() {
            return Err(ContractError::new(format!("{label} shape type required")));
        }
        if required.contains('|') {
            for variant in required.split('|') {
                if variant.is_empty()
                    || !variant.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'-' | b'_')
                    })
                {
                    return Err(ContractError::new(format!(
                        "{label} enum shape contains invalid variant"
                    )));
                }
            }
            return Ok(());
        }
        if matches!(
            required,
            "string" | "boolean" | "number" | "integer" | "array" | "object" | "json"
        ) {
            return Ok(());
        }
        Err(ContractError::new(format!(
            "{label} unsupported shape type: {descriptor}"
        )))
    }

    fn object_keys(value: &Value) -> Vec<String> {
        let Some(object) = value.as_object() else {
            return Vec::new();
        };
        let mut keys = object.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        keys
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
        let manifest_value = serde_json::json!({
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
        let manifest = product_runtime::validate_runtime_manifest_value(&manifest_value).unwrap();
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
        let bridge = product_runtime::runtime_bridge_from_manifest_value(&manifest_value).unwrap();
        assert_eq!(bridge.app_id, "deployments");
        assert_eq!(bridge.product_ir, "app-blueprint/product.ir.json");
        assert_eq!(
            bridge.legacy_native_desktop_ir.as_deref(),
            Some("app-blueprint/app.ir.yaml")
        );
        assert_eq!(
            bridge.state_command,
            vec!["deployments-core".to_string(), "runtime-state".to_string()]
        );
        assert_eq!(
            bridge.action_command,
            vec!["deployments-core".to_string(), "runtime-action".to_string()]
        );
        assert_eq!(
            bridge.subscribe_status_command,
            vec!["deployments-core".to_string(), "runtime-status".to_string()]
        );
        assert!(
            bridge
                .compatibility
                .wizardry_apps_shell_first_still_supported
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

    #[test]
    fn product_runtime_loads_runtime_manifest_from_disk() {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "theurgy-runtime-manifest-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::write(
            &path,
            serde_json::json!({
                "version": product_runtime::RUNTIME_MANIFEST_SCHEMA,
                "app": "deployments",
                "productIr": "app-blueprint/product.ir.json",
                "runtime": {
                    "protocol": product_runtime::RUNTIME_ACTION_PROTOCOL,
                    "stateCommand": ["deployments-core", "runtime-state"],
                    "actionCommand": ["deployments-core", "runtime-action"]
                }
            })
            .to_string(),
        )
        .unwrap();
        let manifest = product_runtime::load_runtime_manifest(&path).unwrap();
        assert_eq!(manifest.app_id, "deployments");
        assert_eq!(manifest.product_ir, "app-blueprint/product.ir.json");
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn product_runtime_validates_action_ir_contracts() {
        let action_ir = serde_json::json!({
            "version": product_runtime::ACTION_IR_SCHEMA,
            "actions": [
                {
                    "id": "refresh_state",
                    "label": "Refresh",
                    "input": {},
                    "output": {"state": "object"},
                    "effect": "read",
                    "failure": {"error": "string?"},
                    "safe": true,
                    "mutating": false,
                    "longRunning": false,
                    "privileged": false
                },
                {
                    "id": "publish_changes",
                    "label": "Push to Production",
                    "input": {"deployment": "string"},
                    "output": {"mode": "queued|parallel"},
                    "effect": "release",
                    "failure": {},
                    "safe": false,
                    "mutating": true,
                    "longRunning": true,
                    "privileged": true,
                    "command": ["deployments-core", "runtime-action"]
                }
            ]
        });
        let summary = product_runtime::validate_action_ir_value(&action_ir).unwrap();
        assert_eq!(summary.actions, 2);
        assert_eq!(
            summary.action_ids,
            vec!["refresh_state".to_string(), "publish_changes".to_string()]
        );
        assert_eq!(
            summary.action_contracts[1]
                .input_shape
                .get("deployment")
                .map(String::as_str),
            Some("string")
        );
        assert!(summary.action_contracts[1].long_running);

        let invalid = serde_json::json!({
            "version": product_runtime::ACTION_IR_SCHEMA,
            "actions": [{
                "id": "publish_changes",
                "label": "Push to Production",
                "input": {"deployment": "String"},
                "output": {},
                "effect": "release",
                "failure": {},
                "safe": false,
                "mutating": true,
                "longRunning": true,
                "privileged": true
            }]
        });
        let error = product_runtime::validate_action_ir_value(&invalid)
            .unwrap_err()
            .to_string();
        assert_eq!(
            error,
            "product IR action.input.deployment unsupported shape type: String"
        );
    }

    #[test]
    fn product_runtime_validates_product_ir_contract() {
        let product = serde_json::json!({
            "version": product_runtime::PRODUCT_IR_SCHEMA,
            "format": "json",
            "app": {
                "id": "deployments",
                "name": "Deployments",
                "targets": ["macos", "linux", "ios", "android"],
                "capabilities": ["native-desktop", "typed-runtime-actions"],
                "permissions": ["files", "network"]
            },
            "surfaces": {
                "desktop": "app-blueprint/desktop.surface.ir.json",
                "mobile": "app-blueprint/mobile.surface.ir.json"
            },
            "domain": {
                "objects": [
                    {"id": "server", "label": "Server"},
                    {"id": "deployment", "label": "Deployment"}
                ]
            },
            "actions": [
                {
                    "id": "refresh_state",
                    "label": "Refresh",
                    "input": {},
                    "output": {"state": "object"},
                    "effect": "read",
                    "failure": {},
                    "safe": true,
                    "mutating": false,
                    "longRunning": false,
                    "privileged": false
                }
            ],
            "state": {
                "snapshotSchema": "deployments-state/v1",
                "command": ["deployments-core", "runtime-state"],
                "statusCommand": ["deployments-core", "runtime-status"],
                "roots": [{"id": "headquarters-workspace", "kind": "xdg-state"}]
            },
            "persistence": {
                "truth": "headquarters-compatible-file-first"
            },
            "backgroundJobs": [
                {
                    "id": "server-scoped-check-and-push-queue",
                    "label": "Server Queue",
                    "state": "server.queue_mode",
                    "command": ["deployments-daemon"]
                }
            ],
            "releaseTargets": [
                {"id": "macos-native", "target": "macos", "surface": "desktop", "artifact": "generated/macos"},
                {"id": "linux-native", "target": "linux", "surface": "desktop", "artifact": "generated/linux"},
                {"id": "ios-native", "target": "ios", "surface": "mobile", "artifact": "generated/mobile/ios"},
                {"id": "android-native", "target": "android", "surface": "mobile", "artifact": "generated/mobile/android"}
            ],
            "audit": {
                "operationHistory": true,
                "cliParity": true
            }
        });
        let product = product_runtime::validate_product_ir_value(&product).unwrap();
        assert_eq!(product.app_id, "deployments");
        assert_eq!(product.targets, vec!["macos", "linux", "ios", "android"]);
        assert_eq!(
            product.desktop_surface_ir.as_deref(),
            Some("app-blueprint/desktop.surface.ir.json")
        );
        assert_eq!(
            product.mobile_surface_ir.as_deref(),
            Some("app-blueprint/mobile.surface.ir.json")
        );
        assert_eq!(product.domain_object_ids, vec!["server", "deployment"]);
        assert_eq!(
            product.state_snapshot_schema,
            "deployments-state/v1".to_string()
        );
        assert_eq!(
            product.persistence_truth,
            "headquarters-compatible-file-first".to_string()
        );
        assert_eq!(
            product.background_job_ids,
            vec!["server-scoped-check-and-push-queue".to_string()]
        );
        assert_eq!(
            product
                .release_targets
                .iter()
                .map(|target| target.id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "macos-native",
                "linux-native",
                "ios-native",
                "android-native"
            ]
        );
        assert_eq!(product.audit_keys, vec!["cliParity", "operationHistory"]);

        let invalid = serde_json::json!({
            "version": product_runtime::PRODUCT_IR_SCHEMA,
            "format": "json",
            "app": {
                "id": "deployments",
                "name": "Deployments",
                "targets": ["macos", "linux"]
            },
            "actions": [{
                "id": "refresh_state",
                "label": "Refresh",
                "input": {},
                "output": {},
                "effect": "read",
                "failure": {},
                "safe": true,
                "mutating": false,
                "longRunning": false,
                "privileged": false
            }],
            "state": {"snapshotSchema": "deployments-state/v1"},
            "releaseTargets": [
                {"id": "macos-native", "target": "macos", "surface": "desktop", "artifact": "generated/macos"}
            ]
        });
        let error = product_runtime::validate_product_ir_value(&invalid)
            .unwrap_err()
            .to_string();
        assert_eq!(
            error,
            "product IR releaseTargets missing release target for app target: linux"
        );
    }

    #[test]
    fn product_runtime_validates_surface_ir_contracts() {
        let desktop = serde_json::json!({
            "version": product_runtime::DESKTOP_SURFACE_IR_SCHEMA,
            "format": "json",
            "product": "deployments",
            "target": "desktop",
            "actions": ["refresh_state", "publish_changes"],
            "window": {
                "id": "window.main",
                "type": "Window",
                "role": "native-product-root",
                "child": {
                    "id": "split.main",
                    "type": "SplitPane",
                    "role": "left-list-detail",
                    "children": [
                        {"id": "list", "type": "TreeList", "role": "product-navigation"},
                        {"id": "detail", "type": "Detail", "role": "product-detail"}
                    ]
                }
            }
        });
        let desktop = product_runtime::validate_surface_ir_value(&desktop).unwrap();
        assert_eq!(desktop.product, "deployments");
        assert_eq!(desktop.target, "desktop");
        assert_eq!(
            desktop.action_ids,
            vec!["refresh_state".to_string(), "publish_changes".to_string()]
        );
        assert!(desktop.roles.contains(&"left-list-detail".to_string()));
        assert!(desktop.roles.contains(&"native-product-root".to_string()));

        let mobile = serde_json::json!({
            "version": product_runtime::MOBILE_SURFACE_IR_SCHEMA,
            "format": "json",
            "product": "deployments",
            "target": "mobile",
            "actions": ["refresh_state"],
            "screens": [{
                "id": "overview",
                "title": "Deployments",
                "node": {
                    "id": "screen.overview",
                    "type": "NavigationStack",
                    "role": "status-overview"
                }
            }]
        });
        let mobile = product_runtime::validate_surface_ir_value(&mobile).unwrap();
        assert_eq!(mobile.schema, product_runtime::MOBILE_SURFACE_IR_SCHEMA);
        assert_eq!(mobile.roles, vec!["status-overview".to_string()]);

        let invalid = serde_json::json!({
            "version": product_runtime::MOBILE_SURFACE_IR_SCHEMA,
            "format": "json",
            "product": "deployments",
            "target": "linux",
            "actions": [],
            "screens": []
        });
        let error = product_runtime::validate_surface_ir_value(&invalid)
            .unwrap_err()
            .to_string();
        assert_eq!(error, "mobile surface IR target invalid");
    }

    #[test]
    fn product_runtime_projects_target_surface_ir() {
        let product = serde_json::json!({
            "version": product_runtime::PRODUCT_IR_SCHEMA,
            "format": "json",
            "app": {
                "id": "deployments",
                "name": "Deployments",
                "targets": ["macos", "ios"]
            },
            "actions": [{
                "id": "refresh_state",
                "label": "Refresh",
                "input": {},
                "output": {},
                "effect": "read",
                "failure": {},
                "safe": true,
                "mutating": false,
                "longRunning": false,
                "privileged": false
            }],
            "state": {"snapshotSchema": "deployments-state/v1"},
            "releaseTargets": [
                {"id": "macos-native", "target": "macos", "surface": "desktop", "artifact": "generated/macos"},
                {"id": "ios-native", "target": "ios", "surface": "mobile", "artifact": "generated/mobile/ios"}
            ]
        });
        let product = product_runtime::validate_product_ir_value(&product).unwrap();
        let desktop = product_runtime::project_surface_from_product(&product, "macos").unwrap();
        let desktop = product_runtime::validate_surface_ir_value(&desktop).unwrap();
        assert_eq!(desktop.schema, product_runtime::DESKTOP_SURFACE_IR_SCHEMA);
        assert_eq!(desktop.target, "macos");
        assert!(desktop.roles.contains(&"left-list-detail".to_string()));
        let release_target =
            product_runtime::validate_native_compile_contract(&product, &desktop, "macos").unwrap();
        assert_eq!(release_target.id, "macos-native");
        let mut invalid_surface = desktop.clone();
        invalid_surface.action_ids = vec!["delete_everything".to_string()];
        let error =
            product_runtime::validate_native_compile_contract(&product, &invalid_surface, "macos")
                .unwrap_err()
                .to_string();
        assert_eq!(
            error,
            "surface IR action not declared in Product IR: delete_everything"
        );

        let mobile_text = product_runtime::project_surface_from_product_text(
            &serde_json::to_string(&serde_json::json!({
                "version": product_runtime::PRODUCT_IR_SCHEMA,
                "format": "json",
                "app": {
                    "id": "deployments",
                    "name": "Deployments",
                    "targets": ["ios"]
                },
                "actions": [{
                    "id": "refresh_state",
                    "label": "Refresh",
                    "input": {},
                    "output": {},
                    "effect": "read",
                    "failure": {},
                    "safe": true,
                    "mutating": false,
                    "longRunning": false,
                    "privileged": false
                }],
                "state": {"snapshotSchema": "deployments-state/v1"},
                "releaseTargets": [
                    {"id": "ios-native", "target": "ios", "surface": "mobile", "artifact": "generated/mobile/ios"}
                ]
            }))
            .unwrap(),
            "ios",
        )
        .unwrap();
        assert!(mobile_text.contains("\"version\": \"theurgy-mobile-surface-ir/v1\""));
        assert!(mobile_text.contains("\"role\": \"status-overview\""));

        let error = product_runtime::project_surface_from_product(&product, "android")
            .unwrap_err()
            .to_string();
        assert_eq!(error, "product IR does not declare target: android");
    }

    #[test]
    fn product_runtime_validates_generated_runtime_contract() {
        let runtime: serde_json::Value = serde_json::from_str(
            r#"{
  "version": "theurgy-generated-runtime/v1",
  "app": "deployments",
  "target": "macos",
  "productIr": "app-blueprint/product.ir.json",
  "runtimeManifest": "app-blueprint/runtime.manifest.json",
  "sourceSurfaceIr": "app-blueprint/desktop.surface.ir.json",
  "compatibilityWizardryAppsShellFirstStillSupported": true,
  "compatibilityTheurgyRequiredForLegacyWizardryApps": false,
  "productTargets": ["macos"],
  "productActions": ["refresh_state"],
  "productActionContracts": [{
    "id": "refresh_state",
    "label": "Refresh",
    "effect": "read",
    "safe": true,
    "mutating": false,
    "longRunning": false,
    "privileged": false,
    "inputKeys": [],
    "outputKeys": ["params"],
    "failureKeys": [],
    "inputShape": {},
    "outputShape": {"params": "object"},
    "failureShape": {}
  }],
  "productCapabilities": ["native-desktop"],
  "productPermissions": ["files"],
  "productDomainObjects": ["deployment"],
  "productStateSnapshotSchema": "deployments-state/v1",
  "productPersistenceRoots": ["headquarters-workspace"],
  "productPersistenceTruth": "file-first",
  "adapterRuntimeTransport": "local-process-json",
  "productBackgroundJobs": [],
  "productReleaseTargets": ["macos-app"],
  "targetReleaseTarget": "macos-app",
  "targetReleaseArtifact": "generated/macos",
  "productAuditKeys": ["cliParity"],
  "protocol": "theurgy-runtime-action/v1",
  "runtimeStatusSchema": "theurgy-runtime-status/v1",
  "runtimeActionRequestSchema": "theurgy-runtime-action-request/v1",
  "runtimeActionResultSchema": "theurgy-runtime-action-result/v1",
  "operationStatusSchema": "theurgy-operation-status/v1",
  "operationHistorySchema": "theurgy-operation-history/v1",
  "stateCommand": ["deployments-core", "runtime-state"],
  "statusCommand": ["deployments-core", "runtime-status"],
  "subscribeStatusCommand": ["deployments-core", "runtime-status"],
  "actionCommand": ["deployments-core", "runtime-action"],
  "historyCommand": ["deployments-core", "runtime-history"],
  "surface": "theurgy-surface.json",
  "surfaceSchema": "theurgy-desktop-surface-ir/v1",
  "surfaceTarget": "macos",
  "surfaceActions": ["refresh_state"],
  "surfaceActionContracts": [{
    "id": "refresh_state",
    "label": "Refresh",
    "effect": "read",
    "safe": true,
    "mutating": false,
    "longRunning": false,
    "privileged": false,
    "inputKeys": [],
    "outputKeys": ["params"],
    "failureKeys": [],
    "inputShape": {},
    "outputShape": {"params": "object"},
    "failureShape": {}
  }],
  "surfaceRoles": ["left-list-detail"]
}"#,
        )
        .unwrap();
        let summary = product_runtime::validate_generated_runtime_value(&runtime).unwrap();
        assert_eq!(summary.app_id, "deployments");
        assert_eq!(
            summary.adapter_runtime_transport,
            product_runtime::DESKTOP_ADAPTER_TRANSPORT
        );
        assert_eq!(summary.surface_action_contracts, 1);

        let mut invalid = runtime.clone();
        invalid.as_object_mut().unwrap().insert(
            "adapterRuntimeTransport".to_string(),
            serde_json::json!("wrong"),
        );
        let error = product_runtime::validate_generated_runtime_value(&invalid)
            .unwrap_err()
            .to_string();
        assert_eq!(
            error,
            "generated runtime adapterRuntimeTransport must match target family"
        );
    }

    #[test]
    fn product_runtime_validates_runtime_action_contract_payloads() {
        let contract = product_runtime::ProductActionContract {
            id: "publish_changes".to_string(),
            label: "Push to Production".to_string(),
            effect: "release".to_string(),
            safe: false,
            mutating: true,
            long_running: true,
            privileged: true,
            command: Vec::new(),
            input_keys: vec!["deployment".to_string()],
            output_keys: vec!["params".to_string()],
            failure_keys: vec!["error".to_string()],
            input_shape: [("deployment".to_string(), "string".to_string())]
                .into_iter()
                .collect(),
            output_shape: [("params".to_string(), "object".to_string())]
                .into_iter()
                .collect(),
            failure_shape: [("error".to_string(), "string".to_string())]
                .into_iter()
                .collect(),
        };
        let contracts = vec![contract];

        product_runtime::validate_runtime_action_params_text(
            "publish_changes",
            "{\"deployment\":\"site-one\"}",
            &contracts,
        )
        .unwrap();
        product_runtime::validate_runtime_action_operation_contract(
            "publish_changes",
            true,
            &contracts,
        )
        .unwrap();
        product_runtime::validate_runtime_action_result_contract_value(
            "publish_changes",
            &serde_json::json!({"result": {"params": {}}}),
            &contracts,
        )
        .unwrap();
        product_runtime::validate_runtime_action_failure_contract_text(
            "publish_changes",
            "{\"success\":false,\"error\":\"failed\"}",
            &contracts,
        )
        .unwrap();

        let error = product_runtime::validate_runtime_action_params_text(
            "publish_changes",
            "{\"deployment\":false}",
            &contracts,
        )
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "runtime action param type mismatch for publish_changes.deployment: expected string"
        );

        let error = product_runtime::validate_runtime_action_failure_contract_text(
            "publish_changes",
            "{\"success\":false,\"error\":false}",
            &contracts,
        )
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "runtime action failure type mismatch for publish_changes.error: expected string"
        );
    }

    #[test]
    fn product_runtime_builds_generated_runtime_metadata() {
        let product = serde_json::json!({
            "version": product_runtime::PRODUCT_IR_SCHEMA,
            "format": "json",
            "app": {
                "id": "deployments",
                "name": "Deployments",
                "targets": ["macos", "linux"],
                "capabilities": ["native-desktop"],
                "permissions": ["files"]
            },
            "domain": {
                "objects": [{"id": "deployment", "label": "Deployment"}]
            },
            "actions": [{
                "id": "refresh_state",
                "label": "Refresh",
                "input": {},
                "output": {"params": "object"},
                "effect": "read",
                "failure": {},
                "safe": true,
                "mutating": false,
                "longRunning": false,
                "privileged": false
            }],
            "state": {
                "snapshotSchema": "deployments-state/v1",
                "roots": [{"id": "headquarters-workspace", "kind": "xdg-state"}]
            },
            "releaseTargets": [
                {"id": "macos-app", "target": "macos", "surface": "desktop", "artifact": "generated/macos"},
                {"id": "linux-app", "target": "linux", "surface": "desktop", "artifact": "generated/linux"}
            ],
            "persistence": {"truth": "file-first"},
            "audit": {"cliParity": true}
        });
        let product = product_runtime::validate_product_ir_value(&product).unwrap();
        let surface = product_runtime::project_surface_from_product(&product, "macos").unwrap();
        let surface = product_runtime::validate_surface_ir_value(&surface).unwrap();
        let runtime = product_runtime::RuntimeBridge {
            app_id: "deployments".to_string(),
            protocol: product_runtime::RUNTIME_ACTION_PROTOCOL.to_string(),
            product_ir: "app-blueprint/product.ir.json".to_string(),
            runtime_manifest: "app-blueprint/runtime.manifest.json".to_string(),
            source_surface_ir: "app-blueprint/desktop.surface.ir.json".to_string(),
            legacy_native_desktop_ir: Some("app-blueprint/app.ir.yaml".to_string()),
            compatibility: product_runtime::RuntimeCompatibility::shell_first_default(),
            state_command: vec!["deployments-core".to_string(), "runtime-state".to_string()],
            status_command: vec!["deployments-core".to_string(), "runtime-status".to_string()],
            subscribe_status_command: Vec::new(),
            operation_status_command: vec![
                "deployments-core".to_string(),
                "runtime-operation-status".to_string(),
            ],
            action_command: vec!["deployments-core".to_string(), "runtime-action".to_string()],
            history_command: vec![
                "deployments-core".to_string(),
                "runtime-history".to_string(),
            ],
            daemon_command: Vec::new(),
        };

        let metadata =
            product_runtime::generated_runtime_metadata(&product, &runtime, "macos", &surface)
                .unwrap();
        let summary = product_runtime::validate_generated_runtime_text(&metadata).unwrap();
        assert_eq!(summary.release_target, "macos-app");
        assert_eq!(summary.release_artifact, "generated/macos");
        assert_eq!(
            summary.adapter_runtime_transport,
            product_runtime::DESKTOP_ADAPTER_TRANSPORT
        );
        assert_eq!(summary.surface_actions, 1);
        assert!(metadata.contains("\"legacyNativeDesktopIr\": \"app-blueprint/app.ir.yaml\""));

        let files = product_runtime::macos_adapter_files(&product, &surface, &runtime);
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|file| file.path == "Package.swift"
            && file.contents.contains("platforms: [.macOS(.v13)]")));
        assert!(files.iter().any(|file| file.path == "Sources/App/App.swift"
            && file.contents.contains("let runtimeStateCommand")
            && file
                .contents
                .contains("runtimeActionCommand + [action.id, json]")));
        let linux_surface =
            product_runtime::project_surface_from_product(&product, "linux").unwrap();
        let linux_surface = product_runtime::validate_surface_ir_value(&linux_surface).unwrap();
        let files = product_runtime::linux_adapter_files(&product, &linux_surface, &runtime);
        assert_eq!(files.len(), 2);
        assert!(files
            .iter()
            .any(|file| file.path == "meson.build" && file.contents.contains("gtk4")));
        assert!(files.iter().any(|file| file.path == "src/main.c"
            && file.contents.contains("surface_action_contracts_json")
            && file.contents.contains("resolve_executable")));
    }
}
