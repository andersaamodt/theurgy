use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use theurgy::product_runtime;

const AGPL_NOTICE: &str = "GNU AGPL-3.0-or-later\n";
const WIZARDRY_ADDENDUM: &str = "Wizardry Addendum 1.0\n\nAdditional terms under GNU AGPL version 3,\nsection 7, apply to this project.\n\n1. No permission is granted to use the names\n\"Wizardry\" or \"Open Wizardry\", or any project\ntrade names, trademarks, or service marks,\nexcept for reasonable descriptive reference.\n\n2. Those names may not be used in advertising,\npublicity, product naming, or public statements\nin any way that misrepresents the origin of the\nsoftware or implies endorsement, sponsorship,\nofficial status, or association.\n\n3. Modified versions and derivative works must\nnot present themselves as the original Wizardry\nproject or as officially associated with it.\n\n4. Truthful descriptive references are allowed,\nincluding statements that a work was generated\nwith, built with, or adapted from Wizardry,\nprovided those statements do not imply\nendorsement, sponsorship, official status,\nor association.\n";

#[derive(Debug)]
struct TheurgyError {
    message: String,
}

impl TheurgyError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for TheurgyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for TheurgyError {}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProjectKind {
    Desktop,
    Website,
}

impl ProjectKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Website => "website",
        }
    }
}

fn main() {
    if let Err(error) = run(env::args().collect()) {
        eprintln!("theurgy-runtime: {error}");
        std::process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<()> {
    match args.get(1).map(String::as_str) {
        None | Some("--help") | Some("-h") => {
            print_usage();
            Ok(())
        }
        Some("assay") => assay(),
        Some("capture-cgi-context") => command_capture_cgi_context(),
        Some("check-web-runtime") => command_check_web_runtime(),
        Some("conjure-native-desktop") => command_conjure(ProjectKind::Desktop, &args[2..]),
        Some("conjure-enterprise-website") => command_conjure(ProjectKind::Website, &args[2..]),
        Some("inspect") => command_inspect(&args[2..]),
        Some("validate-product-ir") => command_validate_product_ir(&args[2..]),
        Some("validate-action-ir") => command_validate_action_ir(&args[2..]),
        Some("validate-state-snapshot") => command_validate_state_snapshot(&args[2..]),
        Some("validate-runtime-status") => command_validate_runtime_status(&args[2..]),
        Some("validate-runtime-action-request") => {
            command_validate_runtime_action_request(&args[2..])
        }
        Some("validate-runtime-action-result") => {
            command_validate_runtime_action_result(&args[2..])
        }
        Some("validate-operation-status") => command_validate_operation_status(&args[2..]),
        Some("validate-operation-history") => command_validate_operation_history(&args[2..]),
        Some("validate-runtime-manifest") => command_validate_runtime_manifest(&args[2..]),
        Some("validate-generated-runtime") => command_validate_generated_runtime(&args[2..]),
        Some("validate-surface-ir") => command_validate_surface_ir(&args[2..]),
        Some("project-surface") => command_project_surface(&args[2..]),
        Some("compile-native") => command_compile_native(&args[2..]),
        Some("compile-app") => command_compile_app(&args[2..]),
        Some("inspect-app") => command_inspect_app(&args[2..]),
        Some("run-state") => command_run_state(&args[2..]),
        Some("run-status") => command_run_status(&args[2..]),
        Some("subscribe-status") => command_subscribe_status(&args[2..]),
        Some("run-operation-status") => command_run_operation_status(&args[2..]),
        Some("run-history") => command_run_history(&args[2..]),
        Some("run-action") => command_run_action(&args[2..]),
        Some(other) => Err(TheurgyError::new(format!("unknown command: {other}")).into()),
    }
}

fn print_usage() {
    println!(
        "Internal runtime. Use spells/assay-theurgy, spells/check-theurgy-web-runtime, spells/capture-theurgy-cgi-context, spells/conjure-native-desktop, spells/conjure-enterprise-website, spells/inspect-theurgy-project, or the product runtime/compiler commands."
    );
}

fn assay() -> Result<()> {
    println!("theurgy=ok");
    println!("runtime=rust");
    println!("wizardry_relation=extension-not-replacement");
    println!("file_first_default=yes");
    println!("database_default=no");
    Ok(())
}

fn command_check_web_runtime() -> Result<()> {
    for line in web_runtime_assay_lines() {
        println!("{line}");
    }
    Ok(())
}

fn web_runtime_assay_lines() -> Vec<&'static str> {
    vec![
        "theurgy_web_runtime=ready",
        "phase=contract-and-adapter",
        "front_doors=nginx,lighttpd",
        "adapters=http,fastcgi,cgi-compat",
        "router=axum",
        "serialization=serde",
        "templates=tera",
        "search=tantivy",
        "truth=file-first",
        "state=plain-files-plus-derived-indexes",
        "wizardry_layer=spells-install-check-publish-maintenance",
        "hot_path_layer=rust",
        "zola=not-core-runtime",
        "desk_phase2_ready=yes",
    ]
}

fn command_capture_cgi_context() -> Result<()> {
    let context = WebRequestContext::from_env();
    println!("{}", context.to_json());
    Ok(())
}

fn command_conjure(kind: ProjectKind, args: &[String]) -> Result<()> {
    if args.is_empty() || args.len() > 2 {
        return Err(TheurgyError::new("usage: spell NAME [PATH]").into());
    }
    let name = validate_name(&args[0])?;
    let path = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(name));

    create_project(kind, name, &path)?;
    println!("created={} path={}", kind.as_str(), path.display());
    Ok(())
}

fn command_inspect(args: &[String]) -> Result<()> {
    if args.len() > 1 {
        return Err(TheurgyError::new("usage: inspect-theurgy-project [PATH]").into());
    }
    let path = args
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let manifest_path = path.join("theurgy.project.toml");
    let manifest = fs::read_to_string(&manifest_path).map_err(|error| {
        TheurgyError::new(format!(
            "could not read {}: {error}",
            manifest_path.display()
        ))
    })?;
    let summary = inspect_manifest(&manifest)?;
    println!("name={}", summary.name);
    println!("kind={}", summary.kind);
    println!("source_root={}", summary.source_root);
    println!("truth=file-first");
    Ok(())
}

fn command_validate_product_ir(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-product-ir PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_product_ir(&value)?;
    println!("status=ok");
    println!("schema=theurgy-product-ir/v1");
    println!("app={}", summary.app_id);
    println!("actions={}", summary.actions);
    println!("targets={}", summary.targets.join(","));
    Ok(())
}

fn command_validate_action_ir(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-action-ir PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_action_ir(&value)?;
    println!("status=ok");
    println!("schema=theurgy-action-ir/v1");
    println!("actions={}", summary.actions);
    println!("ids={}", summary.action_ids.join(","));
    Ok(())
}

fn command_validate_state_snapshot(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-state-snapshot PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_state_snapshot(&value)?;
    println!("status=ok");
    println!("schema=theurgy-state-snapshot/v1");
    println!("app={}", summary.app_id);
    Ok(())
}

fn command_validate_runtime_status(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-runtime-status PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_runtime_status(&value)?;
    println!("status=ok");
    println!("schema=theurgy-runtime-status/v1");
    println!("app={}", summary.app_id);
    Ok(())
}

fn command_validate_runtime_action_result(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-runtime-action-result PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_runtime_action_result(&value)?;
    println!("status=ok");
    println!("protocol={}", product_runtime::RUNTIME_ACTION_PROTOCOL);
    println!("app={}", summary.app_id);
    println!("action={}", summary.action_id);
    println!("operation={}", summary.operation_id);
    Ok(())
}

fn command_validate_runtime_action_request(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(TheurgyError::new(
            "usage: validate-runtime-action-request PATH [--manifest PATH]",
        )
        .into());
    }
    let mut path: Option<&str> = None;
    let mut manifest_path: Option<&Path> = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--manifest" => {
                index += 1;
                let Some(raw) = args.get(index) else {
                    return Err(TheurgyError::new("missing value for --manifest").into());
                };
                manifest_path = Some(Path::new(raw));
            }
            raw if path.is_none() => path = Some(raw),
            other => {
                return Err(TheurgyError::new(format!(
                    "unexpected validate-runtime-action-request argument: {other}"
                ))
                .into())
            }
        }
        index += 1;
    }
    let Some(path) = path else {
        return Err(TheurgyError::new(
            "usage: validate-runtime-action-request PATH [--manifest PATH]",
        )
        .into());
    };
    let value = read_json(Path::new(path))?;
    let summary = validate_runtime_action_request(&value)?;
    if let Some(manifest_path) = manifest_path {
        let runtime = runtime_contract_from_path_with_product_actions(manifest_path)?;
        let parsed = parse_json(&value)?;
        validate_runtime_action_request_against_runtime(&summary, &parsed, &runtime)?;
    }
    println!("status=ok");
    println!("protocol={}", product_runtime::RUNTIME_ACTION_PROTOCOL);
    println!("app={}", summary.app_id);
    println!("action={}", summary.action_id);
    Ok(())
}

fn command_validate_operation_status(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-operation-status PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_operation_status(&value)?;
    println!("status=ok");
    println!("schema=theurgy-operation-status/v1");
    println!("app={}", summary.app_id);
    println!("operation={}", summary.operation_id);
    Ok(())
}

fn command_validate_operation_history(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-operation-history PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_operation_history(&value)?;
    println!("status=ok");
    println!("schema=theurgy-operation-history/v1");
    println!("app={}", summary.app_id);
    println!("entries={}", summary.entries);
    Ok(())
}

fn command_validate_runtime_manifest(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-runtime-manifest PATH").into());
    }
    let summary =
        runtime_manifest_summary_from_library(product_runtime::load_runtime_manifest(&args[0])?);
    println!("status=ok");
    println!("schema=theurgy-runtime-manifest/v1");
    println!("app={}", summary.app_id);
    println!("product_ir={}", summary.product_ir);
    println!("protocol={}", summary.protocol);
    println!(
        "compatibility_wizardry_apps_shell_first={}",
        summary
            .compatibility
            .wizardry_apps_shell_first_still_supported
    );
    println!(
        "compatibility_theurgy_required_for_legacy_wizardry_apps={}",
        summary
            .compatibility
            .theurgy_required_for_legacy_wizardry_apps
    );
    if let Some(path) = summary.desktop_surface_ir {
        println!("desktop_surface_ir={path}");
    }
    if let Some(path) = summary.mobile_surface_ir {
        println!("mobile_surface_ir={path}");
    }
    if let Some(path) = summary.legacy_native_desktop_ir {
        println!("legacy_native_desktop_ir={path}");
    }
    Ok(())
}

fn command_validate_generated_runtime(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-generated-runtime PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_generated_runtime(&value)?;
    println!("status=ok");
    println!("schema=theurgy-generated-runtime/v1");
    println!("app={}", summary.app_id);
    println!("target={}", summary.target);
    println!(
        "product_state_snapshot_schema={}",
        summary.state_snapshot_schema
    );
    println!("product_persistence_truth={}", summary.persistence_truth);
    println!(
        "adapter_runtime_transport={}",
        summary.adapter_runtime_transport
    );
    println!("runtime_status_schema={}", summary.runtime_status_schema);
    println!(
        "runtime_action_request_schema={}",
        summary.runtime_action_request_schema
    );
    println!(
        "runtime_action_result_schema={}",
        summary.runtime_action_result_schema
    );
    println!(
        "operation_status_schema={}",
        summary.operation_status_schema
    );
    println!(
        "operation_history_schema={}",
        summary.operation_history_schema
    );
    println!("surface_schema={}", summary.surface_schema);
    println!("surface_target={}", summary.surface_target);
    println!("actions={}", summary.actions);
    println!("product_actions={}", summary.product_actions);
    println!("surface_actions={}", summary.surface_actions);
    println!(
        "surface_action_contracts={}",
        summary.surface_action_contracts
    );
    Ok(())
}

fn command_validate_surface_ir(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-surface-ir PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_surface_ir(&value)?;
    println!("status=ok");
    println!("schema={}", summary.schema);
    println!("product={}", summary.product);
    println!("target={}", summary.target);
    Ok(())
}

fn command_project_surface(args: &[String]) -> Result<()> {
    let (path, target) =
        parse_product_target_args(args, "usage: project-surface PRODUCT_IR --target TARGET")?;
    let product = read_json(path)?;
    validate_product_ir(&product)?;
    let surface = project_surface(&product, target)?;
    println!("{surface}");
    Ok(())
}

fn command_compile_native(args: &[String]) -> Result<()> {
    let (product_path, target, out_dir) = parse_compile_args(args)?;
    let product = read_json(product_path)?;
    validate_product_ir(&product)?;
    compile_native(&product, target, out_dir)?;
    println!("status=ok");
    println!("target={target}");
    println!("out={}", out_dir.display());
    Ok(())
}

fn command_compile_app(args: &[String]) -> Result<()> {
    let (app_dir, target, out_dir) = parse_compile_args(args)?;
    let manifest_path = app_dir.join("theurgy.project.toml");
    let manifest = fs::read_to_string(&manifest_path).map_err(|error| {
        TheurgyError::new(format!(
            "could not read {}: {error}",
            manifest_path.display()
        ))
    })?;
    let product_ir = manifest_value(&manifest, "product_ir").map_err(|_| {
        TheurgyError::new("compile-app requires product_ir in theurgy.project.toml")
    })?;
    let runtime_manifest = manifest_value(&manifest, "runtime_manifest").map_err(|_| {
        TheurgyError::new("compile-app requires runtime_manifest in theurgy.project.toml")
    })?;
    let surface_key = if matches!(target, "macos" | "linux") {
        "desktop_surface_ir"
    } else {
        "mobile_surface_ir"
    };
    let surface_ir = manifest_value(&manifest, surface_key).map_err(|_| {
        TheurgyError::new(format!(
            "compile-app requires {surface_key} in theurgy.project.toml for target {target}"
        ))
    })?;
    let product_path = app_dir.join(&product_ir);
    let runtime_path = app_dir.join(&runtime_manifest);
    let surface_path = app_dir.join(&surface_ir);
    let product = read_json(&product_path)?;
    let product_summary = validate_product_ir(&product)?;
    let surface_kind = if matches!(target, "macos" | "linux") {
        "desktop"
    } else {
        "mobile"
    };
    validate_product_surface_path(&product_summary, surface_kind, &surface_ir)?;
    if !product_summary
        .targets
        .iter()
        .any(|candidate| candidate == target)
    {
        return Err(
            TheurgyError::new(format!("product IR does not declare target: {target}")).into(),
        );
    }
    let runtime_text = read_json(&runtime_path)?;
    let runtime_summary = validate_runtime_manifest(&runtime_text)?;
    if runtime_summary.app_id != product_summary.app_id {
        return Err(TheurgyError::new("runtime manifest app does not match product IR app").into());
    }
    if runtime_summary.product_ir != product_ir {
        return Err(TheurgyError::new(format!(
            "runtime manifest productIr does not match theurgy.project.toml product_ir: {}",
            runtime_summary.product_ir
        ))
        .into());
    }
    let manifest_surface_ir = if matches!(target, "macos" | "linux") {
        runtime_summary.desktop_surface_ir.as_deref()
    } else {
        runtime_summary.mobile_surface_ir.as_deref()
    }
    .ok_or_else(|| {
        TheurgyError::new(format!(
            "runtime manifest surfaces entry required for target {target}"
        ))
    })?;
    if manifest_surface_ir != surface_ir {
        return Err(TheurgyError::new(format!(
            "runtime manifest surface path does not match theurgy.project.toml {surface_key}: {manifest_surface_ir}"
        ))
        .into());
    }
    let runtime_contract = runtime_contract_from_manifest(&runtime_text)?;
    let runtime_contract = runtime_contract.with_sources(
        product_ir.clone(),
        runtime_manifest.clone(),
        surface_ir.clone(),
    );
    validate_product_action_commands(&product_summary, &runtime_contract)?;
    let surface = read_json(&surface_path)?;
    let surface_summary = validate_surface_ir(&surface)?;
    if surface_summary.product != product_summary.app_id {
        return Err(TheurgyError::new("surface IR product does not match product IR app").into());
    }
    for action_id in &surface_summary.action_ids {
        if !product_summary
            .action_ids
            .iter()
            .any(|product_action| product_action == action_id)
        {
            return Err(TheurgyError::new(format!(
                "surface IR action not declared in Product IR: {action_id}"
            ))
            .into());
        }
    }
    let expected_surface_target = if surface_kind == "desktop" {
        "desktop"
    } else {
        "mobile"
    };
    if surface_summary.target != target && surface_summary.target != expected_surface_target {
        return Err(TheurgyError::new(format!(
            "surface IR target must be {target} or {expected_surface_target}"
        ))
        .into());
    }
    compile_native_with_contract(
        &product_summary,
        &surface,
        &runtime_contract,
        target,
        out_dir,
        runtime_contract.legacy_native_desktop_ir.is_some(),
    )?;
    println!("status=ok");
    println!("app={}", app_dir.display());
    println!("target={target}");
    println!("out={}", out_dir.display());
    Ok(())
}

fn command_inspect_app(args: &[String]) -> Result<()> {
    if args.len() > 1 {
        return Err(TheurgyError::new("usage: inspect-app [PATH]").into());
    }
    let path = args
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    for line in inspect_app_lines(&path)? {
        println!("{line}");
    }
    Ok(())
}

fn inspect_app_lines(path: &Path) -> Result<Vec<String>> {
    let manifest_path = path.join("theurgy.project.toml");
    let manifest = fs::read_to_string(&manifest_path).map_err(|error| {
        TheurgyError::new(format!(
            "could not read {}: {error}",
            manifest_path.display()
        ))
    })?;
    let manifest_summary = inspect_manifest(&manifest)?;
    let mut lines = vec![
        format!("name={}", manifest_summary.name),
        format!("kind={}", manifest_summary.kind),
        format!("source_root={}", manifest_summary.source_root),
        "truth=file-first".to_string(),
    ];
    let product_ir = manifest_value(&manifest, "product_ir").map_err(|_| {
        TheurgyError::new("inspect-app requires product_ir in theurgy.project.toml")
    })?;
    let runtime_manifest = manifest_value(&manifest, "runtime_manifest").map_err(|_| {
        TheurgyError::new("inspect-app requires runtime_manifest in theurgy.project.toml")
    })?;
    lines.push(format!("product_ir={product_ir}"));
    if let Ok(desktop_surface_ir) = manifest_value(&manifest, "desktop_surface_ir") {
        lines.push(format!("desktop_surface_ir={desktop_surface_ir}"));
    }
    if let Ok(mobile_surface_ir) = manifest_value(&manifest, "mobile_surface_ir") {
        lines.push(format!("mobile_surface_ir={mobile_surface_ir}"));
    }
    lines.push(format!("runtime_manifest={runtime_manifest}"));

    let product_text = read_json(&path.join(&product_ir))?;
    let product = validate_product_ir(&product_text)?;
    lines.push(format!("product_app={}", product.app_id));
    lines.push(format!("product_targets={}", product.targets.join(",")));
    if let Some(desktop_surface_ir) = &product.desktop_surface_ir {
        lines.push(format!("product_surface_desktop={desktop_surface_ir}"));
    }
    if let Some(mobile_surface_ir) = &product.mobile_surface_ir {
        lines.push(format!("product_surface_mobile={mobile_surface_ir}"));
    }
    lines.push(format!(
        "product_state_snapshot_schema={}",
        product.state_snapshot_schema
    ));
    if !product.state_command.is_empty() {
        lines.push(format!(
            "product_state_command={}",
            command_text(&product.state_command)
        ));
    }
    if !product.state_status_command.is_empty() {
        lines.push(format!(
            "product_state_status_command={}",
            command_text(&product.state_status_command)
        ));
    }
    lines.push(format!("product_actions={}", product.actions));
    lines.push(format!(
        "product_long_running_actions={}",
        product
            .action_contracts
            .iter()
            .filter(|contract| contract.long_running)
            .count()
    ));
    lines.push(format!(
        "product_persistence_truth={}",
        product.persistence_truth
    ));
    lines.push(format!(
        "product_background_jobs={}",
        product.background_job_ids.join(",")
    ));
    for job in &product.background_jobs {
        if !job.command.is_empty() {
            lines.push(format!(
                "product_background_job_{}_command={}",
                job.id,
                command_text(&job.command)
            ));
        }
    }
    lines.push(format!(
        "product_release_targets={}",
        release_target_ids(&product).join(",")
    ));
    lines.push(format!(
        "product_audit_keys={}",
        product.audit_keys.join(",")
    ));

    let runtime_path = path.join(&runtime_manifest);
    let runtime_text = read_json(&runtime_path)?;
    let runtime_summary = validate_runtime_manifest(&runtime_text)?;
    if runtime_summary.app_id != product.app_id {
        return Err(TheurgyError::new("runtime manifest app does not match product IR app").into());
    }
    if runtime_summary.product_ir != product_ir {
        return Err(TheurgyError::new(format!(
            "runtime manifest productIr does not match theurgy.project.toml product_ir: {}",
            runtime_summary.product_ir
        ))
        .into());
    }
    let runtime = runtime_contract_from_manifest(&runtime_text)?;
    validate_product_action_commands(&product, &runtime)?;
    lines.push(format!("runtime_protocol={}", runtime.protocol));
    lines.push(format!(
        "runtime_state_command={}",
        command_text(&runtime.state_command)
    ));
    lines.push(format!(
        "runtime_status_command={}",
        command_text(&runtime.status_command)
    ));
    lines.push(format!(
        "runtime_subscribe_status_command={}",
        command_text(&effective_subscribe_status_command(&runtime))
    ));
    lines.push(format!(
        "runtime_operation_status_command={}",
        command_text(&runtime.operation_status_command)
    ));
    lines.push(format!(
        "runtime_action_command={}",
        command_text(&runtime.action_command)
    ));
    lines.push(format!(
        "runtime_history_command={}",
        command_text(&runtime.history_command)
    ));
    lines.push(format!(
        "runtime_daemon_command={}",
        command_text(&runtime.daemon_command)
    ));
    lines.extend(inspect_runtime_compatibility_lines(&runtime_text)?);

    if let Ok(desktop_surface_ir) = manifest_value(&manifest, "desktop_surface_ir") {
        validate_product_surface_path(&product, "desktop", &desktop_surface_ir)?;
        let surface = validate_declared_surface(&path, &desktop_surface_ir, &product)?;
        lines.push(format!("desktop_surface_schema={}", surface.schema));
        lines.push(format!("desktop_surface_target={}", surface.target));
        lines.push(format!(
            "desktop_surface_actions={}",
            surface.action_ids.len()
        ));
        lines.push(format!("desktop_surface_roles={}", surface.roles.join(",")));
        if runtime_summary.desktop_surface_ir.as_deref() != Some(desktop_surface_ir.as_str()) {
            return Err(TheurgyError::new(
                "runtime manifest surfaces.desktop does not match theurgy.project.toml",
            )
            .into());
        }
    }
    if let Ok(mobile_surface_ir) = manifest_value(&manifest, "mobile_surface_ir") {
        validate_product_surface_path(&product, "mobile", &mobile_surface_ir)?;
        let surface = validate_declared_surface(&path, &mobile_surface_ir, &product)?;
        lines.push(format!("mobile_surface_schema={}", surface.schema));
        lines.push(format!("mobile_surface_target={}", surface.target));
        lines.push(format!(
            "mobile_surface_actions={}",
            surface.action_ids.len()
        ));
        lines.push(format!("mobile_surface_roles={}", surface.roles.join(",")));
        if runtime_summary.mobile_surface_ir.as_deref() != Some(mobile_surface_ir.as_str()) {
            return Err(TheurgyError::new(
                "runtime manifest surfaces.mobile does not match theurgy.project.toml",
            )
            .into());
        }
    }
    if let Some(legacy_native_desktop_ir) = &runtime_summary.legacy_native_desktop_ir {
        lines.push(format!(
            "runtime_legacy_native_desktop_ir={legacy_native_desktop_ir}"
        ));
    }
    Ok(lines)
}

fn validate_declared_surface(
    app_dir: &Path,
    surface_ir: &str,
    product: &ProductSummary,
) -> Result<SurfaceSummary> {
    let surface_text = read_json(&app_dir.join(surface_ir))?;
    let surface = validate_surface_ir(&surface_text)?;
    if surface.product != product.app_id {
        return Err(TheurgyError::new("surface IR product does not match product IR app").into());
    }
    for action_id in &surface.action_ids {
        if !product
            .action_ids
            .iter()
            .any(|product_action| product_action == action_id)
        {
            return Err(TheurgyError::new(format!(
                "surface IR action not declared in Product IR: {action_id}"
            ))
            .into());
        }
    }
    Ok(surface)
}

