pub mod product_runtime {
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
}