fn inspect_runtime_compatibility_lines(runtime_text: &str) -> Result<Vec<String>> {
    let value = parse_json(runtime_text)?;
    let mut lines = Vec::new();
    let Some(compatibility) = value.get("compatibility") else {
        return Ok(lines);
    };
    let compatibility = compatibility
        .as_object()
        .ok_or_else(|| TheurgyError::new("runtime manifest compatibility must be an object"))?;
    if let Some(value) = compatibility.get("wizardryAppsShellFirstStillSupported") {
        let flag = value.as_bool().ok_or_else(|| {
            TheurgyError::new(
                "runtime manifest compatibility.wizardryAppsShellFirstStillSupported must be boolean",
            )
        })?;
        lines.push(format!("compatibility_wizardry_apps_shell_first={flag}"));
    }
    if let Some(value) = compatibility.get("theurgyRequiredForLegacyWizardryApps") {
        let flag = value.as_bool().ok_or_else(|| {
            TheurgyError::new(
                "runtime manifest compatibility.theurgyRequiredForLegacyWizardryApps must be boolean",
            )
        })?;
        lines.push(format!(
            "compatibility_theurgy_required_for_legacy_wizardry_apps={flag}"
        ));
    }
    Ok(lines)
}

fn validate_product_action_commands(
    product: &ProductSummary,
    runtime: &RuntimeContract,
) -> Result<()> {
    validate_optional_product_command(
        "state.command",
        &product.state_command,
        "stateCommand",
        &runtime.state_command,
    )?;
    validate_optional_product_command(
        "state.statusCommand",
        &product.state_status_command,
        "statusCommand",
        &runtime.status_command,
    )?;
    for job in &product.background_jobs {
        validate_optional_product_command(
            &format!("backgroundJobs.{}.command", job.id),
            &job.command,
            "daemonCommand",
            &runtime.daemon_command,
        )?;
    }
    for contract in &product.action_contracts {
        if contract.command.is_empty() {
            continue;
        }
        if runtime.action_command.is_empty() {
            return Err(TheurgyError::new(
                "product IR action.command requires runtime manifest actionCommand",
            )
            .into());
        }
        let expected_len = runtime.action_command.len() + 2;
        if contract.command.len() != expected_len {
            return Err(TheurgyError::new(format!(
                "product IR action.command for {} must be runtime actionCommand plus action id and JSON params",
                contract.id
            ))
            .into());
        }
        if contract.command[..runtime.action_command.len()] != runtime.action_command[..] {
            return Err(TheurgyError::new(format!(
                "product IR action.command for {} must start with runtime manifest actionCommand",
                contract.id
            ))
            .into());
        }
        if contract.command[runtime.action_command.len()] != contract.id {
            return Err(TheurgyError::new(format!(
                "product IR action.command action id mismatch for {}",
                contract.id
            ))
            .into());
        }
        let params = contract
            .command
            .last()
            .map(String::as_str)
            .unwrap_or_default();
        let expected_params = if contract.input_keys.is_empty() {
            "{}"
        } else {
            "<json>"
        };
        if params != expected_params {
            return Err(TheurgyError::new(format!(
                "product IR action.command params for {} must be {}",
                contract.id, expected_params
            ))
            .into());
        }
    }
    Ok(())
}

fn validate_optional_product_command(
    product_key: &str,
    product_command: &[String],
    runtime_key: &str,
    runtime_command: &[String],
) -> Result<()> {
    if product_command.is_empty() {
        return Ok(());
    }
    if runtime_command.is_empty() {
        return Err(TheurgyError::new(format!(
            "product IR {product_key} requires runtime manifest {runtime_key}"
        ))
        .into());
    }
    if product_command != runtime_command {
        return Err(TheurgyError::new(format!(
            "product IR {product_key} must match runtime manifest {runtime_key}"
        ))
        .into());
    }
    Ok(())
}

fn validate_product_surface_path(
    product: &ProductSummary,
    surface_kind: &str,
    surface_ir: &str,
) -> Result<()> {
    let declared = match surface_kind {
        "desktop" => product.desktop_surface_ir.as_deref(),
        "mobile" => product.mobile_surface_ir.as_deref(),
        _ => None,
    };
    if let Some(declared) = declared {
        if declared != surface_ir {
            return Err(TheurgyError::new(format!(
                "product IR surfaces.{surface_kind} does not match declared surface IR: {declared}"
            ))
            .into());
        }
    }
    Ok(())
}

fn command_text(command: &[String]) -> String {
    command.join(" ")
}

fn command_run_state(args: &[String]) -> Result<()> {
    let manifest_path = parse_manifest_only_args(args, "usage: run-state --manifest PATH")?;
    let output = run_state_output(&manifest_path)?;
    print!("{output}");
    Ok(())
}

fn command_run_status(args: &[String]) -> Result<()> {
    let manifest_path = parse_manifest_only_args(args, "usage: run-status --manifest PATH")?;
    let output = run_status_output(&manifest_path)?;
    print!("{output}");
    Ok(())
}

fn command_subscribe_status(args: &[String]) -> Result<()> {
    let manifest_path = parse_subscribe_status_args(args)?;
    let output = subscribe_status_output(&manifest_path)?;
    print!("{output}");
    Ok(())
}

fn command_run_operation_status(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(
            TheurgyError::new("usage: run-operation-status OPERATION_ID --manifest PATH").into(),
        );
    }
    let operation_id = &args[0];
    if operation_id.is_empty() {
        return Err(TheurgyError::new("run-operation-status OPERATION_ID required").into());
    }
    let mut manifest_path: Option<PathBuf> = None;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--manifest" => {
                let raw = args.get(index + 1).ok_or_else(|| {
                    TheurgyError::new("run-operation-status --manifest requires PATH")
                })?;
                manifest_path = Some(PathBuf::from(raw));
                index += 2;
            }
            other => {
                return Err(TheurgyError::new(format!(
                    "unknown run-operation-status option: {other}"
                ))
                .into())
            }
        }
    }
    let manifest_path = manifest_path
        .ok_or_else(|| TheurgyError::new("run-operation-status --manifest PATH required"))?;
    let output = run_operation_status_output(&manifest_path, operation_id)?;
    print!("{output}");
    Ok(())
}

fn parse_subscribe_status_args(args: &[String]) -> Result<PathBuf> {
    let mut manifest_path: Option<PathBuf> = None;
    let mut once = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--manifest" => {
                let raw = args.get(index + 1).ok_or_else(|| {
                    TheurgyError::new("subscribe-status --manifest requires PATH")
                })?;
                manifest_path = Some(PathBuf::from(raw));
                index += 2;
            }
            "--once" => {
                once = true;
                index += 1;
            }
            other => {
                return Err(
                    TheurgyError::new(format!("unknown subscribe-status option: {other}")).into(),
                )
            }
        }
    }
    if !once {
        return Err(TheurgyError::new("usage: subscribe-status --manifest PATH --once").into());
    }
    manifest_path
        .ok_or_else(|| TheurgyError::new("subscribe-status --manifest PATH required").into())
}

fn command_run_history(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(TheurgyError::new("usage: run-history SUBJECT [LIMIT] --manifest PATH").into());
    }
    let subject = &args[0];
    if subject.is_empty() {
        return Err(TheurgyError::new("run-history SUBJECT required").into());
    }
    let mut limit: Option<String> = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--manifest" => {
                let raw = args
                    .get(index + 1)
                    .ok_or_else(|| TheurgyError::new("run-history --manifest requires PATH"))?;
                manifest_path = Some(PathBuf::from(raw));
                index += 2;
            }
            other if limit.is_none() => {
                if other.is_empty() || !other.bytes().all(|byte| byte.is_ascii_digit()) {
                    return Err(
                        TheurgyError::new("run-history LIMIT must be a positive integer").into(),
                    );
                }
                limit = Some(other.to_string());
                index += 1;
            }
            other => {
                return Err(
                    TheurgyError::new(format!("unknown run-history option: {other}")).into(),
                )
            }
        }
    }
    let manifest_path =
        manifest_path.ok_or_else(|| TheurgyError::new("run-history --manifest PATH required"))?;
    let output = run_history_output(&manifest_path, subject, limit.as_deref())?;
    print!("{output}");
    Ok(())
}

fn command_run_action(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(TheurgyError::new(
            "usage: run-action ACTION_ID [--json PARAMS] [--manifest PATH]",
        )
        .into());
    }
    let action_id = &args[0];
    if !valid_action_id(action_id) {
        return Err(TheurgyError::new("run-action ACTION_ID must be a stable action id").into());
    }
    let mut params = "{}".to_string();
    let mut manifest_path: Option<PathBuf> = None;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                let raw = args
                    .get(index + 1)
                    .ok_or_else(|| TheurgyError::new("run-action --json requires PARAMS"))?;
                validate_json_params(raw)?;
                params = raw.to_string();
                index += 2;
            }
            "--manifest" => {
                let raw = args
                    .get(index + 1)
                    .ok_or_else(|| TheurgyError::new("run-action --manifest requires PATH"))?;
                manifest_path = Some(PathBuf::from(raw));
                index += 2;
            }
            other => {
                return Err(TheurgyError::new(format!("unknown run-action option: {other}")).into())
            }
        }
    }
    let output = run_action_output(action_id, &params, manifest_path.as_deref())?;
    print!("{output}");
    Ok(())
}

fn parse_manifest_only_args(args: &[String], usage: &str) -> Result<PathBuf> {
    if args.len() != 2 || args.first().map(String::as_str) != Some("--manifest") {
        return Err(TheurgyError::new(usage).into());
    }
    Ok(PathBuf::from(&args[1]))
}

fn run_state_output(manifest_path: &Path) -> Result<String> {
    let runtime = runtime_contract_from_path(manifest_path)?;
    let output = run_manifest_command(&runtime.state_command, "state")?;
    validate_manifest_state_output(&runtime.app_id, &output)?;
    Ok(output)
}

fn run_status_output(manifest_path: &Path) -> Result<String> {
    let runtime = runtime_contract_from_path(manifest_path)?;
    if runtime.status_command.is_empty() {
        return Err(TheurgyError::new("runtime manifest statusCommand required").into());
    }
    let output = run_manifest_command(&runtime.status_command, "status")?;
    validate_manifest_status_output(&runtime.app_id, &output)?;
    Ok(output)
}

fn run_operation_status_output(manifest_path: &Path, operation_id: &str) -> Result<String> {
    let runtime = runtime_contract_from_path(manifest_path)?;
    if runtime.operation_status_command.is_empty() {
        return Err(TheurgyError::new("runtime manifest operationStatusCommand required").into());
    }
    let output = run_manifest_command_with_args(
        &runtime.operation_status_command,
        &[operation_id.to_string()],
        "operation status",
    )?;
    validate_manifest_operation_status_output(&runtime.app_id, &output)?;
    Ok(output)
}

fn subscribe_status_output(manifest_path: &Path) -> Result<String> {
    let runtime = runtime_contract_from_path(manifest_path)?;
    let command = if runtime.subscribe_status_command.is_empty() {
        &runtime.status_command
    } else {
        &runtime.subscribe_status_command
    };
    if command.is_empty() {
        return Err(TheurgyError::new(
            "runtime manifest subscribeStatusCommand or statusCommand required",
        )
        .into());
    }
    let output = run_manifest_command(command, "status")?;
    validate_manifest_status_output(&runtime.app_id, &output)?;
    Ok(output)
}

fn run_history_output(manifest_path: &Path, subject: &str, limit: Option<&str>) -> Result<String> {
    let runtime = runtime_contract_from_path(manifest_path)?;
    if runtime.history_command.is_empty() {
        return Err(TheurgyError::new("runtime manifest historyCommand required").into());
    }
    let mut args = vec![subject.to_string()];
    if let Some(limit) = limit {
        args.push(limit.to_string());
    }
    let output = run_manifest_command_with_args(&runtime.history_command, &args, "history")?;
    validate_manifest_history_output(&runtime.app_id, &output)?;
    Ok(output)
}

fn run_action_output(
    action_id: &str,
    params: &str,
    manifest_path: Option<&Path>,
) -> Result<String> {
    validate_json_params(params)?;
    if let Some(path) = manifest_path {
        let runtime = runtime_contract_from_path_with_product_actions(path)?;
        return run_manifest_action(&runtime, action_id, params);
    }
    Ok(format!(
        "{{\n  \"success\": true,\n  \"protocol\": \"{}\",\n  \"app\": \"theurgy-runtime\",\n  \"action\": \"{}\",\n  \"operation\": {{\n    \"id\": \"op-{}\",\n    \"status\": \"accepted\",\n    \"progress\": 0,\n    \"longRunning\": false\n  }},\n  \"params\": {}\n}}",
        product_runtime::RUNTIME_ACTION_PROTOCOL,
        json_escape(action_id),
        json_escape(action_id),
        params
    ))
}

fn run_manifest_action(runtime: &RuntimeContract, action_id: &str, params: &str) -> Result<String> {
    if runtime.action_command.is_empty() {
        return Err(TheurgyError::new("runtime manifest actionCommand required").into());
    }
    if let Some(action_ids) = &runtime.product_action_ids {
        if !action_ids.iter().any(|declared| declared == action_id) {
            return Err(TheurgyError::new(format!(
                "runtime action not declared in Product IR: {action_id}"
            ))
            .into());
        }
    }
    if let Some(contracts) = &runtime.product_action_contracts {
        validate_runtime_action_params(action_id, params, contracts)?;
    }
    let output = run_manifest_action_command(
        &runtime.action_command,
        &[action_id.to_string(), params.to_string()],
        action_id,
        runtime.product_action_contracts.as_deref(),
    )?;
    validate_manifest_action_output(
        &runtime.app_id,
        action_id,
        &output,
        runtime.product_action_contracts.as_deref(),
    )?;
    Ok(output)
}

fn run_manifest_action_command(
    command: &[String],
    extra_args: &[String],
    action_id: &str,
    contracts: Option<&[ActionContract]>,
) -> Result<String> {
    let Some(executable) = command.first() else {
        return Err(TheurgyError::new("runtime manifest action command required").into());
    };
    let output = Command::new(executable)
        .args(&command[1..])
        .args(extra_args)
        .output()
        .map_err(|error| TheurgyError::new(format!("could not run action command: {error}")))?;
    let stdout = String::from_utf8(output.stdout).map_err(|error| {
        TheurgyError::new(format!("action command output was not UTF-8: {error}"))
    })?;
    if !output.status.success() {
        if let Some(contracts) = contracts {
            validate_runtime_action_failure_keys(action_id, stdout.trim(), contracts)?;
        }
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let message = structured_failure_message(&stdout)
            .or_else(|| (!stderr.is_empty()).then_some(stderr))
            .unwrap_or_else(|| format!("action command exited with {}", output.status));
        return Err(TheurgyError::new(message).into());
    }
    Ok(stdout)
}

fn validate_manifest_action_output(
    expected_app: &str,
    action_id: &str,
    output: &str,
    contracts: Option<&[ActionContract]>,
) -> Result<()> {
    let value = parse_json(output)?;
    let result = manifest_payload_or_raw(&value);
    let summary = validate_runtime_action_result_value(result)?;
    if summary.action_id != action_id {
        return Err(TheurgyError::new(format!(
            "runtime action result action mismatch: expected {action_id}, got {}",
            summary.action_id
        ))
        .into());
    }
    if let Some(contracts) = contracts {
        validate_runtime_action_operation_contract(action_id, summary.long_running, contracts)?;
        validate_runtime_action_result_keys(action_id, result, contracts)?;
    }
    validate_runtime_output_app("runtime action result", expected_app, &summary.app_id)
}

fn validate_manifest_state_output(expected_app: &str, output: &str) -> Result<()> {
    let value = parse_json(output)?;
    let result = manifest_payload_or_raw(&value);
    let summary = validate_state_snapshot_value(result)?;
    validate_runtime_output_app("state snapshot", expected_app, &summary.app_id)
}

fn validate_manifest_status_output(expected_app: &str, output: &str) -> Result<()> {
    let value = parse_json(output)?;
    let result = manifest_payload_or_raw(&value);
    let summary = validate_runtime_status_value(result)?;
    validate_runtime_output_app("runtime status", expected_app, &summary.app_id)
}

fn validate_manifest_operation_status_output(expected_app: &str, output: &str) -> Result<()> {
    let value = parse_json(output)?;
    let result = manifest_payload_or_raw(&value);
    let summary = validate_operation_status_value(result)?;
    validate_runtime_output_app("operation status", expected_app, &summary.app_id)
}

fn validate_manifest_history_output(expected_app: &str, output: &str) -> Result<()> {
    let value = parse_json(output)?;
    let result = manifest_payload_or_raw(&value);
    let summary = validate_operation_history_value(result)?;
    validate_runtime_output_app("operation history", expected_app, &summary.app_id)
}

fn validate_runtime_output_app(label: &str, expected_app: &str, actual_app: &str) -> Result<()> {
    if actual_app != expected_app {
        return Err(TheurgyError::new(format!(
            "{label} app mismatch: expected {expected_app}, got {actual_app}"
        ))
        .into());
    }
    Ok(())
}

fn manifest_payload_or_raw(value: &Value) -> &Value {
    if value.get("success").is_some() {
        if let Some(data) = value.get("data") {
            return data;
        }
    }
    value
}

fn validate_runtime_action_params(
    action_id: &str,
    params: &str,
    contracts: &[ActionContract],
) -> Result<()> {
    let contracts = product_action_contracts_from_cli(contracts);
    product_runtime::validate_runtime_action_params_text(action_id, params, &contracts)
        .map_err(Into::into)
}

fn validate_runtime_action_result_keys(
    action_id: &str,
    result: &Value,
    contracts: &[ActionContract],
) -> Result<()> {
    let contracts = product_action_contracts_from_cli(contracts);
    product_runtime::validate_runtime_action_result_contract_value(action_id, result, &contracts)
        .map_err(Into::into)
}

fn validate_runtime_action_operation_contract(
    action_id: &str,
    actual_long_running: bool,
    contracts: &[ActionContract],
) -> Result<()> {
    let contracts = product_action_contracts_from_cli(contracts);
    product_runtime::validate_runtime_action_operation_contract(
        action_id,
        actual_long_running,
        &contracts,
    )
    .map_err(Into::into)
}

fn validate_runtime_action_failure_keys(
    action_id: &str,
    output: &str,
    contracts: &[ActionContract],
) -> Result<()> {
    let contracts = product_action_contracts_from_cli(contracts);
    product_runtime::validate_runtime_action_failure_contract_text(action_id, output, &contracts)
        .map_err(Into::into)
}

fn structured_failure_message(output: &str) -> Option<String> {
    let value = parse_json(output).ok()?;
    if value.get("success").and_then(Value::as_bool) != Some(false) {
        return None;
    }
    value
        .get("error")
        .and_then(Value::as_str)
        .filter(|error| !error.is_empty())
        .map(String::from)
}

fn runtime_contract_from_path(path: &Path) -> Result<RuntimeContract> {
    let manifest = fs::read_to_string(path).map_err(|error| {
        TheurgyError::new(format!("could not read {}: {error}", path.display()))
    })?;
    runtime_contract_from_manifest(&manifest)
}

fn runtime_contract_from_path_with_product_actions(path: &Path) -> Result<RuntimeContract> {
    let manifest = fs::read_to_string(path).map_err(|error| {
        TheurgyError::new(format!("could not read {}: {error}", path.display()))
    })?;
    let summary = validate_runtime_manifest(&manifest)?;
    let mut runtime = runtime_contract_from_manifest(&manifest)?;
    let product_path = manifest_relative_path(path, &summary.product_ir);
    let product_text = fs::read_to_string(&product_path).map_err(|error| {
        TheurgyError::new(format!(
            "could not read {}: {error}",
            product_path.display()
        ))
    })?;
    let product = validate_product_ir(&product_text)?;
    if product.app_id != runtime.app_id {
        return Err(TheurgyError::new(format!(
            "runtime manifest app {} does not match Product IR app {}",
            runtime.app_id, product.app_id
        ))
        .into());
    }
    runtime.product_action_ids = Some(product.action_ids);
    runtime.product_action_contracts = Some(product.action_contracts);
    Ok(runtime)
}

fn manifest_relative_path(manifest_path: &Path, relative_or_absolute: &str) -> PathBuf {
    let path = PathBuf::from(relative_or_absolute);
    if path.is_absolute() {
        return path;
    }
    let manifest_relative = manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(&path);
    if manifest_relative.exists() {
        return manifest_relative;
    }
    path
}

fn run_manifest_command(command: &[String], label: &str) -> Result<String> {
    run_manifest_command_with_args(command, &[], label)
}

fn run_manifest_command_with_args(
    command: &[String],
    extra_args: &[String],
    label: &str,
) -> Result<String> {
    let Some(executable) = command.first() else {
        return Err(TheurgyError::new(format!("runtime manifest {label} command required")).into());
    };
    let output = Command::new(executable)
        .args(&command[1..])
        .args(extra_args)
        .output()
        .map_err(|error| TheurgyError::new(format!("could not run {label} command: {error}")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let message = if stderr.is_empty() {
            format!("{label} command exited with {}", output.status)
        } else {
            stderr
        };
        return Err(TheurgyError::new(message).into());
    }
    String::from_utf8(output.stdout).map_err(|error| {
        TheurgyError::new(format!("{label} command output was not UTF-8: {error}")).into()
    })
}

#[derive(Debug, Eq, PartialEq)]
struct ProductSummary {
    app_id: String,
    app_name: String,
    targets: Vec<String>,
    desktop_surface_ir: Option<String>,
    mobile_surface_ir: Option<String>,
    capabilities: Vec<String>,
    permissions: Vec<String>,
    domain_object_ids: Vec<String>,
    state_snapshot_schema: String,
    state_command: Vec<String>,
    state_status_command: Vec<String>,
    persistence_truth: String,
    persistence_root_ids: Vec<String>,
    background_jobs: Vec<BackgroundJob>,
    background_job_ids: Vec<String>,
    release_targets: Vec<ReleaseTarget>,
    audit_keys: Vec<String>,
    action_contracts: Vec<ActionContract>,
    action_ids: Vec<String>,
    actions: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ActionContract {
    id: String,
    label: String,
    effect: String,
    safe: bool,
    mutating: bool,
    long_running: bool,
    privileged: bool,
    command: Vec<String>,
    input_keys: Vec<String>,
    output_keys: Vec<String>,
    failure_keys: Vec<String>,
    input_shape: BTreeMap<String, String>,
    output_shape: BTreeMap<String, String>,
    failure_shape: BTreeMap<String, String>,
}

#[derive(Debug, Eq, PartialEq)]
struct ActionSummary {
    action_ids: Vec<String>,
    actions: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BackgroundJob {
    id: String,
    command: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
struct StateSnapshotSummary {
    app_id: String,
}

#[derive(Debug, Eq, PartialEq)]
struct RuntimeStatusSummary {
    app_id: String,
}

#[derive(Debug, Eq, PartialEq)]
struct RuntimeActionRequestSummary {
    app_id: String,
    action_id: String,
}

#[derive(Debug, Eq, PartialEq)]
struct RuntimeActionResultSummary {
    app_id: String,
    action_id: String,
    operation_id: String,
    long_running: bool,
}

#[derive(Debug, Eq, PartialEq)]
struct OperationStatusSummary {
    app_id: String,
    operation_id: String,
    long_running: bool,
}

#[derive(Debug, Eq, PartialEq)]
struct OperationHistorySummary {
    app_id: String,
    entries: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct RuntimeManifestSummary {
    app_id: String,
    product_ir: String,
    desktop_surface_ir: Option<String>,
    mobile_surface_ir: Option<String>,
    legacy_native_desktop_ir: Option<String>,
    protocol: String,
    compatibility: RuntimeCompatibility,
}

type GeneratedRuntimeSummary = product_runtime::GeneratedRuntime;

#[derive(Debug, Eq, PartialEq)]
struct SurfaceSummary {
    schema: String,
    product: String,
    target: String,
    action_ids: Vec<String>,
    roles: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
struct RuntimeContract {
    app_id: String,
    protocol: String,
    product_ir: String,
    runtime_manifest: String,
    source_surface_ir: String,
    legacy_native_desktop_ir: Option<String>,
    compatibility: RuntimeCompatibility,
    state_command: Vec<String>,
    status_command: Vec<String>,
    subscribe_status_command: Vec<String>,
    operation_status_command: Vec<String>,
    action_command: Vec<String>,
    history_command: Vec<String>,
    daemon_command: Vec<String>,
    product_action_ids: Option<Vec<String>>,
    product_action_contracts: Option<Vec<ActionContract>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RuntimeCompatibility {
    wizardry_apps_shell_first_still_supported: bool,
    theurgy_required_for_legacy_wizardry_apps: bool,
}

impl RuntimeCompatibility {
    fn shell_first_default() -> Self {
        Self {
            wizardry_apps_shell_first_still_supported: true,
            theurgy_required_for_legacy_wizardry_apps: false,
        }
    }
}

impl RuntimeContract {
    fn with_sources(
        mut self,
        product_ir: String,
        runtime_manifest: String,
        source_surface_ir: String,
    ) -> Self {
        self.product_ir = product_ir;
        self.runtime_manifest = runtime_manifest;
        self.source_surface_ir = source_surface_ir;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReleaseTarget {
    id: String,
    target: String,
    surface: String,
    artifact: String,
}

fn read_json(path: &Path) -> Result<String> {
    let text = fs::read_to_string(path).map_err(|error| {
        TheurgyError::new(format!("could not read {}: {error}", path.display()))
    })?;
    validate_json_params(&text)?;
    Ok(text)
}

fn validate_runtime_manifest(text: &str) -> Result<RuntimeManifestSummary> {
    let value = parse_json(text)?;
    validate_runtime_manifest_value(&value)
}

fn validate_generated_runtime(text: &str) -> Result<GeneratedRuntimeSummary> {
    product_runtime::validate_generated_runtime_text(text).map_err(Into::into)
}

fn runtime_contract_from_manifest(text: &str) -> Result<RuntimeContract> {
    let bridge = product_runtime::runtime_bridge_from_manifest_text(text)?;
    Ok(RuntimeContract {
        app_id: bridge.app_id,
        protocol: bridge.protocol,
        product_ir: bridge.product_ir,
        runtime_manifest: bridge.runtime_manifest,
        source_surface_ir: bridge.source_surface_ir,
        legacy_native_desktop_ir: bridge.legacy_native_desktop_ir,
        compatibility: RuntimeCompatibility {
            wizardry_apps_shell_first_still_supported: bridge
                .compatibility
                .wizardry_apps_shell_first_still_supported,
            theurgy_required_for_legacy_wizardry_apps: bridge
                .compatibility
                .theurgy_required_for_legacy_wizardry_apps,
        },
        state_command: bridge.state_command,
        status_command: bridge.status_command,
        subscribe_status_command: bridge.subscribe_status_command,
        operation_status_command: bridge.operation_status_command,
        action_command: bridge.action_command,
        history_command: bridge.history_command,
        daemon_command: bridge.daemon_command,
        product_action_ids: None,
        product_action_contracts: None,
    })
}

fn validate_surface_ir(text: &str) -> Result<SurfaceSummary> {
    let value = parse_json(text)?;
    let surface = product_runtime::validate_surface_ir_value(&value)?;
    Ok(surface_summary_from_library(surface))
}

fn surface_summary_from_library(surface: product_runtime::SurfaceIr) -> SurfaceSummary {
    SurfaceSummary {
        schema: surface.schema,
        product: surface.product,
        target: surface.target,
        action_ids: surface.action_ids,
        roles: surface.roles,
    }
}

fn validate_product_ir(text: &str) -> Result<ProductSummary> {
    let value = parse_json(text)?;
    validate_product_ir_value(&value)
}

fn validate_action_ir(text: &str) -> Result<ActionSummary> {
    let value = parse_json(text)?;
    let action_ir = product_runtime::validate_action_ir_value(&value)?;
    Ok(ActionSummary {
        actions: action_ir.actions,
        action_ids: action_ir.action_ids,
    })
}

fn validate_state_snapshot(text: &str) -> Result<StateSnapshotSummary> {
    let value = parse_json(text)?;
    validate_state_snapshot_value(&value)
}

fn validate_runtime_status(text: &str) -> Result<RuntimeStatusSummary> {
    let value = parse_json(text)?;
    validate_runtime_status_value(&value)
}

fn validate_state_snapshot_value(value: &Value) -> Result<StateSnapshotSummary> {
    let snapshot = product_runtime::validate_state_snapshot_value(value)?;
    Ok(StateSnapshotSummary {
        app_id: snapshot.app_id,
    })
}

fn validate_runtime_status_value(value: &Value) -> Result<RuntimeStatusSummary> {
    let status = product_runtime::validate_runtime_status_value(value)?;
    Ok(RuntimeStatusSummary {
        app_id: status.app_id,
    })
}

fn validate_runtime_action_result(text: &str) -> Result<RuntimeActionResultSummary> {
    let value = parse_json(text)?;
    validate_runtime_action_result_value(&value)
}

fn validate_runtime_action_request(text: &str) -> Result<RuntimeActionRequestSummary> {
    let value = parse_json(text)?;
    validate_runtime_action_request_value(&value)
}

fn validate_runtime_action_request_value(value: &Value) -> Result<RuntimeActionRequestSummary> {
    let request = product_runtime::validate_runtime_action_request_value(value)?;
    Ok(RuntimeActionRequestSummary {
        app_id: request.app_id,
        action_id: request.action_id,
    })
}

fn validate_runtime_action_request_against_runtime(
    summary: &RuntimeActionRequestSummary,
    value: &Value,
    runtime: &RuntimeContract,
) -> Result<()> {
    validate_runtime_output_app("runtime action request", &runtime.app_id, &summary.app_id)?;
    if let Some(action_ids) = &runtime.product_action_ids {
        if !action_ids
            .iter()
            .any(|declared| declared == &summary.action_id)
        {
            return Err(TheurgyError::new(format!(
                "runtime action request not declared in Product IR: {}",
                summary.action_id
            ))
            .into());
        }
    }
    if let Some(contracts) = &runtime.product_action_contracts {
        let params = serde_json::to_string(value_object(value, "params")?)
            .map_err(|error| TheurgyError::new(format!("could not serialize params: {error}")))?;
        validate_runtime_action_params(&summary.action_id, &params, contracts)?;
    }
    Ok(())
}

fn validate_runtime_action_result_value(value: &Value) -> Result<RuntimeActionResultSummary> {
    let result = product_runtime::validate_runtime_action_result_value(value)?;
    Ok(RuntimeActionResultSummary {
        app_id: result.app_id,
        action_id: result.action_id,
        operation_id: result.operation_id,
        long_running: result.long_running,
    })
}

fn validate_operation_status(text: &str) -> Result<OperationStatusSummary> {
    let value = parse_json(text)?;
    validate_operation_status_value(&value)
}

fn validate_operation_status_value(value: &Value) -> Result<OperationStatusSummary> {
    let status = product_runtime::validate_operation_status_value(value)?;
    Ok(OperationStatusSummary {
        app_id: status.app_id,
        operation_id: status.operation_id,
        long_running: status.long_running,
    })
}

fn validate_operation_history(text: &str) -> Result<OperationHistorySummary> {
    let value = parse_json(text)?;
    validate_operation_history_value(&value)
}

fn validate_operation_history_value(value: &Value) -> Result<OperationHistorySummary> {
    let history = product_runtime::validate_operation_history_value(value)?;
    Ok(OperationHistorySummary {
        app_id: history.app_id,
        entries: history.entries,
    })
}

fn validate_product_ir_value(value: &Value) -> Result<ProductSummary> {
    let product = product_runtime::validate_product_ir_value(value)?;
    Ok(product_summary_from_library(product))
}

fn product_summary_from_library(product: product_runtime::ProductIr) -> ProductSummary {
    ProductSummary {
        app_id: product.app_id,
        app_name: product.app_name,
        targets: product.targets,
        desktop_surface_ir: product.desktop_surface_ir,
        mobile_surface_ir: product.mobile_surface_ir,
        capabilities: product.capabilities,
        permissions: product.permissions,
        domain_object_ids: product.domain_object_ids,
        state_snapshot_schema: product.state_snapshot_schema,
        state_command: product.state_command,
        state_status_command: product.state_status_command,
        persistence_truth: product.persistence_truth,
        persistence_root_ids: product.persistence_root_ids,
        background_jobs: product
            .background_jobs
            .into_iter()
            .map(|job| BackgroundJob {
                id: job.id,
                command: job.command,
            })
            .collect(),
        background_job_ids: product.background_job_ids,
        release_targets: product
            .release_targets
            .into_iter()
            .map(|target| ReleaseTarget {
                id: target.id,
                target: target.target,
                surface: target.surface,
                artifact: target.artifact,
            })
            .collect(),
        audit_keys: product.audit_keys,
        action_contracts: product
            .action_contracts
            .into_iter()
            .map(action_contract_from_library)
            .collect(),
        action_ids: product.action_ids,
        actions: product.actions,
    }
}

fn action_contract_from_library(
    contract: product_runtime::ProductActionContract,
) -> ActionContract {
    ActionContract {
        id: contract.id,
        label: contract.label,
        effect: contract.effect,
        safe: contract.safe,
        mutating: contract.mutating,
        long_running: contract.long_running,
        privileged: contract.privileged,
        command: contract.command,
        input_keys: contract.input_keys,
        output_keys: contract.output_keys,
        failure_keys: contract.failure_keys,
        input_shape: contract.input_shape,
        output_shape: contract.output_shape,
        failure_shape: contract.failure_shape,
    }
}

fn product_action_contracts_from_cli(
    contracts: &[ActionContract],
) -> Vec<product_runtime::ProductActionContract> {
    contracts
        .iter()
        .map(|contract| product_runtime::ProductActionContract {
            id: contract.id.clone(),
            label: contract.label.clone(),
            effect: contract.effect.clone(),
            safe: contract.safe,
            mutating: contract.mutating,
            long_running: contract.long_running,
            privileged: contract.privileged,
            command: contract.command.clone(),
            input_keys: contract.input_keys.clone(),
            output_keys: contract.output_keys.clone(),
            failure_keys: contract.failure_keys.clone(),
            input_shape: contract.input_shape.clone(),
            output_shape: contract.output_shape.clone(),
            failure_shape: contract.failure_shape.clone(),
        })
        .collect()
}

fn product_ir_from_summary(summary: &ProductSummary) -> product_runtime::ProductIr {
    product_runtime::ProductIr {
        app_id: summary.app_id.clone(),
        app_name: summary.app_name.clone(),
        targets: summary.targets.clone(),
        desktop_surface_ir: summary.desktop_surface_ir.clone(),
        mobile_surface_ir: summary.mobile_surface_ir.clone(),
        capabilities: summary.capabilities.clone(),
        permissions: summary.permissions.clone(),
        domain_object_ids: summary.domain_object_ids.clone(),
        state_snapshot_schema: summary.state_snapshot_schema.clone(),
        state_command: summary.state_command.clone(),
        state_status_command: summary.state_status_command.clone(),
        persistence_truth: summary.persistence_truth.clone(),
        persistence_root_ids: summary.persistence_root_ids.clone(),
        background_jobs: summary
            .background_jobs
            .iter()
            .map(|job| product_runtime::BackgroundJob {
                id: job.id.clone(),
                command: job.command.clone(),
            })
            .collect(),
        background_job_ids: summary.background_job_ids.clone(),
        release_targets: summary
            .release_targets
            .iter()
            .map(|target| product_runtime::ReleaseTarget {
                id: target.id.clone(),
                target: target.target.clone(),
                surface: target.surface.clone(),
                artifact: target.artifact.clone(),
            })
            .collect(),
        audit_keys: summary.audit_keys.clone(),
        action_contracts: product_action_contracts_from_cli(&summary.action_contracts),
        action_ids: summary.action_ids.clone(),
        actions: summary.actions,
    }
}

fn surface_ir_from_summary(surface: &SurfaceSummary) -> product_runtime::SurfaceIr {
    product_runtime::SurfaceIr {
        schema: surface.schema.clone(),
        product: surface.product.clone(),
        target: surface.target.clone(),
        action_ids: surface.action_ids.clone(),
        roles: surface.roles.clone(),
    }
}

fn runtime_bridge_from_contract(runtime: &RuntimeContract) -> product_runtime::RuntimeBridge {
    product_runtime::RuntimeBridge {
        app_id: runtime.app_id.clone(),
        protocol: runtime.protocol.clone(),
        product_ir: runtime.product_ir.clone(),
        runtime_manifest: runtime.runtime_manifest.clone(),
        source_surface_ir: runtime.source_surface_ir.clone(),
        legacy_native_desktop_ir: runtime.legacy_native_desktop_ir.clone(),
        compatibility: product_runtime::RuntimeCompatibility {
            wizardry_apps_shell_first_still_supported: runtime
                .compatibility
                .wizardry_apps_shell_first_still_supported,
            theurgy_required_for_legacy_wizardry_apps: runtime
                .compatibility
                .theurgy_required_for_legacy_wizardry_apps,
        },
        state_command: runtime.state_command.clone(),
        status_command: runtime.status_command.clone(),
        subscribe_status_command: runtime.subscribe_status_command.clone(),
        operation_status_command: runtime.operation_status_command.clone(),
        action_command: runtime.action_command.clone(),
        history_command: runtime.history_command.clone(),
        daemon_command: runtime.daemon_command.clone(),
    }
}

fn validate_runtime_manifest_value(value: &Value) -> Result<RuntimeManifestSummary> {
    let manifest = product_runtime::validate_runtime_manifest_value(value)?;
    Ok(runtime_manifest_summary_from_library(manifest))
}

fn runtime_manifest_summary_from_library(
    manifest: product_runtime::RuntimeManifest,
) -> RuntimeManifestSummary {
    RuntimeManifestSummary {
        app_id: manifest.app_id,
        product_ir: manifest.product_ir,
        desktop_surface_ir: manifest.desktop_surface_ir,
        mobile_surface_ir: manifest.mobile_surface_ir,
        legacy_native_desktop_ir: manifest.legacy_native_desktop_ir,
        protocol: manifest.protocol,
        compatibility: RuntimeCompatibility {
            wizardry_apps_shell_first_still_supported: manifest
                .compatibility
                .wizardry_apps_shell_first_still_supported,
            theurgy_required_for_legacy_wizardry_apps: manifest
                .compatibility
                .theurgy_required_for_legacy_wizardry_apps,
        },
    }
}

fn parse_json(text: &str) -> Result<Value> {
    serde_json::from_str(text)
        .map_err(|error| TheurgyError::new(format!("invalid JSON: {error}")).into())
}

fn validate_json_params(raw: &str) -> Result<()> {
    let value = parse_json(raw)?;
    if value.is_object() || value.is_array() {
        Ok(())
    } else {
        Err(TheurgyError::new("expected a JSON object or array literal").into())
    }
}

fn value_object<'a>(value: &'a Value, key: &str) -> Result<&'a Value> {
    value
        .get(key)
        .filter(|candidate| candidate.is_object())
        .ok_or_else(|| TheurgyError::new(format!("missing JSON object key: {key}")).into())
}

fn action_contracts_value(contracts: &[ActionContract]) -> Value {
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
        .map(|(key, type_name)| {
            format!("\"{}\": \"{}\"", swift_escape(key), swift_escape(type_name))
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{items}]")
}

fn swift_action_contracts_literal(contracts: &[ActionContract]) -> String {
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

fn java_string_array_literal(values: &[String]) -> String {
    let items = values
        .iter()
        .map(|value| format!("\"{}\"", java_escape(value)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("new String[] {{{items}}}")
}

fn java_shape_literal(shape: &BTreeMap<String, String>) -> String {
    let items = shape
        .iter()
        .map(|(key, type_name)| {
            format!(
                "{{\"{}\", \"{}\"}}",
                java_escape(key),
                java_escape(type_name)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("new String[][] {{{items}}}")
}

fn java_action_contracts_literal(contracts: &[ActionContract]) -> String {
    let items = contracts
        .iter()
        .map(|contract| {
            format!(
                "new ProductActionContract(\"{}\", \"{}\", \"{}\", {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                java_escape(&contract.id),
                java_escape(&contract.label),
                java_escape(&contract.effect),
                java_bool(contract.safe),
                java_bool(contract.mutating),
                java_bool(contract.long_running),
                java_bool(contract.privileged),
                java_string_array_literal(&contract.input_keys),
                java_string_array_literal(&contract.output_keys),
                java_string_array_literal(&contract.failure_keys),
                java_shape_literal(&contract.input_shape),
                java_shape_literal(&contract.output_shape),
                java_shape_literal(&contract.failure_shape)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("new ProductActionContract[] {{{items}}}")
}

fn java_bool(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn valid_action_id(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_' | b'.')
        })
}

fn parse_product_target_args<'a>(args: &'a [String], usage: &str) -> Result<(&'a Path, &'a str)> {
    if args.len() != 3 || args.get(1).map(String::as_str) != Some("--target") {
        return Err(TheurgyError::new(usage).into());
    }
    let target = args[2].as_str();
    if !matches!(target, "macos" | "linux" | "ios" | "android") {
        return Err(TheurgyError::new("target must be macos, linux, ios, or android").into());
    }
    Ok((Path::new(&args[0]), target))
}

fn parse_compile_args<'a>(args: &'a [String]) -> Result<(&'a Path, &'a str, &'a Path)> {
    if args.len() != 5
        || args.get(1).map(String::as_str) != Some("--target")
        || args.get(3).map(String::as_str) != Some("--out")
    {
        return Err(TheurgyError::new("usage: compile-native PRODUCT_IR --target TARGET --out OUT_DIR or compile-app APP_DIR --target TARGET --out OUT_DIR").into());
    }
    let target = args[2].as_str();
    if !matches!(target, "macos" | "linux" | "ios" | "android") {
        return Err(TheurgyError::new("target must be macos, linux, ios, or android").into());
    }
    Ok((Path::new(&args[0]), target, Path::new(&args[4])))
}

fn project_surface(product: &str, target: &str) -> Result<String> {
    product_runtime::project_surface_from_product_text(product, target).map_err(Into::into)
}

fn compile_native(product: &str, target: &str, out_dir: &Path) -> Result<()> {
    let summary = validate_product_ir(product)?;
    let surface = project_surface(product, target)?;
    let runtime = RuntimeContract {
        app_id: summary.app_id.clone(),
        protocol: product_runtime::RUNTIME_ACTION_PROTOCOL.to_string(),
        product_ir: "direct-product-ir".to_string(),
        runtime_manifest: "generated-runtime-manifest".to_string(),
        source_surface_ir: "projected-surface-ir".to_string(),
        legacy_native_desktop_ir: None,
        compatibility: RuntimeCompatibility::shell_first_default(),
        state_command: vec![
            format!("{}-core", summary.app_id),
            "runtime-state".to_string(),
        ],
        status_command: vec![
            format!("{}-core", summary.app_id),
            "runtime-status".to_string(),
        ],
        subscribe_status_command: Vec::new(),
        operation_status_command: vec![
            format!("{}-core", summary.app_id),
            "runtime-operation-status".to_string(),
        ],
        action_command: vec![
            format!("{}-core", summary.app_id),
            "runtime-action".to_string(),
        ],
        history_command: Vec::new(),
        daemon_command: Vec::new(),
        product_action_ids: Some(summary.action_ids.clone()),
        product_action_contracts: Some(summary.action_contracts.clone()),
    };
    compile_native_with_contract(&summary, &surface, &runtime, target, out_dir, false)
}

fn compile_native_with_contract(
    summary: &ProductSummary,
    surface: &str,
    runtime: &RuntimeContract,
    target: &str,
    out_dir: &Path,
    preserve_existing_legacy_desktop_adapter: bool,
) -> Result<()> {
    if !summary
        .targets
        .iter()
        .any(|candidate| candidate.as_str() == target)
    {
        return Err(
            TheurgyError::new(format!("product IR does not declare target: {target}")).into(),
        );
    }
    let release_target = release_target_for_target(summary, target).ok_or_else(|| {
        TheurgyError::new(format!(
            "product IR release target missing for target: {target}"
        ))
    })?;
    let surface_summary = validate_surface_ir(surface)?;
    if surface_summary.product != summary.app_id {
        return Err(TheurgyError::new("surface IR product does not match product IR app").into());
    }
    let expected_surface_target = product_runtime::surface_family_for_target(target)
        .ok_or_else(|| TheurgyError::new("unsupported target"))?;
    let expected_surface_schema = product_runtime::surface_schema_for_target(target)
        .ok_or_else(|| TheurgyError::new("unsupported target"))?;
    if release_target.surface != expected_surface_target {
        return Err(TheurgyError::new(format!(
            "product IR release target surface for {target} must be {expected_surface_target}"
        ))
        .into());
    }
    if surface_summary.schema != expected_surface_schema {
        return Err(TheurgyError::new(format!(
            "surface IR schema for {target} must be {expected_surface_schema}"
        ))
        .into());
    }
    if surface_summary.target != target && surface_summary.target != expected_surface_target {
        return Err(TheurgyError::new(format!(
            "surface IR target must be {target} or {expected_surface_target}"
        ))
        .into());
    }
    for action_id in &surface_summary.action_ids {
        if !summary
            .action_ids
            .iter()
            .any(|product_action| product_action == action_id)
        {
            return Err(TheurgyError::new(format!(
                "surface IR action not declared in Product IR: {action_id}"
            ))
            .into());
        }
    }
    fs::create_dir_all(out_dir)?;
    write_or_replace(
        &out_dir.join("theurgy-surface.json"),
        &format!("{}\n", surface.trim_end()),
    )?;
    let runtime_metadata = generated_runtime_metadata(summary, runtime, target, &surface_summary);
    validate_generated_runtime(&runtime_metadata)?;
    write_or_replace(&out_dir.join("theurgy-runtime.json"), &runtime_metadata)?;
    if preserve_existing_legacy_desktop_adapter
        && product_runtime::is_desktop_target(target)
        && desktop_adapter_source_exists(target, out_dir)
    {
        return Ok(());
    }
    match target {
        "macos" => compile_macos(summary, &surface_summary, runtime, out_dir),
        "linux" => compile_linux(summary, &surface_summary, runtime, out_dir),
        "ios" => compile_ios(summary, &surface_summary, runtime, out_dir),
        "android" => compile_android(summary, &surface_summary, runtime, out_dir),
        _ => Err(TheurgyError::new("unsupported target").into()),
    }
}

fn desktop_adapter_source_exists(target: &str, out_dir: &Path) -> bool {
    match target {
        "macos" => {
            out_dir.join("Package.swift").exists() && out_dir.join("Sources/App/App.swift").exists()
        }
        "linux" => out_dir.join("meson.build").exists() && out_dir.join("src/main.c").exists(),
        _ => false,
    }
}

fn generated_runtime_metadata(
    summary: &ProductSummary,
    runtime: &RuntimeContract,
    target: &str,
    surface: &SurfaceSummary,
) -> String {
    product_runtime::generated_runtime_metadata(
        &product_ir_from_summary(summary),
        &runtime_bridge_from_contract(runtime),
        target,
        &surface_ir_from_summary(surface),
    )
    .expect("validated compile inputs produce generated runtime metadata")
}

fn release_target_ids(summary: &ProductSummary) -> Vec<String> {
    summary
        .release_targets
        .iter()
        .map(|release_target| release_target.id.clone())
        .collect()
}

fn release_target_for_target<'a>(
    summary: &'a ProductSummary,
    target: &str,
) -> Option<&'a ReleaseTarget> {
    summary
        .release_targets
        .iter()
        .find(|release_target| release_target.target == target)
}

fn string_vec_value(values: &[String]) -> Value {
    Value::Array(values.iter().cloned().map(Value::String).collect())
}

fn effective_subscribe_status_command(runtime: &RuntimeContract) -> Vec<String> {
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

fn compile_macos(
    summary: &ProductSummary,
    surface: &SurfaceSummary,
    runtime: &RuntimeContract,
    out_dir: &Path,
) -> Result<()> {
    fs::create_dir_all(out_dir.join("Sources/App"))?;
    write_or_replace(
        &out_dir.join("Package.swift"),
        &format!(
            "// Generated by theurgy-runtime compile-native.\n// swift-tools-version: 6.0\nimport PackageDescription\n\nlet package = Package(name: \"{}\", platforms: [.macOS(.v13)], products: [.executable(name: \"{}\", targets: [\"App\"])], targets: [.executableTarget(name: \"App\", path: \"Sources/App\")])\n",
            summary.app_id, summary.app_id
        ),
    )?;
    write_or_replace(
        &out_dir.join("Sources/App/App.swift"),
        &macos_adapter_source(summary, surface, runtime),
    )
}

fn compile_linux(
    summary: &ProductSummary,
    surface: &SurfaceSummary,
    runtime: &RuntimeContract,
    out_dir: &Path,
) -> Result<()> {
    fs::create_dir_all(out_dir.join("src"))?;
    write_or_replace(
        &out_dir.join("meson.build"),
        &format!(
            "project('{}', 'c', version: '0.1.0')\ngtk = dependency('gtk4')\njson_glib = dependency('json-glib-1.0')\nexecutable('{}', 'src/main.c', dependencies: [gtk, json_glib], install: true)\n",
            summary.app_id, summary.app_id
        ),
    )?;
    write_or_replace(
        &out_dir.join("src/main.c"),
        &linux_adapter_source(summary, surface, runtime),
    )
}

fn macos_adapter_source(
    summary: &ProductSummary,
    surface: &SurfaceSummary,
    runtime: &RuntimeContract,
) -> String {
    let action_contracts = surface_action_contracts(summary, surface);
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
        .replace("__APP_NAME__", &swift_escape(&summary.app_name))
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

fn linux_adapter_source(
    summary: &ProductSummary,
    surface: &SurfaceSummary,
    runtime: &RuntimeContract,
) -> String {
    let action_contracts = surface_action_contracts(summary, surface);
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
        .replace("__APP_NAME__", &c_escape(&summary.app_name))
        .replace("__APP_ID__", &c_escape(&summary.app_id.replace('-', "_")))
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

fn compile_ios(
    summary: &ProductSummary,
    surface: &SurfaceSummary,
    runtime: &RuntimeContract,
    out_dir: &Path,
) -> Result<()> {
    fs::create_dir_all(out_dir.join("Host"))?;
    write_or_replace(
        &out_dir.join("Package.swift"),
        &format!(
            "// Generated by theurgy-runtime compile-native.\n// swift-tools-version: 6.0\nimport PackageDescription\n\nlet package = Package(name: \"{}-ios\", platforms: [.iOS(.v16)], products: [.executable(name: \"{}IOS\", targets: [\"Host\"])], targets: [.executableTarget(name: \"Host\", path: \"Host\", resources: [.copy(\"Resources\")])])\n",
            summary.app_id, summary.app_id
        ),
    )?;
    copy_generated_contract_resources(out_dir, &out_dir.join("Host/Resources"))?;
    write_or_replace(
        &out_dir.join("Host/App.swift"),
        &ios_adapter_source(summary, surface, runtime),
    )
}

fn compile_android(
    summary: &ProductSummary,
    surface: &SurfaceSummary,
    runtime: &RuntimeContract,
    out_dir: &Path,
) -> Result<()> {
    fs::create_dir_all(out_dir.join("app/src/main/java/app/theurgy/generated"))?;
    fs::create_dir_all(out_dir.join("app/src/main"))?;
    write_or_replace(
        &out_dir.join("settings.gradle"),
        &format!("pluginManagement {{ repositories {{ google(); mavenCentral(); gradlePluginPortal() }} }}\ndependencyResolutionManagement {{ repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS); repositories {{ google(); mavenCentral() }} }}\nrootProject.name = '{}-theurgy'\ninclude ':app'\n", summary.app_id),
    )?;
    write_or_replace(
        &out_dir.join("build.gradle"),
        "plugins {\n    id 'com.android.application' version '8.7.3' apply false\n}\n",
    )?;
    write_or_replace(
        &out_dir.join("app/build.gradle"),
        &format!(
            "plugins {{\n    id 'com.android.application'\n}}\n\nandroid {{\n    namespace 'app.theurgy.generated'\n    compileSdk 35\n\n    defaultConfig {{\n        applicationId 'app.theurgy.{}'\n        minSdk 26\n        targetSdk 35\n        versionCode 1\n        versionName '0.1.0'\n    }}\n}}\n",
            summary.app_id.replace('-', "_")
        ),
    )?;
    write_or_replace(
        &out_dir.join("app/src/main/AndroidManifest.xml"),
        &format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<manifest xmlns:android=\"http://schemas.android.com/apk/res/android\">\n    <application android:theme=\"@style/AppTheme\" android:label=\"{}\">\n        <activity android:name=\".MainActivity\" android:exported=\"true\">\n            <intent-filter>\n                <action android:name=\"android.intent.action.MAIN\" />\n                <category android:name=\"android.intent.category.LAUNCHER\" />\n            </intent-filter>\n        </activity>\n    </application>\n</manifest>\n",
            xml_escape(&summary.app_name)
        ),
    )?;
    fs::create_dir_all(out_dir.join("app/src/main/res/values"))?;
    write_or_replace(
        &out_dir.join("app/src/main/res/values/styles.xml"),
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<resources>\n    <style name=\"AppTheme\" parent=\"android:style/Theme.Material.Light.NoActionBar\" />\n</resources>\n",
    )?;
    copy_generated_contract_resources(out_dir, &out_dir.join("app/src/main/assets"))?;
    write_or_replace(
        &out_dir.join("app/src/main/java/app/theurgy/generated/MainActivity.java"),
        &android_adapter_source(summary, surface, runtime),
    )
}

fn copy_generated_contract_resources(out_dir: &Path, resources_dir: &Path) -> Result<()> {
    fs::create_dir_all(resources_dir)?;
    for name in ["theurgy-runtime.json", "theurgy-surface.json"] {
        let contents = fs::read_to_string(out_dir.join(name)).map_err(|error| {
            TheurgyError::new(format!(
                "could not read generated contract resource {}: {error}",
                out_dir.join(name).display()
            ))
        })?;
        write_or_replace(&resources_dir.join(name), &contents)?;
    }
    Ok(())
}

fn ios_adapter_source(
    summary: &ProductSummary,
    surface: &SurfaceSummary,
    runtime: &RuntimeContract,
) -> String {
    let action_contracts = surface_action_contracts(summary, surface);
    let subscribe_status_command = effective_subscribe_status_command(runtime);
    let template = r#"// Generated by theurgy-runtime compile-native.
// Runtime: theurgy-runtime.json
// Surface: theurgy-surface.json
import Foundation
import SwiftUI

func loadBundledContract(_ name: String) -> String {
  guard let url = Bundle.module.url(forResource: name, withExtension: "json") else {
    return "\(name).json missing"
  }
  return (try? String(contentsOf: url, encoding: .utf8)) ?? "\(name).json unreadable"
}

func contractObject(_ json: String) -> [String: Any]? {
  guard let data = json.data(using: .utf8) else {
    return nil
  }
  return (try? JSONSerialization.jsonObject(with: data)) as? [String: Any]
}

func runtimeString(_ json: String, key: String) -> String {
  guard let value = contractObject(json)?[key] as? String else {
    return "\(key) unavailable"
  }
  return value
}

func runtimeStringArray(_ json: String, key: String) -> [String] {
  guard let values = contractObject(json)?[key] as? [String] else {
    return []
  }
  return values
}

func surfaceScreens(_ json: String) -> [String] {
  guard let screens = contractObject(json)?["screens"] as? [[String: Any]] else {
    return []
  }
  return screens.compactMap { screen in
    guard let id = screen["id"] as? String else {
      return nil
    }
    let title = screen["title"] as? String ?? id
    return "\(id): \(title)"
  }
}

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

struct RuntimeContract {
  let runtimeMetadata = loadBundledContract("theurgy-runtime")
  let surfaceMetadata = loadBundledContract("theurgy-surface")
  var runtimeApp: String { runtimeString(runtimeMetadata, key: "app") }
  var runtimeTarget: String { runtimeString(runtimeMetadata, key: "target") }
  var runtimeTransport: String { runtimeString(runtimeMetadata, key: "adapterRuntimeTransport") }
  var runtimeStatusSchema: String { runtimeString(runtimeMetadata, key: "runtimeStatusSchema") }
  var runtimeActionRequestSchema: String { runtimeString(runtimeMetadata, key: "runtimeActionRequestSchema") }
  var runtimeActionResultSchema: String { runtimeString(runtimeMetadata, key: "runtimeActionResultSchema") }
  var operationStatusSchema: String { runtimeString(runtimeMetadata, key: "operationStatusSchema") }
  var operationHistorySchema: String { runtimeString(runtimeMetadata, key: "operationHistorySchema") }
  var runtimeSurfaceActions: [String] { runtimeStringArray(runtimeMetadata, key: "surfaceActions") }
  var surfaceSchema: String { runtimeString(surfaceMetadata, key: "version") }
  var surfaceTarget: String { runtimeString(surfaceMetadata, key: "target") }
  var surfaceScreens: [String] { surfaceScreens(surfaceMetadata) }
  let protocolName = "__PROTOCOL__"
  let stateCommand = __STATE_COMMAND__
  let statusCommand = __STATUS_COMMAND__
  let subscribeStatusCommand = __SUBSCRIBE_STATUS_COMMAND__
  let operationStatusCommand = __OPERATION_STATUS_COMMAND__
  let actionCommand = __ACTION_COMMAND__
  let historyCommand = __HISTORY_COMMAND__
  let daemonCommand = __DAEMON_COMMAND__
  let actionContracts = __ACTION_CONTRACTS__

  func command(for action: ProductActionContract, json: String) -> [String] {
    actionCommand + [action.id, json]
  }

  func actionEnvelope(for action: ProductActionContract, params: [String: Any]) -> String {
    let envelope: [String: Any] = [
      "protocol": protocolName,
      "app": runtimeApp,
      "action": action.id,
      "params": params
    ]
    guard let data = try? JSONSerialization.data(withJSONObject: envelope, options: [.sortedKeys]) else {
      return "{}"
    }
    return String(data: data, encoding: .utf8) ?? "{}"
  }
}

struct RuntimeContractView: View {
  let contract = RuntimeContract()

  var body: some View {
    NavigationStack {
      List {
        Section("Runtime") {
          Text(contract.protocolName)
          Text("Runtime app: \(contract.runtimeApp)")
          Text("Runtime target: \(contract.runtimeTarget)")
          Text("Runtime transport: \(contract.runtimeTransport)")
          Text("Runtime status schema: \(contract.runtimeStatusSchema)")
          Text("Runtime action request schema: \(contract.runtimeActionRequestSchema)")
          Text("Runtime action result schema: \(contract.runtimeActionResultSchema)")
          Text("Operation status schema: \(contract.operationStatusSchema)")
          Text("Operation history schema: \(contract.operationHistorySchema)")
          Text("Runtime surface actions: \(contract.runtimeSurfaceActions.joined(separator: ", "))")
          Text(contract.stateCommand.joined(separator: " "))
          Text(contract.statusCommand.joined(separator: " "))
          Text(contract.subscribeStatusCommand.joined(separator: " "))
          Text(contract.operationStatusCommand.joined(separator: " "))
          Text(contract.actionCommand.joined(separator: " "))
          Text(contract.historyCommand.joined(separator: " "))
          Text(contract.daemonCommand.joined(separator: " "))
        }
        Section("Surface") {
          Text("Surface schema: \(contract.surfaceSchema)")
          Text("Surface target: \(contract.surfaceTarget)")
          Text("Surface screens: \(contract.surfaceScreens.joined(separator: ", "))")
        }
        Section("Actions") {
          ForEach(contract.actionContracts, id: \.id) { action in
            VStack(alignment: .leading) {
              Text(action.label)
              Text(action.effect)
              Text(contract.command(for: action, json: "{}").joined(separator: " "))
              Text(contract.actionEnvelope(for: action, params: [:]))
            }
          }
        }
      }
      .navigationTitle("__APP_NAME__")
    }
  }
}

@main
struct TheurgyMobileApp: App {
  var body: some Scene {
    WindowGroup {
      RuntimeContractView()
    }
  }
}
"#;
    template
        .replace("__APP_NAME__", &swift_escape(&summary.app_name))
        .replace("__PROTOCOL__", &swift_escape(&runtime.protocol))
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
}

fn android_adapter_source(
    summary: &ProductSummary,
    surface: &SurfaceSummary,
    runtime: &RuntimeContract,
) -> String {
    let action_contracts = surface_action_contracts(summary, surface);
    let subscribe_status_command = effective_subscribe_status_command(runtime);
    let template = r#"package app.theurgy.generated;

import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

public final class MainActivity extends Activity {
  private static final String PROTOCOL = "__PROTOCOL__";
  private static final String[] STATE_COMMAND = __STATE_COMMAND__;
  private static final String[] STATUS_COMMAND = __STATUS_COMMAND__;
  private static final String[] SUBSCRIBE_STATUS_COMMAND = __SUBSCRIBE_STATUS_COMMAND__;
  private static final String[] OPERATION_STATUS_COMMAND = __OPERATION_STATUS_COMMAND__;
  private static final String[] ACTION_COMMAND = __ACTION_COMMAND__;
  private static final String[] HISTORY_COMMAND = __HISTORY_COMMAND__;
  private static final String[] DAEMON_COMMAND = __DAEMON_COMMAND__;
  private static final ProductActionContract[] ACTION_CONTRACTS = __ACTION_CONTRACTS__;

  private static final class ProductActionContract {
    final String id;
    final String label;
    final String effect;
    final boolean safe;
    final boolean mutating;
    final boolean longRunning;
    final boolean privileged;
    final String[] inputKeys;
    final String[] outputKeys;
    final String[] failureKeys;
    final String[][] inputShape;
    final String[][] outputShape;
    final String[][] failureShape;

    ProductActionContract(String id, String label, String effect, boolean safe, boolean mutating, boolean longRunning, boolean privileged, String[] inputKeys, String[] outputKeys, String[] failureKeys, String[][] inputShape, String[][] outputShape, String[][] failureShape) {
      this.id = id;
      this.label = label;
      this.effect = effect;
      this.safe = safe;
      this.mutating = mutating;
      this.longRunning = longRunning;
      this.privileged = privileged;
      this.inputKeys = inputKeys;
      this.outputKeys = outputKeys;
      this.failureKeys = failureKeys;
      this.inputShape = inputShape;
      this.outputShape = outputShape;
      this.failureShape = failureShape;
    }
  }

  private static String[] commandFor(ProductActionContract action, String json) {
    String[] command = new String[ACTION_COMMAND.length + 2];
    System.arraycopy(ACTION_COMMAND, 0, command, 0, ACTION_COMMAND.length);
    command[ACTION_COMMAND.length] = action.id;
    command[ACTION_COMMAND.length + 1] = json;
    return command;
  }

  private static String actionEnvelope(String app, ProductActionContract action, JSONObject params) {
    try {
      JSONObject envelope = new JSONObject();
      envelope.put("protocol", PROTOCOL);
      envelope.put("app", app);
      envelope.put("action", action.id);
      envelope.put("params", params);
      return envelope.toString();
    } catch (JSONException error) {
      return "{}";
    }
  }

  private String loadBundledContract(String name) {
    try (InputStream input = getAssets().open(name)) {
      ByteArrayOutputStream output = new ByteArrayOutputStream();
      byte[] buffer = new byte[4096];
      int read;
      while ((read = input.read(buffer)) != -1) {
        output.write(buffer, 0, read);
      }
      return output.toString(StandardCharsets.UTF_8.name());
    } catch (IOException error) {
      return name + " missing";
    }
  }

  private static String jsonString(String json, String key) {
    try {
      JSONObject object = new JSONObject(json);
      return object.has(key) ? object.getString(key) : key + " unavailable";
    } catch (JSONException error) {
      return key + " unavailable";
    }
  }

  private static String jsonStringArray(String json, String key) {
    try {
      JSONArray values = new JSONObject(json).optJSONArray(key);
      if (values == null) {
        return "";
      }
      StringBuilder text = new StringBuilder();
      for (int i = 0; i < values.length(); i += 1) {
        if (i > 0) {
          text.append(", ");
        }
        text.append(values.optString(i));
      }
      return text.toString();
    } catch (JSONException error) {
      return "";
    }
  }

  private static String surfaceScreens(String json) {
    try {
      JSONArray screens = new JSONObject(json).optJSONArray("screens");
      if (screens == null) {
        return "";
      }
      StringBuilder text = new StringBuilder();
      for (int i = 0; i < screens.length(); i += 1) {
        JSONObject screen = screens.optJSONObject(i);
        if (screen == null) {
          continue;
        }
        if (text.length() > 0) {
          text.append(", ");
        }
        String id = screen.optString("id");
        String title = screen.optString("title", id);
        text.append(id).append(": ").append(title);
      }
      return text.toString();
    } catch (JSONException error) {
      return "";
    }
  }

  @Override public void onCreate(Bundle state) {
    super.onCreate(state);
    TextView view = new TextView(this);
    String runtimeMetadata = loadBundledContract("theurgy-runtime.json");
    String surfaceMetadata = loadBundledContract("theurgy-surface.json");
    String runtimeApp = jsonString(runtimeMetadata, "app");
    StringBuilder text = new StringBuilder();
    text.append("__APP_NAME__\nRuntime: ").append(PROTOCOL)
      .append("\nRuntime app: ").append(runtimeApp)
      .append("\nRuntime target: ").append(jsonString(runtimeMetadata, "target"))
      .append("\nRuntime transport: ").append(jsonString(runtimeMetadata, "adapterRuntimeTransport"))
      .append("\nRuntime status schema: ").append(jsonString(runtimeMetadata, "runtimeStatusSchema"))
      .append("\nRuntime action request schema: ").append(jsonString(runtimeMetadata, "runtimeActionRequestSchema"))
      .append("\nRuntime action result schema: ").append(jsonString(runtimeMetadata, "runtimeActionResultSchema"))
      .append("\nOperation status schema: ").append(jsonString(runtimeMetadata, "operationStatusSchema"))
      .append("\nOperation history schema: ").append(jsonString(runtimeMetadata, "operationHistorySchema"))
      .append("\nRuntime surface actions: ").append(jsonStringArray(runtimeMetadata, "surfaceActions"))
      .append("\nSurface schema: ").append(jsonString(surfaceMetadata, "version"))
      .append("\nSurface target: ").append(jsonString(surfaceMetadata, "target"))
      .append("\nSurface screens: ").append(surfaceScreens(surfaceMetadata))
      .append("\nState: ").append(String.join(" ", STATE_COMMAND))
      .append("\nStatus: ").append(String.join(" ", STATUS_COMMAND))
      .append("\nSubscribe: ").append(String.join(" ", SUBSCRIBE_STATUS_COMMAND))
      .append("\nOperation status: ").append(String.join(" ", OPERATION_STATUS_COMMAND))
      .append("\nAction: ").append(String.join(" ", ACTION_COMMAND))
      .append("\nHistory: ").append(String.join(" ", HISTORY_COMMAND))
      .append("\nDaemon: ").append(String.join(" ", DAEMON_COMMAND))
      .append("\nActions:");
    for (ProductActionContract action : ACTION_CONTRACTS) {
      text.append("\n").append(action.label).append(" [").append(action.effect).append("] ")
        .append(String.join(" ", commandFor(action, "{}")))
        .append("\n  envelope: ").append(actionEnvelope(runtimeApp, action, new JSONObject()));
    }
    view.setText(text.toString());
    setContentView(view);
  }
}
"#;
    template
        .replace("__APP_NAME__", &java_escape(&summary.app_name))
        .replace("__PROTOCOL__", &java_escape(&runtime.protocol))
        .replace(
            "__STATE_COMMAND__",
            &java_string_array_literal(&runtime.state_command),
        )
        .replace(
            "__STATUS_COMMAND__",
            &java_string_array_literal(&runtime.status_command),
        )
        .replace(
            "__SUBSCRIBE_STATUS_COMMAND__",
            &java_string_array_literal(&subscribe_status_command),
        )
        .replace(
            "__OPERATION_STATUS_COMMAND__",
            &java_string_array_literal(&runtime.operation_status_command),
        )
        .replace(
            "__ACTION_COMMAND__",
            &java_string_array_literal(&runtime.action_command),
        )
        .replace(
            "__HISTORY_COMMAND__",
            &java_string_array_literal(&runtime.history_command),
        )
        .replace(
            "__DAEMON_COMMAND__",
            &java_string_array_literal(&runtime.daemon_command),
        )
        .replace(
            "__ACTION_CONTRACTS__",
            &java_action_contracts_literal(&action_contracts),
        )
}

fn surface_action_contracts(
    summary: &ProductSummary,
    surface: &SurfaceSummary,
) -> Vec<ActionContract> {
    surface
        .action_ids
        .iter()
        .filter_map(|action_id| {
            summary
                .action_contracts
                .iter()
                .find(|contract| &contract.id == action_id)
                .cloned()
        })
        .collect()
}

fn write_or_replace(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}

fn swift_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn c_escape(value: &str) -> String {
    swift_escape(value).replace('\n', "\\n")
}

fn java_escape(value: &str) -> String {
    c_escape(value)
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn create_project(kind: ProjectKind, name: &str, path: &Path) -> Result<()> {
    if path.exists() && path.read_dir()?.next().is_some() {
        return Err(TheurgyError::new(format!(
            "target already exists and is not empty: {}",
            path.display()
        ))
        .into());
    }

    fs::create_dir_all(path)?;
    fs::create_dir_all(path.join(".github"))?;
    fs::create_dir_all(path.join("src"))?;

    write_new(
        &path.join(".gitignore"),
        "/target/\n/.theurgy-state/\n*.log\n*.tmp\n.DS_Store\n",
    )?;
    write_new(&path.join("LICENSE"), AGPL_NOTICE)?;
    write_new(&path.join("WIZARDRY_ADDENDUM.md"), WIZARDRY_ADDENDUM)?;
    write_new(&path.join(".github/AI_DOCS.md"), &generated_ai_docs(kind))?;
    write_new(&path.join("theurgy.project.toml"), &manifest(kind, name))?;

    match kind {
        ProjectKind::Desktop => create_desktop_project(path, name),
        ProjectKind::Website => create_website_project(path, name),
    }
}

fn create_desktop_project(path: &Path, name: &str) -> Result<()> {
    write_new(
        &path.join("src/app.theurgy"),
        &format!(
            "app \"{name}\" {{\n  track = \"native-desktop\"\n  runtime = \"rust\"\n  state = \"file-first\"\n}}\n"
        ),
    )?;
    write_new(
        &path.join("src/main.rs"),
        "fn main() {\n    println!(\"theurgy desktop app scaffold\");\n}\n",
    )?;
    write_new(
        &path.join("README.md"),
        &format!(
            "# {name}\n\nThis native desktop project was generated by theurgy.\n\nThe generated project is licensed under GNU AGPL-3.0-or-later with Wizardry Addendum 1.0.\n"
        ),
    )?;
    Ok(())
}

fn create_website_project(path: &Path, name: &str) -> Result<()> {
    fs::create_dir_all(path.join("content/pages"))?;
    fs::create_dir_all(path.join("cgi"))?;
    write_new(
        &path.join("src/site.theurgy"),
        &format!(
            "site \"{name}\" {{\n  track = \"enterprise-web\"\n  runtime = \"theurgy-web\"\n  truth = \"content-files\"\n  database = \"optional\"\n}}\n"
        ),
    )?;
    write_new(&path.join("theurgy.web.toml"), &web_manifest(name))?;
    write_executable(
        &path.join("cgi/theurgy-cgi-context"),
        "#!/bin/sh\nset -eu\nexec \"${THEURGY_RUNTIME:-theurgy-runtime}\" capture-cgi-context\n",
    )?;
    write_new(
        &path.join("content/pages/index.html"),
        &format!(
            "<!doctype html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"utf-8\">\n  <title>{name}</title>\n</head>\n<body>\n  <main>\n    <h1>{name}</h1>\n  </main>\n</body>\n</html>\n"
        ),
    )?;
    write_new(
        &path.join("README.md"),
        &format!(
            "# {name}\n\nThis enterprise web project was generated by theurgy.\n\nThe generated project keeps content and config file-first. Dynamic hot paths should move through the Theurgy web runtime harness before they replace wizardry shell routes.\n\nThe generated project is licensed under GNU AGPL-3.0-or-later with Wizardry Addendum 1.0.\n"
        ),
    )?;
    Ok(())
}

fn write_new(path: &Path, contents: &str) -> Result<()> {
    if path.exists() {
        return Err(TheurgyError::new(format!("refusing to overwrite {}", path.display())).into());
    }
    fs::write(path, contents)?;
    Ok(())
}

fn write_executable(path: &Path, contents: &str) -> Result<()> {
    write_new(path, contents)?;
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
    }
    Ok(())
}

fn validate_name(value: &str) -> Result<&str> {
    let valid = !value.is_empty()
        && value.len() <= 80
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && value
            .as_bytes()
            .first()
            .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        && value
            .as_bytes()
            .last()
            .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        && !value.contains("--");
    if valid {
        Ok(value)
    } else {
        Err(
            TheurgyError::new("name must be lowercase ASCII letters, digits, and single hyphens")
                .into(),
        )
    }
}

fn manifest(kind: ProjectKind, name: &str) -> String {
    format!(
        "name = \"{name}\"\nkind = \"{}\"\nsource_root = \"src\"\ntruth = \"file-first\"\nlicense = \"AGPL-3.0-or-later WITH Wizardry-Addendum-1.0\"\n",
        kind.as_str()
    )
}

fn web_manifest(name: &str) -> String {
    format!(
        "name = \"{name}\"\ntrack = \"enterprise-web\"\nruntime = \"theurgy-web\"\ntruth = \"file-first\"\ncanonical_state = \".sitedata/<site>\"\nfront_doors = [\"nginx\", \"lighttpd\"]\nadapters = [\"http\", \"fastcgi\", \"cgi-compat\"]\nrouter = \"axum\"\nserialization = \"serde\"\ntemplates = \"tera\"\nsearch = \"tantivy\"\nzola_core_runtime = false\nphase = \"contract-and-adapter\"\n"
    )
}

fn generated_ai_docs(kind: ProjectKind) -> String {
    format!(
        "# Generated Theurgy Project\n\n- Track: `{}`.\n- Keep durable truth in source files unless a documented database need exists.\n- Keep runtime state and build products out of Git.\n- Do not add shell fragments for user-controlled execution paths.\n- Preserve CLI parity for GUI behavior.\n- For enterprise web projects, use `theurgy.web.toml` as the runtime contract and keep wizardry shell out of hot request paths once a typed handler exists.\n",
        kind.as_str()
    )
}

#[derive(Debug, Eq, PartialEq)]
struct WebRequestContext {
    method: String,
    script_name: String,
    path_info: String,
    query_string: String,
    content_type: String,
    content_length: u64,
    host: String,
    https: bool,
    site_name: String,
    sites_dir: String,
}

impl WebRequestContext {
    fn from_env() -> Self {
        Self {
            method: env_value("REQUEST_METHOD", "GET"),
            script_name: env_value("SCRIPT_NAME", ""),
            path_info: env_value("PATH_INFO", ""),
            query_string: env_value("QUERY_STRING", ""),
            content_type: env_value("CONTENT_TYPE", ""),
            content_length: parse_u64_env("CONTENT_LENGTH"),
            host: normalized_host(&env_value("HTTP_HOST", "")),
            https: env_flag("HTTPS") || env_value("REQUEST_SCHEME", "") == "https",
            site_name: env_value("WIZARDRY_SITE_NAME", "default"),
            sites_dir: env_value("WIZARDRY_SITES_DIR", "~/sites"),
        }
    }

    fn to_json(&self) -> String {
        format!(
            "{{\"method\":\"{}\",\"script_name\":\"{}\",\"path_info\":\"{}\",\"query_string\":\"{}\",\"content_type\":\"{}\",\"content_length\":{},\"host\":\"{}\",\"https\":{},\"site_name\":\"{}\",\"sites_dir\":\"{}\"}}",
            json_escape(&self.method),
            json_escape(&self.script_name),
            json_escape(&self.path_info),
            json_escape(&self.query_string),
            json_escape(&self.content_type),
            self.content_length,
            json_escape(&self.host),
            if self.https { "true" } else { "false" },
            json_escape(&self.site_name),
            json_escape(&self.sites_dir)
        )
    }
}

fn env_value(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

fn parse_u64_env(name: &str) -> u64 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0)
}

fn env_flag(name: &str) -> bool {
    matches!(
        env::var(name)
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes" | "on"
    )
}

fn normalized_host(raw: &str) -> String {
    let host = raw
        .split(',')
        .next()
        .unwrap_or("")
        .trim()
        .trim_end_matches('.')
        .to_ascii_lowercase();
    let host = host
        .strip_prefix("http://")
        .or_else(|| host.strip_prefix("https://"))
        .unwrap_or(&host);
    host.split('/')
        .next()
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("")
        .to_string()
}

fn json_escape(raw: &str) -> String {
    let mut escaped = String::with_capacity(raw.len());
    for character in raw.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => {
                escaped.push_str(&format!("\\u{:04x}", character as u32));
            }
            character => escaped.push(character),
        }
    }
    escaped
}

#[derive(Debug, Eq, PartialEq)]
struct ManifestSummary {
    name: String,
    kind: String,
    source_root: String,
}

fn inspect_manifest(manifest: &str) -> Result<ManifestSummary> {
    let name = manifest_value(manifest, "name")?;
    let kind = manifest_value(manifest, "kind")?;
    let source_root = manifest_value(manifest, "source_root")?;
    Ok(ManifestSummary {
        name,
        kind,
        source_root,
    })
}

fn manifest_value(manifest: &str, key: &str) -> Result<String> {
    for line in manifest.lines() {
        let Some((left, right)) = line.split_once('=') else {
            continue;
        };
        if left.trim() == key {
            let value = right.trim();
            if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
                return Ok(value[1..value.len() - 1].to_string());
            }
        }
    }
    Err(TheurgyError::new(format!("manifest missing string key: {key}")).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn validates_safe_project_names() {
        assert!(validate_name("serious-app-1").is_ok());
        assert!(validate_name("Serious").is_err());
        assert!(validate_name("../escape").is_err());
        assert!(validate_name("bad--gap").is_err());
        assert!(validate_name("-bad").is_err());
    }

    #[test]
    fn inspects_manifest_values() {
        let summary = inspect_manifest(
            "name = \"demo\"\nkind = \"desktop\"\nsource_root = \"src\"\ntruth = \"file-first\"\n",
        )
        .unwrap();
        assert_eq!(
            summary,
            ManifestSummary {
                name: "demo".to_string(),
                kind: "desktop".to_string(),
                source_root: "src".to_string()
            }
        );
    }

    #[test]
    fn inspect_app_summarizes_product_runtime_and_surface_contracts() {
        let app = test_root("inspect-app-contract");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/product.ir.json"),
            &sample_product_with_state_commands("app-blueprint/desktop.surface.ir.json"),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();

        let lines = inspect_app_lines(&app).unwrap();
        assert!(lines.contains(&"product_app=deployments".to_string()));
        assert!(lines.contains(
            &"product_surface_desktop=app-blueprint/desktop.surface.ir.json".to_string()
        ));
        assert!(lines
            .contains(&"product_surface_mobile=app-blueprint/mobile.surface.ir.json".to_string()));
        assert!(lines.contains(&"product_state_snapshot_schema=deployments-state/v1".to_string()));
        assert!(lines.contains(&"product_state_command=custom-core state".to_string()));
        assert!(lines.contains(&"product_state_status_command=custom-core status".to_string()));
        assert!(lines.contains(&"product_actions=2".to_string()));
        assert!(lines.contains(&"product_long_running_actions=1".to_string()));
        assert!(lines.contains(&"product_persistence_truth=file-first".to_string()));
        assert!(lines.contains(
            &"product_background_job_server-queue_command=deployments-daemon".to_string()
        ));
        assert!(lines.contains(&"runtime_protocol=deployments-runtime/v1".to_string()));
        assert!(lines
            .contains(&"runtime_legacy_native_desktop_ir=app-blueprint/app.ir.yaml".to_string()));
        assert!(lines.contains(
            &"runtime_operation_status_command=custom-core operation-status".to_string()
        ));
        assert!(lines.contains(&"desktop_surface_actions=2".to_string()));
        assert!(lines.contains(&"compatibility_wizardry_apps_shell_first=true".to_string()));
        assert!(lines.contains(
            &"compatibility_theurgy_required_for_legacy_wizardry_apps=false".to_string()
        ));
        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn inspect_app_rejects_surface_action_drift() {
        let app = test_root("inspect-app-surface-drift");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/product.ir.json"),
            &sample_product(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface().replace(
                "\"actions\": [\"refresh_state\", \"publish_changes\"]",
                "\"actions\": [\"refresh_state\", \"delete_everything\"]",
            ),
        )
        .unwrap();

        let error = inspect_app_lines(&app).unwrap_err().to_string();
        assert!(error.contains("surface IR action not declared in Product IR: delete_everything"));
        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn creates_desktop_project_without_overwrite() {
        let root = test_root("desktop");
        create_project(ProjectKind::Desktop, "demo-desktop", &root).unwrap();
        assert!(root.join("theurgy.project.toml").exists());
        assert!(root.join("src/main.rs").exists());
        assert!(create_project(ProjectKind::Desktop, "demo-desktop", &root).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn creates_website_project() {
        let root = test_root("website");
        create_project(ProjectKind::Website, "demo-website", &root).unwrap();
        assert!(root.join("content/pages/index.html").exists());
        assert!(root.join("theurgy.web.toml").exists());
        assert!(root.join("cgi/theurgy-cgi-context").exists());
        let manifest = fs::read_to_string(root.join("theurgy.project.toml")).unwrap();
        assert!(manifest.contains("kind = \"website\""));
        let web_manifest = fs::read_to_string(root.join("theurgy.web.toml")).unwrap();
        assert!(web_manifest.contains("router = \"axum\""));
        assert!(web_manifest.contains("search = \"tantivy\""));
        assert!(web_manifest.contains("zola_core_runtime = false"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn web_runtime_contract_names_integrated_components() {
        let lines = web_runtime_assay_lines().join("\n");
        assert!(lines.contains("front_doors=nginx,lighttpd"));
        assert!(lines.contains("adapters=http,fastcgi,cgi-compat"));
        assert!(lines.contains("router=axum"));
        assert!(lines.contains("serialization=serde"));
        assert!(lines.contains("templates=tera"));
        assert!(lines.contains("search=tantivy"));
        assert!(lines.contains("zola=not-core-runtime"));
    }

    #[test]
    fn cgi_context_helpers_normalize_and_escape() {
        assert_eq!(
            normalized_host("HTTPS://Desk.Example.TEST:443/path, ignored"),
            "desk.example.test"
        );
        assert_eq!(json_escape("line\n\"quoted\""), "line\\n\\\"quoted\\\"");
    }

    fn test_root(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        env::temp_dir().join(format!("theurgy-test-{label}-{nanos}"))
    }

    #[test]
    fn generated_paths_are_plain_files() {
        let path = Path::new("theurgy.project.toml");
        assert_eq!(path.file_name(), Some(OsStr::new("theurgy.project.toml")));
    }

    #[test]
    fn conjure_defaults_path_to_name() {
        let cwd = env::current_dir().unwrap();
        let root = test_root("default-path");
        fs::create_dir_all(&root).unwrap();
        env::set_current_dir(&root).unwrap();
        command_conjure(ProjectKind::Desktop, &[String::from("default-app")]).unwrap();
        assert!(root.join("default-app/theurgy.project.toml").exists());
        env::set_current_dir(cwd).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn validates_product_ir_contract() {
        let product = sample_product();
        let summary = validate_product_ir(&product).unwrap();
        assert_eq!(summary.app_id, "deployments");
        assert_eq!(
            summary.targets,
            vec!["macos".to_string(), "linux".to_string()]
        );
        assert_eq!(summary.desktop_surface_ir, None);
        assert_eq!(summary.mobile_surface_ir, None);
        assert_eq!(summary.actions, 2);
        assert_eq!(
            summary.capabilities,
            vec!["native-desktop".to_string(), "runtime-actions".to_string()]
        );
        assert_eq!(summary.permissions, vec!["files".to_string()]);
        assert_eq!(
            summary.domain_object_ids,
            vec!["server".to_string(), "deployment".to_string()]
        );
        assert_eq!(
            summary.persistence_root_ids,
            vec!["headquarters-workspace".to_string()]
        );
        assert_eq!(summary.background_job_ids, vec!["server-queue".to_string()]);
        assert_eq!(
            release_target_ids(&summary),
            vec!["macos-app".to_string(), "linux-app".to_string()]
        );
        assert_eq!(
            summary.audit_keys,
            vec!["cliParity".to_string(), "operationHistory".to_string()]
        );
        assert_eq!(summary.action_contracts.len(), 2);
        let publish = summary
            .action_contracts
            .iter()
            .find(|contract| contract.id == "publish_changes")
            .unwrap();
        assert_eq!(publish.label, "Push to Production");
        assert_eq!(publish.effect, "release");
        assert!(publish.long_running);
        assert!(publish.privileged);
        assert_eq!(publish.input_keys, vec!["deployment".to_string()]);
    }

    #[test]
    fn product_ir_validation_uses_structured_json_types() {
        let product = sample_product().replace("\"safe\": true", "\"safe\": \"true\"");
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("action.safe boolean required"));
        assert!(validate_product_ir("{\"version\":\"theurgy-product-ir/v1\",").is_err());
        let product = sample_product().replace(
            "\"capabilities\": [\"native-desktop\", \"runtime-actions\"]",
            "\"capabilities\": [\"native-desktop\", 7]",
        );
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("app.capabilities must contain non-empty strings"));
        let product = sample_product().replace("\"id\": \"server\"", "\"id\": \"\"");
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("domain.objects object.id must be stable"));
        let product = sample_product().replace("\"params\": \"object\"", "\"params\": \"blob\"");
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("unsupported shape type: blob"));
        let product = sample_product().replace(
            "\"persistence\": {\n    \"truth\": \"file-first\"\n  },",
            "\"persistence\": {\n    \"database\": \"none\"\n  },",
        );
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("persistence.truth required"));
        let product = sample_product().replace("\"target\": \"macos\"", "\"target\": \"ios\"");
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("releaseTargets object.target not declared in app.targets: ios"));
        let product =
            sample_product().replace("\"surface\": \"desktop\"", "\"surface\": \"mobile\"");
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("releaseTargets object.surface for macos must be desktop"));
        let product = sample_product().replace("\"label\": \"Server Queue\"", "\"label\": \"\"");
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("backgroundJobs object.label required"));
        let product = sample_product().replace(
            "\"safe\": true, \"mutating\": false",
            "\"safe\": true, \"mutating\": false, \"command\": []",
        );
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("action.command required"));
        let product = sample_product().replace(
            "\"audit\": {\n    \"operationHistory\": true,\n    \"cliParity\": true\n  }",
            "\"surfaces\": []",
        );
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("product IR surfaces must be an object"));
        let product = sample_product().replace(
            "\"audit\": {\n    \"operationHistory\": true,\n    \"cliParity\": true\n  }",
            "\"surfaces\": {\"desktop\": \"\"}",
        );
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("product IR surfaces.desktop must be a non-empty string"));
        let product = sample_product().replace(
            ",\n    {\"id\": \"linux-app\", \"target\": \"linux\", \"surface\": \"desktop\", \"artifact\": \"generated/linux\"}",
            "",
        );
        let error = validate_product_ir(&product).unwrap_err().to_string();
        assert!(error.contains("releaseTargets missing release target for app target: linux"));
    }

    #[test]
    fn validates_action_ir_contract() {
        let actions = sample_action_ir();
        let summary = validate_action_ir(&actions).unwrap();
        assert_eq!(summary.actions, 2);
        assert_eq!(
            summary.action_ids,
            vec!["refresh_state".to_string(), "publish_changes".to_string()]
        );
    }

    #[test]
    fn action_ir_validation_uses_typed_action_contract() {
        let actions = sample_action_ir().replace("\"longRunning\": true", "\"longRunning\": 1");
        let error = validate_action_ir(&actions).unwrap_err().to_string();
        assert!(error.contains("action.longRunning boolean required"));
        assert!(validate_action_ir("{\"version\":\"theurgy-action-ir/v1\",").is_err());
    }

    #[test]
    fn validates_state_snapshot_contract() {
        let snapshot = sample_state_snapshot();
        let summary = validate_state_snapshot(&snapshot).unwrap();
        assert_eq!(summary.app_id, "deployments");
    }

    #[test]
    fn state_snapshot_validation_requires_data_object() {
        let snapshot = "{\n  \"schema\": \"theurgy-state-snapshot/v1\",\n  \"app\": \"deployments\",\n  \"generatedAt\": \"2026-06-11T00:00:00Z\",\n  \"data\": []\n}";
        let error = validate_state_snapshot(&snapshot).unwrap_err().to_string();
        assert!(error.contains("missing JSON object key: data"));
        let snapshot =
            sample_state_snapshot().replace("\"generatedAt\": \"2026-06-11T00:00:00Z\",", "");
        let error = validate_state_snapshot(&snapshot).unwrap_err().to_string();
        assert!(error.contains("generatedAt required"));
    }

    #[test]
    fn validates_runtime_status_contract() {
        let status = sample_runtime_status();
        let summary = validate_runtime_status(&status).unwrap();
        assert_eq!(summary.app_id, "deployments");
    }

    #[test]
    fn runtime_status_validation_requires_state_ready_boolean() {
        let status = sample_runtime_status().replace("\"state_ready\": true", "\"state_ready\": 1");
        let error = validate_runtime_status(&status).unwrap_err().to_string();
        assert!(error.contains("state_ready must be boolean"));
        let status =
            sample_runtime_status().replace("\"generatedAt\": \"2026-06-11T00:00:00Z\",", "");
        let error = validate_runtime_status(&status).unwrap_err().to_string();
        assert!(error.contains("generatedAt required"));
    }

    #[test]
    fn validates_runtime_action_result_contract() {
        let result = sample_runtime_action_result();
        let summary = validate_runtime_action_result(&result).unwrap();
        assert_eq!(summary.app_id, "deployments");
        assert_eq!(summary.action_id, "publish_changes");
        assert_eq!(summary.operation_id, "deployments-publish_changes-123");
    }

    #[test]
    fn runtime_action_result_validation_requires_typed_operation() {
        let result = sample_runtime_action_result().replace("\"app\": \"deployments\",\n", "");
        let error = validate_runtime_action_result(&result)
            .unwrap_err()
            .to_string();
        assert!(error.contains("runtime action result app"));
        let result = sample_runtime_action_result()
            .replace("\"longRunning\": true", "\"long_running\": true");
        let error = validate_runtime_action_result(&result)
            .unwrap_err()
            .to_string();
        assert!(error.contains("operation.longRunning boolean required"));
        let result =
            sample_runtime_action_result().replace("\"progress\": 100", "\"progress\": 101");
        let error = validate_runtime_action_result(&result)
            .unwrap_err()
            .to_string();
        assert!(error.contains("progress must be 0..100"));
    }

    #[test]
    fn validates_operation_history_contract() {
        let history = sample_operation_history();
        let summary = validate_operation_history(&history).unwrap();
        assert_eq!(summary.app_id, "deployments");
        assert_eq!(summary.entries, 1);
    }

    #[test]
    fn validates_operation_status_contract() {
        let status = sample_operation_status();
        let summary = validate_operation_status(&status).unwrap();
        assert_eq!(summary.app_id, "deployments");
        assert_eq!(summary.operation_id, "deployments-publish_changes-123");
        assert!(summary.long_running);
    }

    #[test]
    fn operation_status_validation_requires_typed_operation() {
        let status = sample_operation_status().replace("\"progress\": 100", "\"progress\": 101");
        let error = validate_operation_status(&status).unwrap_err().to_string();
        assert!(error.contains("progress must be 0..100"));
        let status =
            sample_operation_status().replace("\"schema\": \"theurgy-operation-status/v1\",", "");
        let error = validate_operation_status(&status).unwrap_err().to_string();
        assert!(error.contains("expected schema = theurgy-operation-status/v1"));
    }

    #[test]
    fn operation_history_validation_requires_array_data() {
        let history = sample_operation_history().replace("\"data\": [", "\"data\": {");
        assert!(validate_operation_history(&history).is_err());
        let history =
            sample_operation_history().replace("\"generatedAt\": \"2026-06-11T00:00:00Z\",", "");
        let error = validate_operation_history(&history)
            .unwrap_err()
            .to_string();
        assert!(error.contains("generatedAt required"));
    }

    #[test]
    fn action_ir_schema_declares_typed_action_contract() {
        let schema: Value =
            serde_json::from_str(include_str!("../schemas/theurgy-action-ir-v1.json")).unwrap();
        assert_eq!(
            schema
                .pointer("/properties/actions/items/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/action")
        );
        let required = schema
            .pointer("/$defs/action/required")
            .and_then(Value::as_array)
            .unwrap();
        for key in [
            "id",
            "label",
            "input",
            "output",
            "effect",
            "failure",
            "safe",
            "mutating",
            "longRunning",
            "privileged",
        ] {
            assert!(required.iter().any(|value| value.as_str() == Some(key)));
        }
        assert_eq!(
            schema
                .pointer("/$defs/action/properties/safe/type")
                .and_then(Value::as_str),
            Some("boolean")
        );
        assert_eq!(
            schema
                .pointer("/$defs/action/properties/input/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/shape")
        );
        assert_eq!(
            schema
                .pointer("/$defs/shape/additionalProperties/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/shapeDescriptor")
        );
        assert_eq!(
            schema
                .pointer("/$defs/shapeDescriptor/type")
                .and_then(Value::as_str),
            Some("string")
        );
    }

    #[test]
    fn product_ir_schema_declares_shape_descriptors() {
        let schema: Value =
            serde_json::from_str(include_str!("../schemas/theurgy-product-ir-v1.json")).unwrap();
        assert_eq!(
            schema
                .pointer("/$defs/action/properties/output/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/shape")
        );
        assert_eq!(
            schema
                .pointer("/$defs/action/properties/failure/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/shape")
        );
        assert_eq!(
            schema
                .pointer("/$defs/action/properties/command/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/command")
        );
        assert_eq!(
            schema
                .pointer("/properties/surfaces/properties/desktop/type")
                .and_then(Value::as_str),
            Some("string")
        );
        assert_eq!(
            schema
                .pointer("/properties/surfaces/properties/mobile/minLength")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            schema
                .pointer("/$defs/shape/additionalProperties/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/shapeDescriptor")
        );
        assert_eq!(
            schema
                .pointer("/properties/backgroundJobs/items/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/backgroundJob")
        );
        assert_eq!(
            schema
                .pointer("/properties/releaseTargets/items/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/releaseTarget")
        );
        let top_level_required = schema
            .pointer("/required")
            .and_then(Value::as_array)
            .unwrap();
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("releaseTargets")));
        assert_eq!(
            schema
                .pointer("/properties/persistence/required/0")
                .and_then(Value::as_str),
            Some("truth")
        );
    }

    #[test]
    fn state_snapshot_schema_uses_product_app_slug() {
        let schema: Value =
            serde_json::from_str(include_str!("../schemas/theurgy-state-snapshot-v1.json"))
                .unwrap();
        assert_eq!(
            schema
                .pointer("/properties/app/pattern")
                .and_then(Value::as_str),
            Some("^[a-z][a-z0-9-]*$")
        );
        assert_eq!(
            schema
                .pointer("/properties/data/type")
                .and_then(Value::as_str),
            Some("object")
        );
    }

    #[test]
    fn runtime_status_schema_uses_state_ready_boolean() {
        let schema: Value =
            serde_json::from_str(include_str!("../schemas/theurgy-runtime-status-v1.json"))
                .unwrap();
        assert_eq!(
            schema
                .pointer("/properties/app/pattern")
                .and_then(Value::as_str),
            Some("^[a-z][a-z0-9-]*$")
        );
        assert_eq!(
            schema
                .pointer("/properties/state_ready/type")
                .and_then(Value::as_str),
            Some("boolean")
        );
    }

    #[test]
    fn runtime_action_result_schema_declares_operation_contract() {
        let schema: Value = serde_json::from_str(include_str!(
            "../schemas/theurgy-runtime-action-result-v1.json"
        ))
        .unwrap();
        assert_eq!(
            schema
                .pointer("/properties/app/pattern")
                .and_then(Value::as_str),
            Some("^[a-z][a-z0-9-]*$")
        );
        assert_eq!(
            schema
                .pointer("/properties/operation/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/operation")
        );
        assert_eq!(
            schema
                .pointer("/$defs/operation/properties/longRunning/type")
                .and_then(Value::as_str),
            Some("boolean")
        );
        assert_eq!(
            schema
                .pointer("/$defs/operation/properties/progress/maximum")
                .and_then(Value::as_u64),
            Some(100)
        );
    }

    #[test]
    fn runtime_action_request_schema_declares_action_envelope_contract() {
        let schema: Value = serde_json::from_str(include_str!(
            "../schemas/theurgy-runtime-action-request-v1.json"
        ))
        .unwrap();
        assert_eq!(
            schema
                .pointer("/properties/protocol/const")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action/v1")
        );
        assert_eq!(
            schema
                .pointer("/properties/app/pattern")
                .and_then(Value::as_str),
            Some("^[a-z][a-z0-9-]*$")
        );
        assert_eq!(
            schema
                .pointer("/properties/action/pattern")
                .and_then(Value::as_str),
            Some("^[a-z][a-z0-9_.-]*$")
        );
        assert_eq!(
            schema
                .pointer("/properties/params/type")
                .and_then(Value::as_str),
            Some("object")
        );
    }

    #[test]
    fn surface_ir_schemas_allow_family_targets() {
        let desktop_schema: Value = serde_json::from_str(include_str!(
            "../schemas/theurgy-desktop-surface-ir-v1.json"
        ))
        .unwrap();
        assert_eq!(
            desktop_schema
                .pointer("/properties/target/enum/0")
                .and_then(Value::as_str),
            Some("desktop")
        );
        let mobile_schema: Value =
            serde_json::from_str(include_str!("../schemas/theurgy-mobile-surface-ir-v1.json"))
                .unwrap();
        assert_eq!(
            mobile_schema
                .pointer("/properties/target/enum/0")
                .and_then(Value::as_str),
            Some("mobile")
        );
    }

    #[test]
    fn operation_status_schema_declares_operation_contract() {
        let schema: Value =
            serde_json::from_str(include_str!("../schemas/theurgy-operation-status-v1.json"))
                .unwrap();
        assert_eq!(
            schema
                .pointer("/properties/operation/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/operation")
        );
        assert_eq!(
            schema
                .pointer("/$defs/operation/properties/progress/maximum")
                .and_then(Value::as_u64),
            Some(100)
        );
    }

    #[test]
    fn operation_history_schema_uses_array_data() {
        let schema: Value =
            serde_json::from_str(include_str!("../schemas/theurgy-operation-history-v1.json"))
                .unwrap();
        assert_eq!(
            schema
                .pointer("/properties/data/type")
                .and_then(Value::as_str),
            Some("array")
        );
    }

    #[test]
    fn generated_runtime_schema_declares_action_contracts() {
        let schema: Value =
            serde_json::from_str(include_str!("../schemas/theurgy-generated-runtime-v1.json"))
                .unwrap();
        assert_eq!(
            schema
                .pointer("/properties/productActionContracts/items/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/actionContract")
        );
        assert_eq!(
            schema
                .pointer("/properties/surfaceActionContracts/items/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/actionContract")
        );
        assert_eq!(
            schema
                .pointer("/properties/subscribeStatusCommand/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/command")
        );
        let top_level_required = schema
            .pointer("/required")
            .and_then(Value::as_array)
            .unwrap();
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("subscribeStatusCommand")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("productIr")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("runtimeManifest")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("sourceSurfaceIr")));
        assert!(top_level_required.iter().any(|value| {
            value.as_str() == Some("compatibilityWizardryAppsShellFirstStillSupported")
        }));
        assert!(top_level_required.iter().any(|value| {
            value.as_str() == Some("compatibilityTheurgyRequiredForLegacyWizardryApps")
        }));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("productStateSnapshotSchema")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("productPersistenceTruth")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("targetReleaseTarget")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("targetReleaseArtifact")));
        assert_eq!(
            schema
                .pointer("/properties/legacyNativeDesktopIr/minLength")
                .and_then(Value::as_i64),
            Some(1)
        );
        assert_eq!(
            schema
                .pointer("/properties/compatibilityWizardryAppsShellFirstStillSupported/type")
                .and_then(Value::as_str),
            Some("boolean")
        );
        assert_eq!(
            schema
                .pointer("/properties/compatibilityTheurgyRequiredForLegacyWizardryApps/type")
                .and_then(Value::as_str),
            Some("boolean")
        );
        assert_eq!(
            schema
                .pointer("/properties/targetReleaseTarget/pattern")
                .and_then(Value::as_str),
            Some("^[a-z][a-z0-9_.-]*$")
        );
        assert_eq!(
            schema
                .pointer("/properties/productIr/minLength")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            schema
                .pointer("/properties/productStateSnapshotSchema/minLength")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            schema
                .pointer("/properties/productPersistenceTruth/minLength")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            schema
                .pointer("/allOf/0/then/properties/surfaceSchema/const")
                .and_then(Value::as_str),
            Some("theurgy-desktop-surface-ir/v1")
        );
        assert_eq!(
            schema
                .pointer("/allOf/1/then/properties/surfaceSchema/const")
                .and_then(Value::as_str),
            Some("theurgy-mobile-surface-ir/v1")
        );
        assert_eq!(
            schema
                .pointer("/$defs/actionContract/properties/inputShape/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/shape")
        );
        let required = schema
            .pointer("/$defs/actionContract/required")
            .and_then(Value::as_array)
            .unwrap();
        for key in [
            "id",
            "label",
            "effect",
            "safe",
            "mutating",
            "longRunning",
            "inputShape",
            "outputShape",
            "failureShape",
        ] {
            assert!(required.iter().any(|value| value.as_str() == Some(key)));
        }
    }

    #[test]
    fn runtime_manifest_validation_requires_string_arrays() {
        let manifest = sample_runtime_manifest().replace(
            "\"stateCommand\": [\"custom-core\", \"state\"]",
            "\"stateCommand\": [\"custom-core\", 7]",
        );
        let error = runtime_contract_from_manifest(&manifest)
            .unwrap_err()
            .to_string();
        assert!(error.contains("stateCommand must contain strings"));
        let manifest = sample_runtime_manifest().replace(
            "\"desktop\": \"app-blueprint/desktop.surface.ir.json\"",
            "\"desktop\": []",
        );
        let error = runtime_contract_from_manifest(&manifest)
            .unwrap_err()
            .to_string();
        assert!(error.contains("surfaces.desktop must be a non-empty string"));
    }

    #[test]
    fn runtime_manifest_validation_rejects_empty_subscribe_status_command() {
        let manifest = sample_runtime_manifest().replace(
            "\"statusCommand\": [\"custom-core\", \"status\"]",
            "\"statusCommand\": [\"custom-core\", \"status\"],\n    \"subscribeStatusCommand\": []",
        );
        let error = runtime_contract_from_manifest(&manifest)
            .unwrap_err()
            .to_string();
        assert_eq!(
            error,
            "runtime manifest subscribeStatusCommand must be non-empty"
        );
    }

    #[test]
    fn runtime_manifest_validation_rejects_invalid_compatibility_flags() {
        let manifest = sample_runtime_manifest().replace(
            "\"compatibility\": {\n    \"wizardryAppsShellFirstStillSupported\": true,\n    \"theurgyRequiredForLegacyWizardryApps\": false\n  }",
            "\"compatibility\": []",
        );
        let error = validate_runtime_manifest(&manifest)
            .unwrap_err()
            .to_string();
        assert!(error.contains("runtime manifest compatibility must be an object"));
        let manifest = sample_runtime_manifest().replace(
            "\"wizardryAppsShellFirstStillSupported\": true",
            "\"wizardryAppsShellFirstStillSupported\": \"yes\"",
        );
        let error = validate_runtime_manifest(&manifest)
            .unwrap_err()
            .to_string();
        assert!(error.contains(
            "runtime manifest compatibility.wizardryAppsShellFirstStillSupported must be boolean"
        ));
    }

    #[test]
    fn action_params_must_be_json_object_or_array() {
        assert!(validate_json_params("{\"ok\":true}").is_ok());
        assert!(validate_json_params("[1,2]").is_ok());
        assert!(validate_json_params("\"scalar\"").is_err());
        assert!(validate_json_params("{").is_err());
    }

    #[test]
    fn validates_runtime_action_request_contract() {
        let request = "{\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"publish_changes\",\n  \"params\": {\"deployment\": \"site-one\"}\n}";
        let summary = validate_runtime_action_request(request).unwrap();
        assert_eq!(summary.app_id, "deployments");
        assert_eq!(summary.action_id, "publish_changes");

        let error = validate_runtime_action_request(
            "{\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"Deployments\",\n  \"action\": \"publish_changes\",\n  \"params\": {}\n}",
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("runtime action request app must be a lowercase slug"));

        let error = validate_runtime_action_request(
            "{\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"publish_changes\",\n  \"params\": []\n}",
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("missing JSON object key: params"));
    }

    #[test]
    fn runtime_action_request_validation_uses_manifest_contract() {
        let root = runtime_fixture_root("validate-action-request");
        let manifest = root.join("runtime.manifest.json");
        let valid_request = root.join("valid-action-request.json");
        write_or_replace(
            &valid_request,
            "{\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"refresh_state\",\n  \"params\": {}\n}",
        )
        .unwrap();
        command_validate_runtime_action_request(&[
            valid_request.display().to_string(),
            "--manifest".to_string(),
            manifest.display().to_string(),
        ])
        .unwrap();

        let app_mismatch = root.join("app-mismatch-action-request.json");
        write_or_replace(
            &app_mismatch,
            "{\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"other-app\",\n  \"action\": \"refresh_state\",\n  \"params\": {}\n}",
        )
        .unwrap();
        let error = command_validate_runtime_action_request(&[
            app_mismatch.display().to_string(),
            "--manifest".to_string(),
            manifest.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "runtime action request app mismatch: expected deployments, got other-app"
        );

        let undeclared_action = root.join("undeclared-action-request.json");
        write_or_replace(
            &undeclared_action,
            "{\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"not_declared\",\n  \"params\": {}\n}",
        )
        .unwrap();
        let error = command_validate_runtime_action_request(&[
            undeclared_action.display().to_string(),
            "--manifest".to_string(),
            manifest.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "runtime action request not declared in Product IR: not_declared"
        );

        let undeclared_param = root.join("undeclared-param-action-request.json");
        write_or_replace(
            &undeclared_param,
            "{\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"refresh_state\",\n  \"params\": {\"force\": true}\n}",
        )
        .unwrap();
        let error = command_validate_runtime_action_request(&[
            undeclared_param.display().to_string(),
            "--manifest".to_string(),
            manifest.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "runtime action param not declared in Product IR input for refresh_state: force"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_action_without_manifest_returns_protocol_envelope() {
        let output = run_action_output("refresh_state", "{\"force\":true}", None).unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(value.get("success").and_then(Value::as_bool), Some(true));
        assert_eq!(
            value.get("protocol").and_then(Value::as_str),
            Some("theurgy-runtime-action/v1")
        );
        assert_eq!(
            value.get("app").and_then(Value::as_str),
            Some("theurgy-runtime")
        );
        assert_eq!(
            value.get("action").and_then(Value::as_str),
            Some("refresh_state")
        );
        assert_eq!(
            value.pointer("/params/force").and_then(Value::as_bool),
            Some(true)
        );
    }

    #[test]
    fn run_action_with_manifest_dispatches_action_command() {
        let root = runtime_fixture_root("run-action");
        let manifest = root.join("runtime.manifest.json");

        let output = run_action_output("refresh_state", "{}", Some(&manifest)).unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(value.get("success").and_then(Value::as_bool), Some(true));
        assert_eq!(
            value.pointer("/data/action").and_then(Value::as_str),
            Some("refresh_state")
        );
        assert_eq!(
            value
                .pointer("/data/result/params")
                .and_then(Value::as_object)
                .map(|params| params.len()),
            Some(0)
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_action_with_manifest_rejects_long_running_mismatch() {
        let root = runtime_fixture_root("run-action-long-running");
        let manifest = root.join("runtime.manifest.json");

        let error = run_action_output(
            "publish_changes",
            "{\"deployment\":\"site-one\"}",
            Some(&manifest),
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime action operation.longRunning mismatch for publish_changes: expected true, got false"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn manifest_action_output_rejects_action_mismatch() {
        let output = sample_runtime_action_result().replace(
            "\"action\": \"publish_changes\"",
            "\"action\": \"refresh_state\"",
        );
        let error =
            validate_manifest_action_output("deployments", "publish_changes", &output, None)
                .unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime action result action mismatch: expected publish_changes, got refresh_state"
        );
    }

    #[test]
    fn manifest_action_output_rejects_app_mismatch() {
        let output = sample_runtime_action_result()
            .replace("\"app\": \"deployments\"", "\"app\": \"other-app\"");
        let error =
            validate_manifest_action_output("deployments", "publish_changes", &output, None)
                .unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime action result app mismatch: expected deployments, got other-app"
        );
    }

    #[test]
    fn run_action_with_manifest_rejects_undeclared_product_action() {
        let root = runtime_fixture_root("run-action-undeclared");
        let manifest = root.join("runtime.manifest.json");

        let error = run_action_output("delete_everything", "{}", Some(&manifest)).unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime action not declared in Product IR: delete_everything"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_action_with_manifest_rejects_undeclared_product_param() {
        let root = runtime_fixture_root("run-action-undeclared-param");
        let manifest = root.join("runtime.manifest.json");

        let error = run_action_output("refresh_state", "{\"unexpected\":true}", Some(&manifest))
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime action param not declared in Product IR input for refresh_state: unexpected"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_action_with_manifest_rejects_param_type_mismatch() {
        let root = runtime_fixture_root("run-action-param-type");
        let manifest = root.join("runtime.manifest.json");

        let error = run_action_output("publish_changes", "{\"deployment\":false}", Some(&manifest))
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime action param type mismatch for publish_changes.deployment: expected string"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_action_with_manifest_rejects_undeclared_result_key() {
        let root = runtime_fixture_root("run-action-undeclared-result");
        write_or_replace(
            &root.join("app-blueprint/product.ir.json"),
            &sample_product().replace("\"output\": {\"params\": \"object\"}", "\"output\": {}"),
        )
        .unwrap();
        let manifest = root.join("runtime.manifest.json");

        let error = run_action_output("refresh_state", "{}", Some(&manifest)).unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime action result key not declared in Product IR output for refresh_state: params"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_action_with_manifest_rejects_result_type_mismatch() {
        let root = runtime_fixture_root("run-action-result-type");
        write_or_replace(
            &root.join("app-blueprint/product.ir.json"),
            &sample_product().replace("\"params\": \"object\"", "\"params\": \"string\""),
        )
        .unwrap();
        let manifest = root.join("runtime.manifest.json");

        let error = run_action_output("refresh_state", "{}", Some(&manifest)).unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime action result type mismatch for refresh_state.params: expected string"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn runtime_action_failure_keys_follow_product_contract() {
        let contracts = validate_product_ir(
            &sample_product().replace("\"failure\": {}", "\"failure\": {\"error\": \"string\"}"),
        )
        .unwrap()
        .action_contracts;
        validate_runtime_action_failure_keys(
            "refresh_state",
            "{\"success\":false,\"error\":\"failed\"}",
            &contracts,
        )
        .unwrap();
        let error = validate_runtime_action_failure_keys(
            "refresh_state",
            "{\"success\":false,\"error\":\"failed\",\"extra\":true}",
            &contracts,
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime action failure key not declared in Product IR failure for refresh_state: extra"
        );
        let error = validate_runtime_action_failure_keys(
            "refresh_state",
            "{\"success\":false,\"error\":false}",
            &contracts,
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime action failure type mismatch for refresh_state.error: expected string"
        );
    }

    #[test]
    fn run_action_with_manifest_requires_object_params_for_product_action() {
        let root = runtime_fixture_root("run-action-array-param");
        let manifest = root.join("runtime.manifest.json");

        let error = run_action_output("refresh_state", "[]", Some(&manifest)).unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime action params must be a JSON object for Product IR action: refresh_state"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_state_with_manifest_dispatches_state_command() {
        let root = runtime_fixture_root("run-state");
        let output = run_state_output(&root.join("runtime.manifest.json")).unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.get("schema").and_then(Value::as_str),
            Some("theurgy-state-snapshot/v1")
        );
        assert_eq!(
            value.get("app").and_then(Value::as_str),
            Some("deployments")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_status_with_manifest_dispatches_status_command() {
        let root = runtime_fixture_root("run-status");
        let output = run_status_output(&root.join("runtime.manifest.json")).unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.get("schema").and_then(Value::as_str),
            Some("theurgy-runtime-status/v1")
        );
        assert_eq!(
            value.get("state_ready").and_then(Value::as_bool),
            Some(true)
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn subscribe_status_once_reuses_typed_status_contract() {
        let root = runtime_fixture_root("subscribe-status");
        let manifest = root.join("runtime.manifest.json");
        let output = subscribe_status_output(&manifest).unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.get("schema").and_then(Value::as_str),
            Some("theurgy-runtime-status/v1")
        );
        assert_eq!(
            value.get("state_ready").and_then(Value::as_bool),
            Some(true)
        );

        command_subscribe_status(&[
            "--manifest".to_string(),
            manifest.display().to_string(),
            "--once".to_string(),
        ])
        .unwrap();

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn subscribe_status_once_uses_explicit_manifest_command() {
        let root = runtime_fixture_root("subscribe-status-explicit");
        let manifest = root.join("runtime.manifest.json");
        let runtime = root.join("runtime-fixture").display().to_string();
        let manifest_text = fs::read_to_string(&manifest).unwrap().replace(
            "\"statusCommand\": [",
            &format!(
                "\"subscribeStatusCommand\": [\"{}\", \"subscribe\"],\n    \"statusCommand\": [",
                json_escape(&runtime)
            ),
        );
        write_or_replace(&manifest, &manifest_text).unwrap();

        let output = subscribe_status_output(&manifest).unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.get("schema").and_then(Value::as_str),
            Some("theurgy-runtime-status/v1")
        );
        assert_eq!(
            value.get("state_ready").and_then(Value::as_bool),
            Some(false)
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_operation_status_with_manifest_dispatches_operation_status_command() {
        let root = runtime_fixture_root("run-operation-status");
        let manifest = root.join("runtime.manifest.json");
        let output =
            run_operation_status_output(&manifest, "deployments-publish_changes-123").unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.get("schema").and_then(Value::as_str),
            Some("theurgy-operation-status/v1")
        );
        assert_eq!(
            value.pointer("/operation/id").and_then(Value::as_str),
            Some("deployments-publish_changes-123")
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn subscribe_status_requires_once_until_streaming_is_implemented() {
        let root = runtime_fixture_root("subscribe-status-requires-once");
        let error = command_subscribe_status(&[
            "--manifest".to_string(),
            root.join("runtime.manifest.json").display().to_string(),
        ])
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "usage: subscribe-status --manifest PATH --once"
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_history_with_manifest_dispatches_history_command() {
        let root = runtime_fixture_root("run-history");
        let output =
            run_history_output(&root.join("runtime.manifest.json"), "site-one", Some("12"))
                .unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.get("schema").and_then(Value::as_str),
            Some("theurgy-operation-history/v1")
        );
        assert_eq!(
            value.get("subject").and_then(Value::as_str),
            Some("site-one")
        );
        assert_eq!(value.get("limit").and_then(Value::as_str), Some("12"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn projects_desktop_surface_from_product_ir() {
        let surface = project_surface(&sample_product(), "macos").unwrap();
        assert!(surface.contains("\"version\": \"theurgy-desktop-surface-ir/v1\""));
        assert!(surface.contains("\"product\": \"deployments\""));
        assert!(surface.contains("\"role\": \"left-list-detail\""));
        let summary = validate_surface_ir(&surface).unwrap();
        assert_eq!(
            summary.action_ids,
            vec!["refresh_state".to_string(), "publish_changes".to_string()]
        );
        assert!(summary.roles.contains(&"left-list-detail".to_string()));
        assert!(summary.roles.contains(&"product-navigation".to_string()));
    }

    #[test]
    fn projects_mobile_surface_from_product_ir() {
        let product = "{\n  \"version\": \"theurgy-product-ir/v1\",\n  \"format\": \"json\",\n  \"app\": {\"id\": \"deployments\", \"name\": \"Deployments\", \"targets\": [\"ios\"]},\n  \"actions\": [{\"id\": \"refresh_state\", \"label\": \"Refresh\", \"input\": {}, \"output\": {}, \"effect\": \"read\", \"failure\": {}, \"safe\": true, \"mutating\": false, \"longRunning\": false, \"privileged\": false}],\n  \"state\": {\"snapshotSchema\": \"deployments-state/v1\"},\n  \"releaseTargets\": [{\"id\": \"ios-app\", \"target\": \"ios\", \"surface\": \"mobile\", \"artifact\": \"generated/mobile/ios\"}]\n}".to_string();
        let surface = project_surface(&product, "ios").unwrap();
        assert!(surface.contains("\"version\": \"theurgy-mobile-surface-ir/v1\""));
        assert!(surface.contains("\"role\": \"status-overview\""));
        let summary = validate_surface_ir(&surface).unwrap();
        assert_eq!(summary.action_ids, vec!["refresh_state".to_string()]);
        assert!(summary.roles.contains(&"status-overview".to_string()));
        assert!(summary.roles.contains(&"focused-action-detail".to_string()));
    }

    #[test]
    fn surface_ir_validation_rejects_target_family_drift() {
        let desktop =
            sample_desktop_surface().replace("\"target\": \"desktop\"", "\"target\": \"ios\"");
        let error = validate_surface_ir(&desktop).unwrap_err().to_string();
        assert_eq!(error, "desktop surface IR target invalid");
        let mobile = "{\n  \"version\": \"theurgy-mobile-surface-ir/v1\",\n  \"format\": \"json\",\n  \"product\": \"deployments\",\n  \"target\": \"linux\",\n  \"actions\": [\"refresh_state\"],\n  \"screens\": [{\"id\": \"overview\", \"title\": \"Deployments\", \"node\": {\"id\": \"screen.overview\", \"type\": \"NavigationStack\", \"role\": \"status-overview\"}}]\n}\n";
        let error = validate_surface_ir(mobile).unwrap_err().to_string();
        assert_eq!(error, "mobile surface IR target invalid");
    }

    #[test]
    fn compile_native_emits_deterministic_adapter_files() {
        let root = test_root("compile-native");
        compile_native(&sample_product(), "linux", &root).unwrap();
        assert!(root.join("theurgy-surface.json").exists());
        assert!(root.join("theurgy-runtime.json").exists());
        let runtime = fs::read_to_string(root.join("theurgy-runtime.json")).unwrap();
        let runtime_json: Value = serde_json::from_str(&runtime).unwrap();
        let generated = validate_generated_runtime(&runtime).unwrap();
        assert_eq!(generated.app_id, "deployments");
        assert_eq!(generated.target, "linux");
        assert_eq!(generated.state_snapshot_schema, "deployments-state/v1");
        assert_eq!(
            runtime_json.get("productIr").and_then(Value::as_str),
            Some("direct-product-ir")
        );
        assert_eq!(
            runtime_json.get("runtimeManifest").and_then(Value::as_str),
            Some("generated-runtime-manifest")
        );
        assert_eq!(
            runtime_json.get("sourceSurfaceIr").and_then(Value::as_str),
            Some("projected-surface-ir")
        );
        assert_eq!(
            runtime_json
                .get("productStateSnapshotSchema")
                .and_then(Value::as_str),
            Some("deployments-state/v1")
        );
        assert_eq!(generated.persistence_truth, "file-first");
        assert_eq!(generated.adapter_runtime_transport, "local-process-json");
        assert_eq!(generated.runtime_status_schema, "theurgy-runtime-status/v1");
        assert_eq!(
            generated.runtime_action_request_schema,
            "theurgy-runtime-action-request/v1"
        );
        assert_eq!(
            generated.runtime_action_result_schema,
            "theurgy-runtime-action-result/v1"
        );
        assert_eq!(
            generated.operation_status_schema,
            "theurgy-operation-status/v1"
        );
        assert_eq!(
            generated.operation_history_schema,
            "theurgy-operation-history/v1"
        );
        assert_eq!(generated.surface_schema, "theurgy-desktop-surface-ir/v1");
        assert_eq!(generated.surface_target, "linux");
        assert_eq!(
            runtime_json
                .get("productPersistenceTruth")
                .and_then(Value::as_str),
            Some("file-first")
        );
        assert_eq!(
            runtime_json
                .get("compatibilityWizardryAppsShellFirstStillSupported")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            runtime_json
                .get("compatibilityTheurgyRequiredForLegacyWizardryApps")
                .and_then(Value::as_bool),
            Some(false)
        );
        assert_eq!(generated.actions, 2);
        assert_eq!(generated.product_actions, 2);
        assert_eq!(generated.surface_actions, 2);
        assert_eq!(generated.surface_action_contracts, 2);
        assert_eq!(
            runtime_json.get("stateCommand").unwrap(),
            &serde_json::json!(["deployments-core", "runtime-state"])
        );
        assert_eq!(
            runtime_json.get("subscribeStatusCommand").unwrap(),
            &serde_json::json!(["deployments-core", "runtime-status"])
        );
        assert_eq!(
            runtime_json.get("operationStatusCommand").unwrap(),
            &serde_json::json!(["deployments-core", "runtime-operation-status"])
        );
        assert_eq!(
            runtime_json
                .get("runtimeStatusSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-status/v1")
        );
        assert_eq!(
            runtime_json
                .get("runtimeActionRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action-request/v1")
        );
        assert_eq!(
            runtime_json
                .get("runtimeActionResultSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action-result/v1")
        );
        assert_eq!(
            runtime_json
                .get("operationStatusSchema")
                .and_then(Value::as_str),
            Some("theurgy-operation-status/v1")
        );
        assert_eq!(
            runtime_json
                .get("operationHistorySchema")
                .and_then(Value::as_str),
            Some("theurgy-operation-history/v1")
        );
        assert_eq!(
            runtime_json
                .get("adapterRuntimeTransport")
                .and_then(Value::as_str),
            Some("local-process-json")
        );
        assert_eq!(
            runtime_json.get("surfaceSchema").and_then(Value::as_str),
            Some("theurgy-desktop-surface-ir/v1")
        );
        assert_eq!(
            runtime_json.get("surfaceActions").unwrap(),
            &serde_json::json!(["refresh_state", "publish_changes"])
        );
        assert_eq!(
            runtime_json.get("surfaceRoles").unwrap(),
            &serde_json::json!([
                "left-list-detail",
                "native-product-root",
                "product-detail",
                "product-navigation"
            ])
        );
        assert_eq!(
            runtime_json.get("productCapabilities").unwrap(),
            &serde_json::json!(["native-desktop", "runtime-actions"])
        );
        assert_eq!(
            runtime_json
                .pointer("/productActionContracts/1/id")
                .and_then(Value::as_str),
            Some("publish_changes")
        );
        assert_eq!(
            runtime_json
                .pointer("/productActionContracts/1/inputKeys")
                .unwrap(),
            &serde_json::json!(["deployment"])
        );
        assert_eq!(
            runtime_json
                .pointer("/productActionContracts/1/inputShape/deployment")
                .and_then(Value::as_str),
            Some("string")
        );
        assert_eq!(
            runtime_json
                .pointer("/productActionContracts/1/longRunning")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            runtime_json
                .pointer("/surfaceActionContracts/1/id")
                .and_then(Value::as_str),
            Some("publish_changes")
        );
        assert_eq!(
            runtime_json
                .pointer("/surfaceActionContracts/1/inputKeys")
                .unwrap(),
            &serde_json::json!(["deployment"])
        );
        assert_eq!(
            runtime_json.get("productDomainObjects").unwrap(),
            &serde_json::json!(["server", "deployment"])
        );
        assert_eq!(
            runtime_json.get("productBackgroundJobs").unwrap(),
            &serde_json::json!(["server-queue"])
        );
        assert_eq!(
            runtime_json.get("productReleaseTargets").unwrap(),
            &serde_json::json!(["macos-app", "linux-app"])
        );
        assert_eq!(generated.release_target, "linux-app".to_string());
        assert_eq!(generated.release_artifact, "generated/linux".to_string());
        assert_eq!(
            runtime_json
                .get("targetReleaseTarget")
                .and_then(Value::as_str),
            Some("linux-app")
        );
        assert_eq!(
            runtime_json
                .get("targetReleaseArtifact")
                .and_then(Value::as_str),
            Some("generated/linux")
        );
        let main_c = fs::read_to_string(root.join("src/main.c")).unwrap();
        assert!(main_c.contains("gtk_application_window_new"));
        assert!(main_c.contains("Deployments"));
        assert!(main_c.contains("theurgy-runtime.json"));
        assert!(main_c.contains("runtime-state"));
        assert!(main_c.contains("subscribe_runtime_status"));
        assert!(main_c.contains("runtime-status"));
        assert!(main_c
            .contains("GtkWidget *subscribe_button = gtk_button_new_with_label(\"Subscribe\")"));
        assert!(main_c.contains("g_subprocess_newv"));
        assert!(main_c.contains("json-glib/json-glib.h"));
        assert!(main_c.contains("surface_action_contracts_json"));
        assert!(main_c.contains("\\\"inputShape\\\""));
        assert!(main_c.contains("\\\"deployment\\\":\\\"string\\\""));
        assert!(main_c.contains("Surface action contracts: refresh_state, publish_changes"));
        assert!(main_c.contains("static char *run_default_action(void)"));
        assert!(main_c.contains("\"runtime-action\""));
        assert!(main_c.contains("\"refresh_state\", \"{}\", NULL"));
        assert!(main_c.contains("GtkWidget *action_button = gtk_button_new_with_label(\"Action\")"));
        assert!(main_c.contains("G_CALLBACK(run_action)"));
        let meson = fs::read_to_string(root.join("meson.build")).unwrap();
        assert!(meson.contains("json-glib-1.0"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generated_runtime_validation_rejects_action_contract_drift() {
        let root = test_root("generated-runtime-drift");
        compile_native(&sample_product(), "linux", &root).unwrap();
        let runtime = fs::read_to_string(root.join("theurgy-runtime.json"))
            .unwrap()
            .replace("\"id\": \"publish_changes\"", "\"id\": \"not_declared\"");
        let error = validate_generated_runtime(&runtime)
            .unwrap_err()
            .to_string();
        assert!(error.contains("productActionContracts order must match productActions"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generated_runtime_validation_rejects_action_shape_key_drift() {
        let root = test_root("generated-runtime-shape-drift");
        compile_native(&sample_product(), "linux", &root).unwrap();
        let mut runtime_json: Value =
            serde_json::from_str(&fs::read_to_string(root.join("theurgy-runtime.json")).unwrap())
                .unwrap();
        let shape = runtime_json
            .pointer_mut("/productActionContracts/1/inputShape")
            .and_then(Value::as_object_mut)
            .unwrap();
        shape.insert("other".to_string(), Value::String("string".to_string()));
        let runtime = serde_json::to_string(&runtime_json).unwrap();
        let error = validate_generated_runtime(&runtime)
            .unwrap_err()
            .to_string();
        assert!(error.contains("inputKeys must match inputShape keys"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generated_runtime_validation_requires_source_identity() {
        for (key, message) in [
            ("productIr", "productIr required"),
            ("runtimeManifest", "runtimeManifest required"),
            ("sourceSurfaceIr", "sourceSurfaceIr required"),
            (
                "productStateSnapshotSchema",
                "productStateSnapshotSchema required",
            ),
            (
                "productPersistenceTruth",
                "productPersistenceTruth required",
            ),
            (
                "compatibilityWizardryAppsShellFirstStillSupported",
                "compatibilityWizardryAppsShellFirstStillSupported boolean required",
            ),
            (
                "compatibilityTheurgyRequiredForLegacyWizardryApps",
                "compatibilityTheurgyRequiredForLegacyWizardryApps boolean required",
            ),
        ] {
            let root = test_root(&format!("generated-runtime-source-{key}"));
            compile_native(&sample_product(), "linux", &root).unwrap();
            let mut runtime_json: Value = serde_json::from_str(
                &fs::read_to_string(root.join("theurgy-runtime.json")).unwrap(),
            )
            .unwrap();
            runtime_json.as_object_mut().unwrap().remove(key);
            let runtime = serde_json::to_string(&runtime_json).unwrap();
            let error = validate_generated_runtime(&runtime)
                .unwrap_err()
                .to_string();
            assert!(error.contains(message));
            fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn generated_runtime_validation_rejects_surface_schema_target_drift() {
        let root = test_root("generated-runtime-surface-schema-drift");
        compile_native(&sample_product(), "linux", &root).unwrap();
        let mut runtime_json: Value =
            serde_json::from_str(&fs::read_to_string(root.join("theurgy-runtime.json")).unwrap())
                .unwrap();
        *runtime_json.pointer_mut("/surfaceSchema").unwrap() =
            Value::String("theurgy-mobile-surface-ir/v1".to_string());
        let runtime = serde_json::to_string(&runtime_json).unwrap();
        let error = validate_generated_runtime(&runtime)
            .unwrap_err()
            .to_string();
        assert!(error.contains("surfaceSchema invalid for target"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generated_runtime_validation_requires_operation_status_for_long_running_actions() {
        let root = test_root("generated-runtime-operation-status");
        compile_native(&sample_product(), "linux", &root).unwrap();
        let mut runtime_json: Value =
            serde_json::from_str(&fs::read_to_string(root.join("theurgy-runtime.json")).unwrap())
                .unwrap();
        runtime_json
            .as_object_mut()
            .unwrap()
            .remove("operationStatusCommand");
        let runtime = serde_json::to_string(&runtime_json).unwrap();
        let error = validate_generated_runtime(&runtime)
            .unwrap_err()
            .to_string();
        assert!(error.contains("operationStatusCommand required for long-running actions"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generated_runtime_validation_rejects_surface_action_contract_drift() {
        let root = test_root("generated-runtime-surface-drift");
        compile_native(&sample_product(), "linux", &root).unwrap();
        let mut runtime_json: Value =
            serde_json::from_str(&fs::read_to_string(root.join("theurgy-runtime.json")).unwrap())
                .unwrap();
        *runtime_json
            .pointer_mut("/surfaceActionContracts/1/id")
            .unwrap() = Value::String("refresh_state".to_string());
        let runtime = serde_json::to_string(&runtime_json).unwrap();
        let error = validate_generated_runtime(&runtime)
            .unwrap_err()
            .to_string();
        assert!(error.contains("surfaceActionContracts order must match surfaceActions"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generated_runtime_validation_rejects_surface_contract_body_drift() {
        let root = test_root("generated-runtime-surface-body-drift");
        compile_native(&sample_product(), "linux", &root).unwrap();
        let mut runtime_json: Value =
            serde_json::from_str(&fs::read_to_string(root.join("theurgy-runtime.json")).unwrap())
                .unwrap();
        *runtime_json
            .pointer_mut("/surfaceActionContracts/1/privileged")
            .unwrap() = Value::Bool(false);
        let runtime = serde_json::to_string(&runtime_json).unwrap();
        let error = validate_generated_runtime(&runtime)
            .unwrap_err()
            .to_string();
        assert!(error.contains(
            "surfaceActionContracts must match productActionContracts for publish_changes"
        ));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn manifest_state_output_rejects_untyped_stdout() {
        let error = validate_manifest_state_output("deployments", "{\"ok\":true}").unwrap_err();
        assert_eq!(
            error.to_string(),
            "expected schema = theurgy-state-snapshot/v1"
        );
    }

    #[test]
    fn manifest_state_output_rejects_app_mismatch() {
        let output =
            sample_state_snapshot().replace("\"app\": \"deployments\"", "\"app\": \"other-app\"");
        let error = validate_manifest_state_output("deployments", &output).unwrap_err();
        assert_eq!(
            error.to_string(),
            "state snapshot app mismatch: expected deployments, got other-app"
        );
    }

    #[test]
    fn manifest_status_output_rejects_untyped_stdout() {
        let error = validate_manifest_status_output("deployments", "{\"ok\":true}").unwrap_err();
        assert_eq!(
            error.to_string(),
            "expected schema = theurgy-runtime-status/v1"
        );
    }

    #[test]
    fn manifest_status_output_rejects_app_mismatch() {
        let output =
            sample_runtime_status().replace("\"app\": \"deployments\"", "\"app\": \"other-app\"");
        let error = validate_manifest_status_output("deployments", &output).unwrap_err();
        assert_eq!(
            error.to_string(),
            "runtime status app mismatch: expected deployments, got other-app"
        );
    }

    #[test]
    fn manifest_history_output_rejects_untyped_stdout() {
        let error = validate_manifest_history_output("deployments", "{\"ok\":true}").unwrap_err();
        assert_eq!(
            error.to_string(),
            "expected schema = theurgy-operation-history/v1"
        );
    }

    #[test]
    fn manifest_history_output_rejects_app_mismatch() {
        let output = sample_operation_history()
            .replace("\"app\": \"deployments\"", "\"app\": \"other-app\"");
        let error = validate_manifest_history_output("deployments", &output).unwrap_err();
        assert_eq!(
            error.to_string(),
            "operation history app mismatch: expected deployments, got other-app"
        );
    }

    #[test]
    fn compile_app_uses_declared_runtime_manifest_and_surface() {
        let app = test_root("compile-app");
        let out = test_root("compile-app-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/product.ir.json"),
            &sample_product(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();

        command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap();

        let runtime = fs::read_to_string(out.join("theurgy-runtime.json")).unwrap();
        let runtime_json: Value = serde_json::from_str(&runtime).unwrap();
        let generated = validate_generated_runtime(&runtime).unwrap();
        assert_eq!(generated.adapter_runtime_transport, "local-process-json");
        assert_eq!(generated.runtime_status_schema, "theurgy-runtime-status/v1");
        assert_eq!(
            generated.runtime_action_request_schema,
            "theurgy-runtime-action-request/v1"
        );
        assert_eq!(
            generated.runtime_action_result_schema,
            "theurgy-runtime-action-result/v1"
        );
        assert_eq!(
            generated.operation_status_schema,
            "theurgy-operation-status/v1"
        );
        assert_eq!(
            generated.operation_history_schema,
            "theurgy-operation-history/v1"
        );
        assert_eq!(generated.surface_schema, "theurgy-desktop-surface-ir/v1");
        assert_eq!(generated.surface_target, "desktop");
        assert_eq!(
            runtime_json.get("protocol").and_then(Value::as_str),
            Some("deployments-runtime/v1")
        );
        assert_eq!(
            runtime_json.get("stateCommand").unwrap(),
            &serde_json::json!(["custom-core", "state"])
        );
        assert_eq!(
            runtime_json
                .get("legacyNativeDesktopIr")
                .and_then(Value::as_str),
            Some("app-blueprint/app.ir.yaml")
        );
        assert_eq!(
            runtime_json
                .get("compatibilityWizardryAppsShellFirstStillSupported")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            runtime_json
                .get("compatibilityTheurgyRequiredForLegacyWizardryApps")
                .and_then(Value::as_bool),
            Some(false)
        );
        assert_eq!(
            runtime_json.get("statusCommand").unwrap(),
            &serde_json::json!(["custom-core", "status"])
        );
        assert_eq!(
            runtime_json.get("subscribeStatusCommand").unwrap(),
            &serde_json::json!(["custom-core", "status"])
        );
        assert_eq!(
            runtime_json.get("operationStatusCommand").unwrap(),
            &serde_json::json!(["custom-core", "operation-status"])
        );
        assert_eq!(
            runtime_json
                .get("runtimeStatusSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-status/v1")
        );
        assert_eq!(
            runtime_json
                .get("runtimeActionRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action-request/v1")
        );
        assert_eq!(
            runtime_json
                .get("runtimeActionResultSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action-result/v1")
        );
        assert_eq!(
            runtime_json
                .get("operationStatusSchema")
                .and_then(Value::as_str),
            Some("theurgy-operation-status/v1")
        );
        assert_eq!(
            runtime_json
                .get("operationHistorySchema")
                .and_then(Value::as_str),
            Some("theurgy-operation-history/v1")
        );
        assert_eq!(
            runtime_json
                .get("adapterRuntimeTransport")
                .and_then(Value::as_str),
            Some("local-process-json")
        );
        assert_eq!(
            runtime_json.get("historyCommand").unwrap(),
            &serde_json::json!(["custom-core", "history"])
        );
        assert_eq!(
            runtime_json.get("surfaceTarget").and_then(Value::as_str),
            Some("desktop")
        );
        assert_eq!(
            runtime_json.get("surfaceActions").unwrap(),
            &serde_json::json!(["refresh_state", "publish_changes"])
        );
        assert_eq!(
            runtime_json.get("surfaceRoles").unwrap(),
            &serde_json::json!(["declared-reference-surface"])
        );
        let main_c = fs::read_to_string(out.join("src/main.c")).unwrap();
        assert!(main_c.contains("\"custom-core\""));
        assert!(main_c.contains("\"state\""));
        assert!(main_c.contains("Status: custom-core status"));
        assert!(main_c.contains("Operation status: custom-core operation-status"));
        assert!(main_c.contains("Action: custom-core action"));
        assert!(main_c.contains("History: custom-core history"));
        let surface = fs::read_to_string(out.join("theurgy-surface.json")).unwrap();
        assert!(surface.contains("\"role\": \"declared-reference-surface\""));

        fs::remove_dir_all(app).unwrap();
        fs::remove_dir_all(out).unwrap();
    }

    #[test]
    fn compile_app_preserves_existing_legacy_desktop_adapter_sources() {
        let app = test_root("compile-app-preserve-legacy-adapter");
        let out = test_root("compile-app-preserve-legacy-adapter-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        fs::create_dir_all(out.join("src")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/product.ir.json"),
            &sample_product(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();
        write_or_replace(&out.join("meson.build"), "legacy meson\n").unwrap();
        write_or_replace(&out.join("src/main.c"), "legacy linux adapter\n").unwrap();

        command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap();

        assert_eq!(
            fs::read_to_string(out.join("meson.build")).unwrap(),
            "legacy meson\n"
        );
        assert_eq!(
            fs::read_to_string(out.join("src/main.c")).unwrap(),
            "legacy linux adapter\n"
        );
        let runtime = fs::read_to_string(out.join("theurgy-runtime.json")).unwrap();
        assert!(runtime.contains("\"legacyNativeDesktopIr\": \"app-blueprint/app.ir.yaml\""));
        let surface = fs::read_to_string(out.join("theurgy-surface.json")).unwrap();
        assert!(surface.contains("\"role\": \"declared-reference-surface\""));

        fs::remove_dir_all(app).unwrap();
        fs::remove_dir_all(out).unwrap();
    }

    #[test]
    fn compile_app_rejects_surface_actions_missing_from_product_ir() {
        let app = test_root("compile-app-surface-action");
        let out = test_root("compile-app-surface-action-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/product.ir.json"),
            &sample_product(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface().replace(
                "\"actions\": [\"refresh_state\", \"publish_changes\"]",
                "\"actions\": [\"refresh_state\", \"delete_everything\"]",
            ),
        )
        .unwrap();

        let error = command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert!(error.contains("surface IR action not declared in Product IR"));

        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn compile_app_rejects_runtime_manifest_product_ir_drift() {
        let app = test_root("compile-app-product-drift");
        let out = test_root("compile-app-product-drift-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/product.ir.json"),
            &sample_product(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest().replace(
                "\"productIr\": \"app-blueprint/product.ir.json\"",
                "\"productIr\": \"app-blueprint/other-product.ir.json\"",
            ),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();

        let error = command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert!(error.contains("runtime manifest productIr does not match"));

        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn compile_app_rejects_runtime_manifest_surface_drift() {
        let app = test_root("compile-app-surface-drift");
        let out = test_root("compile-app-surface-drift-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/product.ir.json"),
            &sample_product(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest().replace(
                "\"desktop\": \"app-blueprint/desktop.surface.ir.json\"",
                "\"desktop\": \"app-blueprint/other.desktop.surface.ir.json\"",
            ),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();

        let error = command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert!(error.contains("runtime manifest surface path does not match"));

        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn compile_app_rejects_product_surface_drift() {
        let app = test_root("compile-app-product-surface-drift");
        let out = test_root("compile-app-product-surface-drift-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/product.ir.json"),
            &sample_product_with_surfaces("app-blueprint/other.desktop.surface.ir.json"),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();

        let error = command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert!(error.contains("product IR surfaces.desktop does not match declared surface IR"));

        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn compile_app_rejects_surface_schema_target_drift() {
        let app = test_root("compile-app-surface-schema-drift");
        let out = test_root("compile-app-surface-schema-drift-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/product.ir.json"),
            &sample_product(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            "{\n  \"version\": \"theurgy-mobile-surface-ir/v1\",\n  \"format\": \"json\",\n  \"product\": \"deployments\",\n  \"target\": \"desktop\",\n  \"actions\": [\"refresh_state\", \"publish_changes\"],\n  \"screens\": [{\"id\": \"overview\", \"title\": \"Deployments\", \"node\": {\"id\": \"screen.overview\", \"type\": \"NavigationStack\", \"role\": \"status-overview\"}}]\n}\n",
        )
        .unwrap();

        let error = command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert_eq!(error, "mobile surface IR target invalid");

        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn compile_app_rejects_product_action_command_drift() {
        let app = test_root("compile-app-action-command-drift");
        let out = test_root("compile-app-action-command-drift-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        let product = sample_product_with_action_commands().replace(
            "[\"custom-core\", \"action\", \"publish_changes\", \"<json>\"]",
            "[\"custom-core\", \"other-action\", \"publish_changes\", \"<json>\"]",
        );
        write_or_replace(&app.join("app-blueprint/product.ir.json"), &product).unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();

        let error = command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert!(error.contains(
            "product IR action.command for publish_changes must start with runtime manifest actionCommand"
        ));

        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn compile_app_rejects_product_state_command_drift() {
        let app = test_root("compile-app-state-command-drift");
        let out = test_root("compile-app-state-command-drift-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        let product = sample_product().replace(
            "\"snapshotSchema\": \"deployments-state/v1\"",
            "\"snapshotSchema\": \"deployments-state/v1\", \"command\": [\"other-core\", \"state\"]",
        );
        write_or_replace(&app.join("app-blueprint/product.ir.json"), &product).unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();

        let error = command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "product IR state.command must match runtime manifest stateCommand"
        );

        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn compile_app_rejects_product_status_command_without_runtime_status() {
        let app = test_root("compile-app-status-command-drift");
        let out = test_root("compile-app-status-command-drift-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        let product = sample_product().replace(
            "\"snapshotSchema\": \"deployments-state/v1\"",
            "\"snapshotSchema\": \"deployments-state/v1\", \"statusCommand\": [\"custom-core\", \"status\"]",
        );
        write_or_replace(&app.join("app-blueprint/product.ir.json"), &product).unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest().replace(
                "\"statusCommand\": [\"custom-core\", \"status\"],\n    ",
                "",
            ),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();

        let error = command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "product IR state.statusCommand requires runtime manifest statusCommand"
        );

        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn compile_app_rejects_background_job_command_drift() {
        let app = test_root("compile-app-background-command-drift");
        let out = test_root("compile-app-background-command-drift-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        let product = sample_product().replace(
            "\"command\": [\"deployments-daemon\"]",
            "\"command\": [\"other-daemon\"]",
        );
        write_or_replace(&app.join("app-blueprint/product.ir.json"), &product).unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_full_runtime_manifest(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();

        let error = command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "product IR backgroundJobs.server-queue.command must match runtime manifest daemonCommand"
        );

        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn compile_app_rejects_background_job_without_runtime_daemon() {
        let app = test_root("compile-app-background-command-missing");
        let out = test_root("compile-app-background-command-missing-out");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/product.ir.json"),
            &sample_product(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest()
                .replace("    \"daemonCommand\": [\"deployments-daemon\"],\n", ""),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();

        let error = command_compile_app(&[
            app.display().to_string(),
            "--target".to_string(),
            "linux".to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "product IR backgroundJobs.server-queue.command requires runtime manifest daemonCommand"
        );

        fs::remove_dir_all(app).unwrap();
    }

    #[test]
    fn compile_macos_emits_full_runtime_bridge() {
        let root = test_root("compile-macos-bridge");
        let product = sample_product();
        let summary = validate_product_ir(&product).unwrap();
        let surface = project_surface(&product, "macos").unwrap();
        let runtime = sample_full_runtime_contract();
        compile_native_with_contract(&summary, &surface, &runtime, "macos", &root, false).unwrap();

        let swift = fs::read_to_string(root.join("Sources/App/App.swift")).unwrap();
        assert!(
            swift.contains("let runtimeStateCommand = [\"deployments-core\", \"runtime-state\"]")
        );
        assert!(
            swift.contains("let runtimeStatusCommand = [\"deployments-core\", \"runtime-status\"]")
        );
        assert!(swift.contains(
            "let runtimeSubscribeStatusCommand = [\"deployments-core\", \"runtime-status\"]"
        ));
        assert!(swift.contains(
            "let runtimeOperationStatusCommand = [\"deployments-core\", \"runtime-operation-status\"]"
        ));
        assert!(
            swift.contains("let runtimeActionCommand = [\"deployments-core\", \"runtime-action\"]")
        );
        assert!(swift
            .contains("let runtimeHistoryCommand = [\"deployments-core\", \"runtime-history\"]"));
        assert!(swift.contains("let runtimeDaemonCommand = [\"deployments-daemon\"]"));
        assert!(swift.contains("runtimeActionCommand + [defaultActionId, \"{}\"]"));
        assert!(swift.contains("struct ProductActionContract"));
        assert!(swift.contains("let actionContracts = [ProductActionContract"));
        assert!(swift
            .contains("func command(for action: ProductActionContract, json: String) -> [String]"));
        assert!(swift.contains("runtimeActionCommand + [action.id, json]"));
        assert!(swift.contains("ForEach(actionContracts, id: \\.id)"));
        assert!(swift.contains("runRuntimeCommand(runtimeSubscribeStatusCommand)"));
        assert!(swift.contains("inputShape: [\"deployment\": \"string\"]"));
        assert!(swift.contains("outputShape: [\"params\": \"object\"]"));
        assert!(swift.contains("Surface actions:"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn compile_native_mobile_outputs_runtime_contract() {
        let ios_root = test_root("compile-ios");
        let android_root = test_root("compile-android");
        let product = sample_mobile_product();
        let summary = validate_product_ir(&product).unwrap();
        let ios_surface = surface_with_actions(
            &project_surface(&product, "ios").unwrap(),
            &["publish_changes"],
        );
        let android_surface = surface_with_actions(
            &project_surface(&product, "android").unwrap(),
            &["publish_changes"],
        );
        let runtime = sample_full_runtime_contract();
        compile_native_with_contract(&summary, &ios_surface, &runtime, "ios", &ios_root, false)
            .unwrap();
        compile_native_with_contract(
            &summary,
            &android_surface,
            &runtime,
            "android",
            &android_root,
            false,
        )
        .unwrap();

        let ios = fs::read_to_string(ios_root.join("Host/App.swift")).unwrap();
        assert!(ios.contains("theurgy-runtime.json"));
        assert!(ios.contains("theurgy-runtime-action/v1"));
        assert!(ios.contains("Bundle.module.url(forResource: name, withExtension: \"json\")"));
        assert!(ios.contains("let runtimeMetadata = loadBundledContract(\"theurgy-runtime\")"));
        assert!(ios.contains("let surfaceMetadata = loadBundledContract(\"theurgy-surface\")"));
        assert!(ios.contains("JSONSerialization.jsonObject(with: data)"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"app\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"target\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"adapterRuntimeTransport\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"runtimeStatusSchema\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"runtimeActionRequestSchema\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"runtimeActionResultSchema\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"operationStatusSchema\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"operationHistorySchema\")"));
        assert!(ios.contains("runtimeStringArray(runtimeMetadata, key: \"surfaceActions\")"));
        assert!(ios.contains("func surfaceScreens(_ json: String) -> [String]"));
        assert!(ios.contains("contractObject(json)?[\"screens\"] as? [[String: Any]]"));
        assert!(ios.contains(
            "var surfaceSchema: String { runtimeString(surfaceMetadata, key: \"version\") }"
        ));
        assert!(ios.contains(
            "var surfaceTarget: String { runtimeString(surfaceMetadata, key: \"target\") }"
        ));
        assert!(ios.contains("var surfaceScreens: [String] { surfaceScreens(surfaceMetadata) }"));
        assert!(ios.contains("Runtime app: \\(contract.runtimeApp)"));
        assert!(ios.contains("Runtime target: \\(contract.runtimeTarget)"));
        assert!(ios.contains("Runtime transport: \\(contract.runtimeTransport)"));
        assert!(ios.contains("Runtime status schema: \\(contract.runtimeStatusSchema)"));
        assert!(
            ios.contains("Runtime action request schema: \\(contract.runtimeActionRequestSchema)")
        );
        assert!(
            ios.contains("Runtime action result schema: \\(contract.runtimeActionResultSchema)")
        );
        assert!(ios.contains("Operation status schema: \\(contract.operationStatusSchema)"));
        assert!(ios.contains("Operation history schema: \\(contract.operationHistorySchema)"));
        assert!(ios.contains("Runtime surface actions: \\(contract.runtimeSurfaceActions.joined"));
        assert!(ios.contains("Surface schema: \\(contract.surfaceSchema)"));
        assert!(ios.contains("Surface target: \\(contract.surfaceTarget)"));
        assert!(ios.contains("Surface screens: \\(contract.surfaceScreens.joined"));
        assert!(ios.contains("\"deployments-core\", \"runtime-state\""));
        assert!(ios.contains("\"deployments-core\", \"runtime-status\""));
        assert!(
            ios.contains("let subscribeStatusCommand = [\"deployments-core\", \"runtime-status\"]")
        );
        assert!(ios.contains(
            "let operationStatusCommand = [\"deployments-core\", \"runtime-operation-status\"]"
        ));
        assert!(ios.contains("\"deployments-core\", \"runtime-history\""));
        assert!(ios.contains("\"deployments-daemon\""));
        assert!(ios.contains("struct ProductActionContract"));
        assert!(ios.contains("let actionContracts = [ProductActionContract"));
        assert!(ios
            .contains("func command(for action: ProductActionContract, json: String) -> [String]"));
        assert!(ios.contains("actionCommand + [action.id, json]"));
        assert!(ios.contains(
            "func actionEnvelope(for action: ProductActionContract, params: [String: Any]) -> String"
        ));
        assert!(ios.contains("\"protocol\": protocolName"));
        assert!(ios.contains("\"app\": runtimeApp"));
        assert!(ios.contains("\"action\": action.id"));
        assert!(ios.contains("\"params\": params"));
        assert!(ios.contains("JSONSerialization.data(withJSONObject: envelope"));
        assert!(ios.contains("contract.actionEnvelope(for: action, params: [:])"));
        assert!(ios.contains("id: \"publish_changes\""));
        assert!(ios.contains("inputKeys: [\"deployment\"]"));
        assert!(ios.contains("outputKeys: [\"params\"]"));
        assert!(ios.contains("failureKeys: []"));
        assert!(ios.contains("inputShape: [\"deployment\": \"string\"]"));
        assert!(ios.contains("outputShape: [\"params\": \"object\"]"));
        assert!(!ios.contains("id: \"refresh_state\""));
        let ios_package = fs::read_to_string(ios_root.join("Package.swift")).unwrap();
        assert!(ios_package.contains("platforms: [.iOS(.v16)]"));
        assert!(ios_package.contains(
            "executableTarget(name: \"Host\", path: \"Host\", resources: [.copy(\"Resources\")])"
        ));
        assert_eq!(
            fs::read_to_string(ios_root.join("Host/Resources/theurgy-runtime.json")).unwrap(),
            fs::read_to_string(ios_root.join("theurgy-runtime.json")).unwrap()
        );
        assert_eq!(
            fs::read_to_string(ios_root.join("Host/Resources/theurgy-surface.json")).unwrap(),
            fs::read_to_string(ios_root.join("theurgy-surface.json")).unwrap()
        );
        let ios_runtime: Value = serde_json::from_str(
            &fs::read_to_string(ios_root.join("theurgy-runtime.json")).unwrap(),
        )
        .unwrap();
        let ios_generated = validate_generated_runtime(
            &fs::read_to_string(ios_root.join("theurgy-runtime.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(ios_generated.adapter_runtime_transport, "external-json-abi");
        assert_eq!(
            ios_generated.runtime_status_schema,
            "theurgy-runtime-status/v1"
        );
        assert_eq!(
            ios_generated.runtime_action_request_schema,
            "theurgy-runtime-action-request/v1"
        );
        assert_eq!(
            ios_generated.runtime_action_result_schema,
            "theurgy-runtime-action-result/v1"
        );
        assert_eq!(
            ios_generated.operation_status_schema,
            "theurgy-operation-status/v1"
        );
        assert_eq!(
            ios_generated.operation_history_schema,
            "theurgy-operation-history/v1"
        );
        assert_eq!(ios_generated.surface_schema, "theurgy-mobile-surface-ir/v1");
        assert_eq!(ios_generated.surface_target, "ios");
        assert_eq!(
            ios_runtime
                .get("runtimeStatusSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-status/v1")
        );
        assert_eq!(
            ios_runtime
                .get("runtimeActionRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action-request/v1")
        );
        assert_eq!(
            ios_runtime
                .get("runtimeActionResultSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action-result/v1")
        );
        assert_eq!(
            ios_runtime
                .get("operationStatusSchema")
                .and_then(Value::as_str),
            Some("theurgy-operation-status/v1")
        );
        assert_eq!(
            ios_runtime
                .get("operationHistorySchema")
                .and_then(Value::as_str),
            Some("theurgy-operation-history/v1")
        );
        assert_eq!(
            ios_runtime
                .get("adapterRuntimeTransport")
                .and_then(Value::as_str),
            Some("external-json-abi")
        );

        let android = fs::read_to_string(
            android_root.join("app/src/main/java/app/theurgy/generated/MainActivity.java"),
        )
        .unwrap();
        assert!(android.contains("theurgy-runtime-action/v1"));
        assert!(android.contains("getAssets().open(name)"));
        assert!(android.contains("loadBundledContract(\"theurgy-runtime.json\")"));
        assert!(android.contains("loadBundledContract(\"theurgy-surface.json\")"));
        assert!(android.contains("new JSONObject(json)"));
        assert!(android.contains("jsonString(runtimeMetadata, \"app\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"target\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"adapterRuntimeTransport\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"runtimeStatusSchema\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"runtimeActionRequestSchema\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"runtimeActionResultSchema\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"operationStatusSchema\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"operationHistorySchema\")"));
        assert!(android.contains("jsonStringArray(runtimeMetadata, \"surfaceActions\")"));
        assert!(android.contains("private static String surfaceScreens(String json)"));
        assert!(android.contains("new JSONObject(json).optJSONArray(\"screens\")"));
        assert!(android.contains("screen.optString(\"title\", id)"));
        assert!(android.contains("Runtime app: "));
        assert!(android.contains("Runtime target: "));
        assert!(android.contains("Runtime transport: "));
        assert!(android.contains("Runtime status schema: "));
        assert!(android.contains("Runtime action request schema: "));
        assert!(android.contains("Runtime action result schema: "));
        assert!(android.contains("Operation status schema: "));
        assert!(android.contains("Operation history schema: "));
        assert!(android.contains("Runtime surface actions: "));
        assert!(android.contains("Surface schema: "));
        assert!(android.contains("Surface target: "));
        assert!(android.contains("Surface screens: "));
        assert!(android.contains("new String[] {\"deployments-core\", \"runtime-action\"}"));
        assert!(android.contains("new String[] {\"deployments-core\", \"runtime-status\"}"));
        assert!(android.contains(
            "private static final String[] SUBSCRIBE_STATUS_COMMAND = new String[] {\"deployments-core\", \"runtime-status\"};"
        ));
        assert!(android.contains(
            "private static final String[] OPERATION_STATUS_COMMAND = new String[] {\"deployments-core\", \"runtime-operation-status\"};"
        ));
        assert!(android.contains("new String[] {\"deployments-core\", \"runtime-history\"}"));
        assert!(android.contains("new String[] {\"deployments-daemon\"}"));
        assert!(android.contains("private static final ProductActionContract[] ACTION_CONTRACTS"));
        assert!(android.contains(
            "private static String[] commandFor(ProductActionContract action, String json)"
        ));
        assert!(android.contains("command[ACTION_COMMAND.length] = action.id;"));
        assert!(android.contains(
            "private static String actionEnvelope(String app, ProductActionContract action, JSONObject params)"
        ));
        assert!(android.contains("envelope.put(\"protocol\", PROTOCOL);"));
        assert!(android.contains("envelope.put(\"app\", app);"));
        assert!(android.contains("envelope.put(\"action\", action.id);"));
        assert!(android.contains("envelope.put(\"params\", params);"));
        assert!(android.contains("String runtimeApp = jsonString(runtimeMetadata, \"app\");"));
        assert!(android.contains("actionEnvelope(runtimeApp, action, new JSONObject())"));
        assert!(android.contains(
            "new ProductActionContract(\"publish_changes\", \"Push to Production\", \"release\""
        ));
        assert!(android.contains("new String[] {\"deployment\"}"));
        assert!(android.contains("new String[] {\"params\"}"));
        assert!(android.contains("new String[][] {{\"deployment\", \"string\"}}"));
        assert!(android.contains("new String[][] {{\"params\", \"object\"}}"));
        assert!(android.contains("final String[] outputKeys;"));
        assert!(android.contains("final String[] failureKeys;"));
        assert!(android.contains("final String[][] outputShape;"));
        assert!(!android.contains("new ProductActionContract(\"refresh_state\""));
        let android_settings = fs::read_to_string(android_root.join("settings.gradle")).unwrap();
        assert!(android_settings.contains("rootProject.name = 'deployments-theurgy'"));
        let android_root_gradle = fs::read_to_string(android_root.join("build.gradle")).unwrap();
        assert!(android_root_gradle.contains("id 'com.android.application' version"));
        let android_app_gradle = fs::read_to_string(android_root.join("app/build.gradle")).unwrap();
        assert!(android_app_gradle.contains("namespace 'app.theurgy.generated'"));
        assert!(android_app_gradle.contains("applicationId 'app.theurgy.deployments'"));
        let android_manifest =
            fs::read_to_string(android_root.join("app/src/main/AndroidManifest.xml")).unwrap();
        assert!(android_manifest.contains("android.intent.action.MAIN"));
        assert!(android_manifest.contains("android:label=\"Deployments\""));
        let android_styles =
            fs::read_to_string(android_root.join("app/src/main/res/values/styles.xml")).unwrap();
        assert!(android_styles.contains("Theme.Material.Light.NoActionBar"));
        assert_eq!(
            fs::read_to_string(android_root.join("app/src/main/assets/theurgy-runtime.json"))
                .unwrap(),
            fs::read_to_string(android_root.join("theurgy-runtime.json")).unwrap()
        );
        assert_eq!(
            fs::read_to_string(android_root.join("app/src/main/assets/theurgy-surface.json"))
                .unwrap(),
            fs::read_to_string(android_root.join("theurgy-surface.json")).unwrap()
        );
        let android_runtime: Value = serde_json::from_str(
            &fs::read_to_string(android_root.join("theurgy-runtime.json")).unwrap(),
        )
        .unwrap();
        let android_generated = validate_generated_runtime(
            &fs::read_to_string(android_root.join("theurgy-runtime.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            android_generated.adapter_runtime_transport,
            "external-json-abi"
        );
        assert_eq!(
            android_generated.runtime_status_schema,
            "theurgy-runtime-status/v1"
        );
        assert_eq!(
            android_generated.runtime_action_request_schema,
            "theurgy-runtime-action-request/v1"
        );
        assert_eq!(
            android_generated.runtime_action_result_schema,
            "theurgy-runtime-action-result/v1"
        );
        assert_eq!(
            android_generated.operation_status_schema,
            "theurgy-operation-status/v1"
        );
        assert_eq!(
            android_generated.operation_history_schema,
            "theurgy-operation-history/v1"
        );
        assert_eq!(
            android_generated.surface_schema,
            "theurgy-mobile-surface-ir/v1"
        );
        assert_eq!(android_generated.surface_target, "android");
        assert_eq!(
            android_runtime
                .get("runtimeStatusSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-status/v1")
        );
        assert_eq!(
            android_runtime
                .get("runtimeActionRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action-request/v1")
        );
        assert_eq!(
            android_runtime
                .get("runtimeActionResultSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action-result/v1")
        );
        assert_eq!(
            android_runtime
                .get("operationStatusSchema")
                .and_then(Value::as_str),
            Some("theurgy-operation-status/v1")
        );
        assert_eq!(
            android_runtime
                .get("operationHistorySchema")
                .and_then(Value::as_str),
            Some("theurgy-operation-history/v1")
        );
        assert_eq!(
            android_runtime
                .get("adapterRuntimeTransport")
                .and_then(Value::as_str),
            Some("external-json-abi")
        );

        fs::remove_dir_all(ios_root).unwrap();
        fs::remove_dir_all(android_root).unwrap();
    }

    fn surface_with_actions(surface: &str, actions: &[&str]) -> String {
        let mut value: Value = serde_json::from_str(surface).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("actions".to_string(), serde_json::json!(actions));
        format!("{}\n", serde_json::to_string_pretty(&value).unwrap())
    }

    fn sample_product() -> String {
        "{\n  \"version\": \"theurgy-product-ir/v1\",\n  \"format\": \"json\",\n  \"app\": {\n    \"id\": \"deployments\",\n    \"name\": \"Deployments\",\n    \"targets\": [\"macos\", \"linux\"],\n    \"capabilities\": [\"native-desktop\", \"runtime-actions\"],\n    \"permissions\": [\"files\"]\n  },\n  \"domain\": {\n    \"objects\": [\n      {\"id\": \"server\", \"label\": \"Server\"},\n      {\"id\": \"deployment\", \"label\": \"Deployment\"}\n    ]\n  },\n  \"actions\": [\n    {\"id\": \"refresh_state\", \"label\": \"Refresh\", \"input\": {}, \"output\": {\"params\": \"object\"}, \"effect\": \"read\", \"failure\": {}, \"safe\": true, \"mutating\": false, \"longRunning\": false, \"privileged\": false},\n    {\"id\": \"publish_changes\", \"label\": \"Push to Production\", \"input\": {\"deployment\": \"string\"}, \"output\": {\"params\": \"object\"}, \"effect\": \"release\", \"failure\": {}, \"safe\": false, \"mutating\": true, \"longRunning\": true, \"privileged\": true}\n  ],\n  \"state\": {\n    \"snapshotSchema\": \"deployments-state/v1\",\n    \"roots\": [{\"id\": \"headquarters-workspace\", \"kind\": \"xdg-state\"}]\n  },\n  \"backgroundJobs\": [\n    {\"id\": \"server-queue\", \"label\": \"Server Queue\", \"command\": [\"deployments-daemon\"], \"state\": \"server.queue_mode\"}\n  ],\n  \"releaseTargets\": [\n    {\"id\": \"macos-app\", \"target\": \"macos\", \"surface\": \"desktop\", \"artifact\": \"generated/macos\"},\n    {\"id\": \"linux-app\", \"target\": \"linux\", \"surface\": \"desktop\", \"artifact\": \"generated/linux\"}\n  ],\n  \"persistence\": {\n    \"truth\": \"file-first\"\n  },\n  \"audit\": {\n    \"operationHistory\": true,\n    \"cliParity\": true\n  }\n}".to_string()
    }

    fn sample_mobile_product() -> String {
        sample_product()
            .replace(
                "\"targets\": [\"macos\", \"linux\"]",
                "\"targets\": [\"ios\", \"android\"]",
            )
            .replace(
                "{\"id\": \"macos-app\", \"target\": \"macos\", \"surface\": \"desktop\", \"artifact\": \"generated/macos\"},\n    {\"id\": \"linux-app\", \"target\": \"linux\", \"surface\": \"desktop\", \"artifact\": \"generated/linux\"}",
                "{\"id\": \"ios-app\", \"target\": \"ios\", \"surface\": \"mobile\", \"artifact\": \"generated/mobile/ios\"},\n    {\"id\": \"android-app\", \"target\": \"android\", \"surface\": \"mobile\", \"artifact\": \"generated/mobile/android\"}",
            )
    }

    fn sample_product_with_action_commands() -> String {
        sample_product()
            .replace(
                "\"id\": \"refresh_state\", \"label\": \"Refresh\", \"input\": {}, \"output\": {\"params\": \"object\"}, \"effect\": \"read\", \"failure\": {}, \"safe\": true, \"mutating\": false, \"longRunning\": false, \"privileged\": false",
                "\"id\": \"refresh_state\", \"label\": \"Refresh\", \"input\": {}, \"output\": {\"params\": \"object\"}, \"effect\": \"read\", \"failure\": {}, \"safe\": true, \"mutating\": false, \"longRunning\": false, \"privileged\": false, \"command\": [\"custom-core\", \"action\", \"refresh_state\", \"{}\"]",
            )
            .replace(
                "\"id\": \"publish_changes\", \"label\": \"Push to Production\", \"input\": {\"deployment\": \"string\"}, \"output\": {\"params\": \"object\"}, \"effect\": \"release\", \"failure\": {}, \"safe\": false, \"mutating\": true, \"longRunning\": true, \"privileged\": true",
                "\"id\": \"publish_changes\", \"label\": \"Push to Production\", \"input\": {\"deployment\": \"string\"}, \"output\": {\"params\": \"object\"}, \"effect\": \"release\", \"failure\": {}, \"safe\": false, \"mutating\": true, \"longRunning\": true, \"privileged\": true, \"command\": [\"custom-core\", \"action\", \"publish_changes\", \"<json>\"]",
            )
    }

    fn sample_product_with_surfaces(desktop_surface_ir: &str) -> String {
        sample_product().replace(
            "\"audit\": {\n    \"operationHistory\": true,\n    \"cliParity\": true\n  }",
            &format!(
                "\"audit\": {{\n    \"operationHistory\": true,\n    \"cliParity\": true\n  }},\n  \"surfaces\": {{\n    \"desktop\": \"{}\",\n    \"mobile\": \"app-blueprint/mobile.surface.ir.json\"\n  }}",
                json_escape(desktop_surface_ir)
            ),
        )
    }

    fn sample_product_with_state_commands(desktop_surface_ir: &str) -> String {
        sample_product_with_surfaces(desktop_surface_ir).replace(
            "\"snapshotSchema\": \"deployments-state/v1\",",
            "\"snapshotSchema\": \"deployments-state/v1\",\n    \"command\": [\"custom-core\", \"state\"],\n    \"statusCommand\": [\"custom-core\", \"status\"],",
        )
    }

    fn sample_full_runtime_contract() -> RuntimeContract {
        RuntimeContract {
            app_id: "deployments".to_string(),
            protocol: "theurgy-runtime-action/v1".to_string(),
            product_ir: "app-blueprint/product.ir.json".to_string(),
            runtime_manifest: "app-blueprint/runtime.manifest.json".to_string(),
            source_surface_ir: "app-blueprint/desktop.surface.ir.json".to_string(),
            legacy_native_desktop_ir: Some("app-blueprint/app.ir.yaml".to_string()),
            compatibility: RuntimeCompatibility::shell_first_default(),
            state_command: vec!["deployments-core".to_string(), "runtime-state".to_string()],
            status_command: vec!["deployments-core".to_string(), "runtime-status".to_string()],
            subscribe_status_command: vec![
                "deployments-core".to_string(),
                "runtime-status".to_string(),
            ],
            operation_status_command: vec![
                "deployments-core".to_string(),
                "runtime-operation-status".to_string(),
            ],
            action_command: vec!["deployments-core".to_string(), "runtime-action".to_string()],
            history_command: vec![
                "deployments-core".to_string(),
                "runtime-history".to_string(),
            ],
            daemon_command: vec!["deployments-daemon".to_string()],
            product_action_ids: Some(vec![
                "refresh_state".to_string(),
                "publish_changes".to_string(),
            ]),
            product_action_contracts: Some(
                validate_product_ir(&sample_product())
                    .unwrap()
                    .action_contracts,
            ),
        }
    }

    fn sample_action_ir() -> String {
        "{\n  \"version\": \"theurgy-action-ir/v1\",\n  \"actions\": [\n    {\"id\": \"refresh_state\", \"label\": \"Refresh\", \"input\": {}, \"output\": {}, \"effect\": \"read\", \"failure\": {}, \"safe\": true, \"mutating\": false, \"longRunning\": false, \"privileged\": false},\n    {\"id\": \"publish_changes\", \"label\": \"Push to Production\", \"input\": {}, \"output\": {}, \"effect\": \"release\", \"failure\": {}, \"safe\": false, \"mutating\": true, \"longRunning\": true, \"privileged\": true}\n  ]\n}".to_string()
    }

    fn sample_state_snapshot() -> String {
        "{\n  \"schema\": \"theurgy-state-snapshot/v1\",\n  \"app\": \"deployments\",\n  \"generatedAt\": \"2026-06-11T00:00:00Z\",\n  \"data\": {\n    \"servers\": [],\n    \"deployments\": []\n  }\n}".to_string()
    }

    fn sample_runtime_status() -> String {
        "{\n  \"schema\": \"theurgy-runtime-status/v1\",\n  \"app\": \"deployments\",\n  \"generatedAt\": \"2026-06-11T00:00:00Z\",\n  \"state_ready\": true,\n  \"servers\": 0,\n  \"deployments\": 0\n}".to_string()
    }

    fn sample_runtime_action_result() -> String {
        "{\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"publish_changes\",\n  \"operation\": {\n    \"id\": \"deployments-publish_changes-123\",\n    \"status\": \"completed\",\n    \"progress\": 100,\n    \"longRunning\": true\n  },\n  \"result\": {\"message\": \"published\"}\n}".to_string()
    }

    fn sample_operation_status() -> String {
        "{\n  \"schema\": \"theurgy-operation-status/v1\",\n  \"app\": \"deployments\",\n  \"generatedAt\": \"2026-06-11T00:00:00Z\",\n  \"operation\": {\n    \"id\": \"deployments-publish_changes-123\",\n    \"status\": \"completed\",\n    \"progress\": 100,\n    \"longRunning\": true\n  }\n}".to_string()
    }

    fn sample_operation_history() -> String {
        "{\n  \"schema\": \"theurgy-operation-history/v1\",\n  \"app\": \"deployments\",\n  \"generatedAt\": \"2026-06-11T00:00:00Z\",\n  \"data\": [\n    {\"action\": \"publish\", \"status\": \"completed\"}\n  ]\n}".to_string()
    }

    fn runtime_fixture_root(label: &str) -> PathBuf {
        let root = test_root(label);
        fs::create_dir_all(&root).unwrap();
        let blueprint = root.join("app-blueprint");
        fs::create_dir_all(&blueprint).unwrap();
        let runtime = root.join("runtime-fixture");
        write_executable(
            &runtime,
            "#!/bin/sh\nset -eu\ncase \"${1-}\" in\n  state) printf '{\"schema\":\"theurgy-state-snapshot/v1\",\"app\":\"deployments\",\"generatedAt\":\"2026-06-11T00:00:00Z\",\"data\":{}}\\n' ;;\n  status) printf '{\"schema\":\"theurgy-runtime-status/v1\",\"app\":\"deployments\",\"generatedAt\":\"2026-06-11T00:00:00Z\",\"state_ready\":true}\\n' ;;\n  subscribe) printf '{\"schema\":\"theurgy-runtime-status/v1\",\"app\":\"deployments\",\"generatedAt\":\"2026-06-11T00:00:00Z\",\"state_ready\":false}\\n' ;;\n  operation-status) printf '{\"schema\":\"theurgy-operation-status/v1\",\"app\":\"deployments\",\"generatedAt\":\"2026-06-11T00:00:00Z\",\"operation\":{\"id\":\"%s\",\"status\":\"completed\",\"progress\":100,\"longRunning\":true}}\\n' \"${2-}\" ;;\n  history) printf '{\"schema\":\"theurgy-operation-history/v1\",\"app\":\"deployments\",\"generatedAt\":\"2026-06-11T00:00:00Z\",\"data\":[],\"subject\":\"%s\",\"limit\":\"%s\"}\\n' \"${2-}\" \"${3-}\" ;;\n  action) printf '{\"success\":true,\"data\":{\"protocol\":\"theurgy-runtime-action/v1\",\"app\":\"deployments\",\"action\":\"%s\",\"operation\":{\"id\":\"op-%s\",\"status\":\"completed\",\"progress\":100,\"longRunning\":false},\"result\":{\"params\":%s}}}\\n' \"${2-}\" \"${2-}\" \"${3-}\" ;;\n  *) printf 'unknown fixture command\\n' >&2; exit 2 ;;\nesac\n",
        )
        .unwrap();
        write_or_replace(&blueprint.join("product.ir.json"), &sample_product()).unwrap();
        write_or_replace(
            &root.join("runtime.manifest.json"),
            &format!(
                "{{\n  \"version\": \"theurgy-runtime-manifest/v1\",\n  \"app\": \"deployments\",\n  \"productIr\": \"app-blueprint/product.ir.json\",\n  \"runtime\": {{\n    \"stateCommand\": [\"{}\", \"state\"],\n    \"statusCommand\": [\"{}\", \"status\"],\n    \"operationStatusCommand\": [\"{}\", \"operation-status\"],\n    \"actionCommand\": [\"{}\", \"action\"],\n    \"historyCommand\": [\"{}\", \"history\"],\n    \"protocol\": \"deployments-runtime/v1\"\n  }}\n}}",
                json_escape(&runtime.display().to_string()),
                json_escape(&runtime.display().to_string()),
                json_escape(&runtime.display().to_string()),
                json_escape(&runtime.display().to_string()),
                json_escape(&runtime.display().to_string())
            ),
        )
        .unwrap();
        root
    }

    fn sample_runtime_manifest() -> String {
        "{\n  \"version\": \"theurgy-runtime-manifest/v1\",\n  \"app\": \"deployments\",\n  \"productIr\": \"app-blueprint/product.ir.json\",\n  \"runtime\": {\n    \"stateCommand\": [\"custom-core\", \"state\"],\n    \"statusCommand\": [\"custom-core\", \"status\"],\n    \"operationStatusCommand\": [\"custom-core\", \"operation-status\"],\n    \"actionCommand\": [\"custom-core\", \"action\"],\n    \"historyCommand\": [\"custom-core\", \"history\"],\n    \"daemonCommand\": [\"deployments-daemon\"],\n    \"protocol\": \"deployments-runtime/v1\"\n  },\n  \"surfaces\": {\n    \"desktop\": \"app-blueprint/desktop.surface.ir.json\",\n    \"legacyNativeDesktop\": \"app-blueprint/app.ir.yaml\"\n  },\n  \"compatibility\": {\n    \"wizardryAppsShellFirstStillSupported\": true,\n    \"theurgyRequiredForLegacyWizardryApps\": false\n  }\n}".to_string()
    }

    fn sample_full_runtime_manifest() -> String {
        sample_runtime_manifest()
    }

    fn sample_desktop_surface() -> String {
        "{\n  \"version\": \"theurgy-desktop-surface-ir/v1\",\n  \"format\": \"json\",\n  \"product\": \"deployments\",\n  \"target\": \"desktop\",\n  \"actions\": [\"refresh_state\", \"publish_changes\"],\n  \"window\": {\n    \"id\": \"window.main\",\n    \"type\": \"Window\",\n    \"title\": \"Deployments\",\n    \"role\": \"declared-reference-surface\"\n  }\n}".to_string()
    }
}
