use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};
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
        Some("validate-runtime-state-request") => {
            command_validate_runtime_state_request(&args[2..])
        }
        Some("validate-runtime-status-request") => {
            command_validate_runtime_status_request(&args[2..])
        }
        Some("validate-runtime-subscribe-status-request") => {
            command_validate_runtime_subscribe_status_request(&args[2..])
        }
        Some("validate-runtime-action-request") => {
            command_validate_runtime_action_request(&args[2..])
        }
        Some("validate-runtime-action-result") => {
            command_validate_runtime_action_result(&args[2..])
        }
        Some("validate-operation-status-request") => {
            command_validate_operation_status_request(&args[2..])
        }
        Some("validate-operation-history-request") => {
            command_validate_operation_history_request(&args[2..])
        }
        Some("validate-operation-status") => command_validate_operation_status(&args[2..]),
        Some("validate-operation-history") => command_validate_operation_history(&args[2..]),
        Some("validate-runtime-manifest") => command_validate_runtime_manifest(&args[2..]),
        Some("validate-generated-runtime") => command_validate_generated_runtime(&args[2..]),
        Some("validate-surface-ir") => command_validate_surface_ir(&args[2..]),
        Some("project-surface") => command_project_surface(&args[2..]),
        Some("compile-native") => command_compile_native(&args[2..]),
        Some("compile-app") => command_compile_app(&args[2..]),
        Some("stage-app-runtime") => command_stage_app_runtime(&args[2..]),
        Some("inspect-app") => command_inspect_app(&args[2..]),
        Some("run-state") => command_run_state(&args[2..]),
        Some("run-status") => command_run_status(&args[2..]),
        Some("subscribe-status") => command_subscribe_status(&args[2..]),
        Some("run-request") => command_run_request(&args[2..]),
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

fn command_validate_operation_status_request(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-operation-status-request PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_operation_status_request(&value)?;
    println!("status=ok");
    println!(
        "schema={}",
        product_runtime::OPERATION_STATUS_REQUEST_SCHEMA
    );
    println!("protocol={}", product_runtime::RUNTIME_ACTION_PROTOCOL);
    println!("app={}", summary.app_id);
    println!("operation={}", summary.operation_id);
    Ok(())
}

fn command_validate_runtime_state_request(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-runtime-state-request PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_runtime_state_request(&value)?;
    println!("status=ok");
    println!("schema={}", product_runtime::RUNTIME_STATE_REQUEST_SCHEMA);
    println!("protocol={}", product_runtime::RUNTIME_ACTION_PROTOCOL);
    println!("app={}", summary.app_id);
    Ok(())
}

fn command_validate_runtime_status_request(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-runtime-status-request PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_runtime_status_request(&value)?;
    println!("status=ok");
    println!("schema={}", product_runtime::RUNTIME_STATUS_REQUEST_SCHEMA);
    println!("protocol={}", product_runtime::RUNTIME_ACTION_PROTOCOL);
    println!("app={}", summary.app_id);
    Ok(())
}

fn command_validate_runtime_subscribe_status_request(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(
            TheurgyError::new("usage: validate-runtime-subscribe-status-request PATH").into(),
        );
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_runtime_subscribe_status_request(&value)?;
    println!("status=ok");
    println!(
        "schema={}",
        product_runtime::RUNTIME_SUBSCRIBE_STATUS_REQUEST_SCHEMA
    );
    println!("protocol={}", product_runtime::RUNTIME_ACTION_PROTOCOL);
    println!("app={}", summary.app_id);
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

fn command_validate_operation_history_request(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-operation-history-request PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_operation_history_request(&value)?;
    println!("status=ok");
    println!(
        "schema={}",
        product_runtime::OPERATION_HISTORY_REQUEST_SCHEMA
    );
    println!("protocol={}", product_runtime::RUNTIME_ACTION_PROTOCOL);
    println!("app={}", summary.app_id);
    println!("subject={}", summary.subject);
    println!("limit={}", summary.limit);
    Ok(())
}

fn command_validate_runtime_manifest(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-runtime-manifest PATH").into());
    }
    let summary = product_runtime::load_runtime_manifest(&args[0])?;
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
    if let Some(command) = &summary.request_command {
        println!("request_command={}", command.join(" "));
    }
    if let Some(manifest) = &summary.request_command_manifest {
        println!("request_command_manifest={manifest}");
    }
    println!(
        "runtime_state_request_schema={}",
        summary.runtime_state_request_schema
    );
    println!(
        "runtime_status_request_schema={}",
        summary.runtime_status_request_schema
    );
    println!(
        "runtime_subscribe_status_request_schema={}",
        summary.runtime_subscribe_status_request_schema
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
        "operation_status_request_schema={}",
        summary.operation_status_request_schema
    );
    println!(
        "operation_history_request_schema={}",
        summary.operation_history_request_schema
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
    println!("surface_screens={}", summary.surface_screens);
    println!(
        "surface_screen_contracts={}",
        summary.surface_screen_contracts
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
    let contract = product_runtime::load_app_compile_contract(app_dir, target)?;
    let product_text = read_json(&app_dir.join(&contract.product_ir_path))?;
    let runtime_manifest_text = read_json(&app_dir.join(&contract.runtime_manifest_path))?;
    let surface_text = read_json(&app_dir.join(&contract.surface_ir_path))?;
    compile_native_with_contract(
        &contract.product,
        &contract.surface_text,
        &contract.runtime,
        target,
        out_dir,
        contract.preserve_existing_legacy_desktop_adapter,
        &[
            (&contract.product_ir_path, product_text.as_str()),
            (
                &contract.runtime_manifest_path,
                runtime_manifest_text.as_str(),
            ),
            (&contract.surface_ir_path, surface_text.as_str()),
        ],
    )?;
    println!("status=ok");
    println!("app={}", app_dir.display());
    println!("target={target}");
    println!("out={}", out_dir.display());
    Ok(())
}

fn command_stage_app_runtime(args: &[String]) -> Result<()> {
    let (app_dir, target, out_dir) = parse_compile_args(args)?;
    if !product_runtime::is_desktop_target(target) {
        return Err(TheurgyError::new("stage-app-runtime only supports macos and linux").into());
    }
    let contract = product_runtime::load_app_compile_contract(app_dir, target)?;
    stage_app_runtime_binaries(app_dir, out_dir, target, &contract.runtime)?;
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
    for line in product_runtime::load_app_inspection_lines(&path)? {
        println!("{line}");
    }
    Ok(())
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

fn command_run_request(args: &[String]) -> Result<()> {
    let (request_path, manifest_path) = parse_request_manifest_args(args)?;
    let request = read_json(&request_path)?;
    let output = run_request_output(&request, &manifest_path)?;
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

fn parse_request_manifest_args(args: &[String]) -> Result<(PathBuf, PathBuf)> {
    if args.is_empty() {
        return Err(TheurgyError::new("usage: run-request REQUEST_JSON --manifest PATH").into());
    }
    let request_path = PathBuf::from(&args[0]);
    let mut manifest_path: Option<PathBuf> = None;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--manifest" => {
                let raw = args
                    .get(index + 1)
                    .ok_or_else(|| TheurgyError::new("run-request --manifest requires PATH"))?;
                manifest_path = Some(PathBuf::from(raw));
                index += 2;
            }
            other => {
                return Err(
                    TheurgyError::new(format!("unknown run-request option: {other}")).into(),
                )
            }
        }
    }
    let manifest_path =
        manifest_path.ok_or_else(|| TheurgyError::new("run-request --manifest PATH required"))?;
    Ok((request_path, manifest_path))
}

fn run_request_output(request_text: &str, manifest_path: &Path) -> Result<String> {
    let value = parse_json(request_text)?;
    let schema = value
        .get("schema")
        .and_then(Value::as_str)
        .ok_or_else(|| TheurgyError::new("runtime request schema required"))?;
    match schema {
        product_runtime::RUNTIME_STATE_REQUEST_SCHEMA => {
            let summary = validate_runtime_state_request_value(&value)?;
            let runtime = runtime_contract_from_path(manifest_path)?;
            validate_runtime_request_app(&summary.app_id, &runtime.app_id, "state")?;
            run_state_output_with_runtime(&runtime, manifest_path.parent())
        }
        product_runtime::RUNTIME_STATUS_REQUEST_SCHEMA => {
            let summary = validate_runtime_status_request_value(&value)?;
            let runtime = runtime_contract_from_path(manifest_path)?;
            validate_runtime_request_app(&summary.app_id, &runtime.app_id, "status")?;
            run_status_output_with_runtime(&runtime, manifest_path.parent())
        }
        product_runtime::RUNTIME_SUBSCRIBE_STATUS_REQUEST_SCHEMA => {
            let summary = validate_runtime_subscribe_status_request_value(&value)?;
            let runtime = runtime_contract_from_path(manifest_path)?;
            validate_runtime_request_app(&summary.app_id, &runtime.app_id, "subscribe status")?;
            subscribe_status_output_with_runtime(&runtime, manifest_path.parent())
        }
        product_runtime::RUNTIME_ACTION_REQUEST_SCHEMA => {
            let summary = validate_runtime_action_request_value(&value)?;
            let runtime = runtime_contract_from_path_with_product_actions(manifest_path)?;
            validate_runtime_action_request_against_runtime(&summary, &value, &runtime)?;
            let params = value
                .get("params")
                .ok_or_else(|| TheurgyError::new("runtime action request params required"))?
                .to_string();
            run_manifest_action(
                &runtime,
                &summary.action_id,
                &params,
                manifest_path.parent(),
            )
        }
        product_runtime::OPERATION_STATUS_REQUEST_SCHEMA => {
            let summary = validate_operation_status_request_value(&value)?;
            let runtime = runtime_contract_from_path(manifest_path)?;
            validate_runtime_request_app(&summary.app_id, &runtime.app_id, "operation status")?;
            run_operation_status_output_with_runtime(
                &runtime,
                &summary.operation_id,
                manifest_path.parent(),
            )
        }
        product_runtime::OPERATION_HISTORY_REQUEST_SCHEMA => {
            let summary = validate_operation_history_request_value(&value)?;
            let runtime = runtime_contract_from_path(manifest_path)?;
            validate_runtime_request_app(&summary.app_id, &runtime.app_id, "operation history")?;
            let limit = summary.limit.to_string();
            run_history_output_with_runtime(
                &runtime,
                &summary.subject,
                Some(limit.as_str()),
                manifest_path.parent(),
            )
        }
        other => {
            Err(TheurgyError::new(format!("unsupported runtime request schema: {other}")).into())
        }
    }
}

fn validate_runtime_request_app(request_app: &str, runtime_app: &str, label: &str) -> Result<()> {
    if request_app != runtime_app {
        return Err(TheurgyError::new(format!(
            "runtime {label} request app mismatch: expected {runtime_app}, got {request_app}"
        ))
        .into());
    }
    Ok(())
}

fn run_state_output(manifest_path: &Path) -> Result<String> {
    let runtime = runtime_contract_from_path(manifest_path)?;
    run_state_output_with_runtime(&runtime, manifest_path.parent())
}

fn run_state_output_with_runtime(
    runtime: &RuntimeContract,
    manifest_dir: Option<&Path>,
) -> Result<String> {
    let command = product_runtime::runtime_state_command(&runtime)?;
    let output = run_manifest_command(&command, "state", manifest_dir)?;
    validate_manifest_state_output(&runtime.app_id, &output)?;
    Ok(output)
}

fn run_status_output(manifest_path: &Path) -> Result<String> {
    let runtime = runtime_contract_from_path(manifest_path)?;
    run_status_output_with_runtime(&runtime, manifest_path.parent())
}

fn run_status_output_with_runtime(
    runtime: &RuntimeContract,
    manifest_dir: Option<&Path>,
) -> Result<String> {
    let command = product_runtime::runtime_status_command(&runtime)?;
    let output = run_manifest_command(&command, "status", manifest_dir)?;
    validate_manifest_status_output(&runtime.app_id, &output)?;
    Ok(output)
}

fn run_operation_status_output(manifest_path: &Path, operation_id: &str) -> Result<String> {
    let runtime = runtime_contract_from_path(manifest_path)?;
    run_operation_status_output_with_runtime(&runtime, operation_id, manifest_path.parent())
}

fn run_operation_status_output_with_runtime(
    runtime: &RuntimeContract,
    operation_id: &str,
    manifest_dir: Option<&Path>,
) -> Result<String> {
    let command = product_runtime::runtime_operation_status_command(&runtime, operation_id)?;
    let output = run_manifest_command(&command, "operation status", manifest_dir)?;
    validate_manifest_operation_status_output(&runtime.app_id, &output)?;
    Ok(output)
}

fn subscribe_status_output(manifest_path: &Path) -> Result<String> {
    let runtime = runtime_contract_from_path(manifest_path)?;
    subscribe_status_output_with_runtime(&runtime, manifest_path.parent())
}

fn subscribe_status_output_with_runtime(
    runtime: &RuntimeContract,
    manifest_dir: Option<&Path>,
) -> Result<String> {
    let command = product_runtime::runtime_subscribe_status_command(&runtime)?;
    let output = run_manifest_command(&command, "status", manifest_dir)?;
    validate_manifest_status_output(&runtime.app_id, &output)?;
    Ok(output)
}

fn run_history_output(manifest_path: &Path, subject: &str, limit: Option<&str>) -> Result<String> {
    let runtime = runtime_contract_from_path(manifest_path)?;
    run_history_output_with_runtime(&runtime, subject, limit, manifest_path.parent())
}

fn run_history_output_with_runtime(
    runtime: &RuntimeContract,
    subject: &str,
    limit: Option<&str>,
    manifest_dir: Option<&Path>,
) -> Result<String> {
    let command = product_runtime::runtime_history_command(&runtime, subject, limit)?;
    let output = run_manifest_command(&command, "history", manifest_dir)?;
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
        validate_runtime_action_dispatch_request(&runtime, action_id, params)?;
        return run_manifest_action(&runtime, action_id, params, path.parent());
    }
    Ok(format!(
        "{{\n  \"success\": true,\n  \"protocol\": \"{}\",\n  \"app\": \"theurgy-runtime\",\n  \"action\": \"{}\",\n  \"operation\": {{\n    \"id\": \"op-{}\",\n    \"status\": \"accepted\",\n    \"progress\": 0,\n    \"longRunning\": false\n  }},\n  \"params\": {}\n}}",
        product_runtime::RUNTIME_ACTION_PROTOCOL,
        json_escape(action_id),
        json_escape(action_id),
        params
    ))
}

fn validate_runtime_action_dispatch_request(
    runtime: &RuntimeContract,
    action_id: &str,
    params: &str,
) -> Result<()> {
    let params_value = parse_json(params)?;
    let request = json!({
        "schema": product_runtime::RUNTIME_ACTION_REQUEST_SCHEMA,
        "protocol": product_runtime::RUNTIME_ACTION_PROTOCOL,
        "app": runtime.app_id,
        "action": action_id,
        "params": params_value
    });
    let summary = validate_runtime_action_request_value(&request)?;
    validate_runtime_action_request_against_runtime(&summary, &request, runtime)
}

fn run_manifest_action(
    runtime: &RuntimeContract,
    action_id: &str,
    params: &str,
    manifest_dir: Option<&Path>,
) -> Result<String> {
    let command = product_runtime::runtime_action_command(runtime, action_id, params)?;
    let output = run_manifest_action_command(
        &command,
        action_id,
        runtime.product_action_contracts.as_deref(),
        manifest_dir,
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
    action_id: &str,
    contracts: Option<&[ActionContract]>,
    manifest_dir: Option<&Path>,
) -> Result<String> {
    let Some(executable) = command.first() else {
        return Err(TheurgyError::new("runtime manifest action command required").into());
    };
    let executable_path = resolve_manifest_executable(executable, manifest_dir);
    let output = Command::new(&executable_path)
        .args(&command[1..])
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
    product_runtime::validate_manifest_action_output_text(
        expected_app,
        action_id,
        output,
        contracts,
    )
    .map_err(Into::into)
}

fn validate_manifest_state_output(expected_app: &str, output: &str) -> Result<()> {
    product_runtime::validate_manifest_state_output_text(expected_app, output).map_err(Into::into)
}

fn validate_manifest_status_output(expected_app: &str, output: &str) -> Result<()> {
    product_runtime::validate_manifest_status_output_text(expected_app, output).map_err(Into::into)
}

fn validate_manifest_operation_status_output(expected_app: &str, output: &str) -> Result<()> {
    product_runtime::validate_manifest_operation_status_output_text(expected_app, output)
        .map_err(Into::into)
}

fn validate_manifest_history_output(expected_app: &str, output: &str) -> Result<()> {
    product_runtime::validate_manifest_history_output_text(expected_app, output).map_err(Into::into)
}

fn validate_runtime_action_failure_keys(
    action_id: &str,
    output: &str,
    contracts: &[ActionContract],
) -> Result<()> {
    product_runtime::validate_runtime_action_failure_contract_text(action_id, output, contracts)
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
    product_runtime::load_runtime_bridge(path).map_err(Into::into)
}

fn runtime_contract_from_path_with_product_actions(path: &Path) -> Result<RuntimeContract> {
    product_runtime::load_runtime_bridge_with_product_actions(path).map_err(Into::into)
}

fn run_manifest_command(
    command: &[String],
    label: &str,
    manifest_dir: Option<&Path>,
) -> Result<String> {
    let Some(executable) = command.first() else {
        return Err(TheurgyError::new(format!("runtime manifest {label} command required")).into());
    };
    let executable_path = resolve_manifest_executable(executable, manifest_dir);
    let output = Command::new(&executable_path)
        .args(&command[1..])
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

fn resolve_manifest_executable(executable: &str, manifest_dir: Option<&Path>) -> PathBuf {
    let executable_path = Path::new(executable);
    if executable_path.is_absolute() || executable.contains(std::path::MAIN_SEPARATOR) {
        return executable_path.to_path_buf();
    }
    if let Some(manifest_dir) = manifest_dir {
        for candidate in [
            manifest_dir.join(executable),
            manifest_dir.join("bin").join(executable),
            manifest_dir.join("libexec").join(executable),
        ] {
            if is_executable_file(&candidate) {
                return candidate;
            }
        }
    }
    executable_path.to_path_buf()
}

#[cfg(unix)]
fn is_executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata()
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable_file(path: &Path) -> bool {
    path.is_file()
}

type ProductSummary = product_runtime::ProductIr;
type ActionContract = product_runtime::ProductActionContract;
type ActionSummary = product_runtime::ActionIr;
type StateSnapshotSummary = product_runtime::StateSnapshot;
type RuntimeStatusSummary = product_runtime::RuntimeStatus;
type RuntimeStateRequestSummary = product_runtime::RuntimeStateRequest;
type RuntimeStatusRequestSummary = product_runtime::RuntimeStatusRequest;
type RuntimeSubscribeStatusRequestSummary = product_runtime::RuntimeSubscribeStatusRequest;
type RuntimeActionRequestSummary = product_runtime::RuntimeActionRequest;
type RuntimeActionResultSummary = product_runtime::RuntimeActionResult;
type OperationStatusRequestSummary = product_runtime::OperationStatusRequest;
type OperationHistoryRequestSummary = product_runtime::OperationHistoryRequest;
type OperationStatusSummary = product_runtime::OperationStatus;
type OperationHistorySummary = product_runtime::OperationHistory;
#[cfg(test)]
type RuntimeManifestSummary = product_runtime::RuntimeManifest;
type GeneratedRuntimeSummary = product_runtime::GeneratedRuntime;
type SurfaceSummary = product_runtime::SurfaceIr;
type RuntimeContract = product_runtime::RuntimeBridge;
type ManifestSummary = product_runtime::ProjectManifest;

fn read_json(path: &Path) -> Result<String> {
    let text = fs::read_to_string(path).map_err(|error| {
        TheurgyError::new(format!("could not read {}: {error}", path.display()))
    })?;
    validate_json_params(&text)?;
    Ok(text)
}

fn validate_generated_runtime(text: &str) -> Result<GeneratedRuntimeSummary> {
    product_runtime::validate_generated_runtime_text(text).map_err(Into::into)
}

#[cfg(test)]
fn validate_runtime_manifest(text: &str) -> Result<RuntimeManifestSummary> {
    let value = parse_json(text)?;
    validate_runtime_manifest_value(&value)
}

#[cfg(test)]
fn runtime_contract_from_manifest(text: &str) -> Result<RuntimeContract> {
    product_runtime::runtime_bridge_from_manifest_text(text).map_err(Into::into)
}

fn validate_surface_ir(text: &str) -> Result<SurfaceSummary> {
    let value = parse_json(text)?;
    product_runtime::validate_surface_ir_value(&value).map_err(Into::into)
}

fn validate_product_ir(text: &str) -> Result<ProductSummary> {
    let value = parse_json(text)?;
    validate_product_ir_value(&value)
}

fn validate_action_ir(text: &str) -> Result<ActionSummary> {
    let value = parse_json(text)?;
    product_runtime::validate_action_ir_value(&value).map_err(Into::into)
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
    product_runtime::validate_state_snapshot_value(value).map_err(Into::into)
}

fn validate_runtime_status_value(value: &Value) -> Result<RuntimeStatusSummary> {
    product_runtime::validate_runtime_status_value(value).map_err(Into::into)
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
    product_runtime::validate_runtime_action_request_value(value).map_err(Into::into)
}

fn validate_runtime_action_request_against_runtime(
    summary: &RuntimeActionRequestSummary,
    value: &Value,
    runtime: &RuntimeContract,
) -> Result<()> {
    let _ = summary;
    product_runtime::validate_runtime_action_request_against_bridge_value(value, runtime)
        .map_err(Into::into)
}

fn validate_runtime_action_result_value(value: &Value) -> Result<RuntimeActionResultSummary> {
    product_runtime::validate_runtime_action_result_value(value).map_err(Into::into)
}

fn validate_runtime_state_request(text: &str) -> Result<RuntimeStateRequestSummary> {
    let value = parse_json(text)?;
    validate_runtime_state_request_value(&value)
}

fn validate_runtime_state_request_value(value: &Value) -> Result<RuntimeStateRequestSummary> {
    product_runtime::validate_runtime_state_request_value(value).map_err(Into::into)
}

fn validate_runtime_status_request(text: &str) -> Result<RuntimeStatusRequestSummary> {
    let value = parse_json(text)?;
    validate_runtime_status_request_value(&value)
}

fn validate_runtime_status_request_value(value: &Value) -> Result<RuntimeStatusRequestSummary> {
    product_runtime::validate_runtime_status_request_value(value).map_err(Into::into)
}

fn validate_runtime_subscribe_status_request(
    text: &str,
) -> Result<RuntimeSubscribeStatusRequestSummary> {
    let value = parse_json(text)?;
    validate_runtime_subscribe_status_request_value(&value)
}

fn validate_runtime_subscribe_status_request_value(
    value: &Value,
) -> Result<RuntimeSubscribeStatusRequestSummary> {
    product_runtime::validate_runtime_subscribe_status_request_value(value).map_err(Into::into)
}

fn validate_operation_status(text: &str) -> Result<OperationStatusSummary> {
    let value = parse_json(text)?;
    validate_operation_status_value(&value)
}

fn validate_operation_status_value(value: &Value) -> Result<OperationStatusSummary> {
    product_runtime::validate_operation_status_value(value).map_err(Into::into)
}

fn validate_operation_status_request(text: &str) -> Result<OperationStatusRequestSummary> {
    let value = parse_json(text)?;
    validate_operation_status_request_value(&value)
}

fn validate_operation_status_request_value(value: &Value) -> Result<OperationStatusRequestSummary> {
    product_runtime::validate_operation_status_request_value(value).map_err(Into::into)
}

fn validate_operation_history(text: &str) -> Result<OperationHistorySummary> {
    let value = parse_json(text)?;
    validate_operation_history_value(&value)
}

fn validate_operation_history_value(value: &Value) -> Result<OperationHistorySummary> {
    product_runtime::validate_operation_history_value(value).map_err(Into::into)
}

fn validate_operation_history_request(text: &str) -> Result<OperationHistoryRequestSummary> {
    let value = parse_json(text)?;
    validate_operation_history_request_value(&value)
}

fn validate_operation_history_request_value(
    value: &Value,
) -> Result<OperationHistoryRequestSummary> {
    product_runtime::validate_operation_history_request_value(value).map_err(Into::into)
}

fn validate_product_ir_value(value: &Value) -> Result<ProductSummary> {
    product_runtime::validate_product_ir_value(value).map_err(Into::into)
}

fn parse_json(text: &str) -> Result<Value> {
    serde_json::from_str(text)
        .map_err(|error| TheurgyError::new(format!("invalid JSON: {error}")).into())
}

#[cfg(test)]
fn validate_runtime_manifest_value(value: &Value) -> Result<RuntimeManifestSummary> {
    product_runtime::validate_runtime_manifest_value(value).map_err(Into::into)
}

fn validate_json_params(raw: &str) -> Result<()> {
    let value = parse_json(raw)?;
    if value.is_object() || value.is_array() {
        Ok(())
    } else {
        Err(TheurgyError::new("expected a JSON object or array literal").into())
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
    let runtime = product_runtime::default_runtime_bridge_for_product(&summary);
    let runtime_manifest = generated_runtime_manifest_text(&runtime, target)?;
    let contract_resources = [
        (runtime.product_ir.as_str(), product),
        (runtime.runtime_manifest.as_str(), runtime_manifest.as_str()),
        (runtime.source_surface_ir.as_str(), surface.as_str()),
    ];
    compile_native_with_contract(
        &summary,
        &surface,
        &runtime,
        target,
        out_dir,
        false,
        &contract_resources,
    )
}

fn generated_runtime_manifest_text(runtime: &RuntimeContract, target: &str) -> Result<String> {
    let mut runtime_object = serde_json::Map::new();
    runtime_object.insert(
        "stateCommand".to_string(),
        json!(runtime.state_command.clone()),
    );
    if !runtime.status_command.is_empty() {
        runtime_object.insert(
            "statusCommand".to_string(),
            json!(runtime.status_command.clone()),
        );
    }
    if !runtime.subscribe_status_command.is_empty() {
        runtime_object.insert(
            "subscribeStatusCommand".to_string(),
            json!(runtime.subscribe_status_command.clone()),
        );
    }
    if !runtime.operation_status_command.is_empty() {
        runtime_object.insert(
            "operationStatusCommand".to_string(),
            json!(runtime.operation_status_command.clone()),
        );
    }
    runtime_object.insert(
        "actionCommand".to_string(),
        json!(runtime.action_command.clone()),
    );
    if !runtime.history_command.is_empty() {
        runtime_object.insert(
            "historyCommand".to_string(),
            json!(runtime.history_command.clone()),
        );
    }
    if !runtime.daemon_command.is_empty() {
        runtime_object.insert(
            "daemonCommand".to_string(),
            json!(runtime.daemon_command.clone()),
        );
    }
    runtime_object.insert("protocol".to_string(), json!(runtime.protocol.clone()));

    let mut surfaces = serde_json::Map::new();
    let surface_key = if product_runtime::is_mobile_target(target) {
        "mobile"
    } else {
        "desktop"
    };
    surfaces.insert(
        surface_key.to_string(),
        json!(runtime.source_surface_ir.clone()),
    );
    if let Some(legacy) = &runtime.legacy_native_desktop_ir {
        surfaces.insert("legacyNativeDesktop".to_string(), json!(legacy.clone()));
    }

    let manifest = json!({
        "version": "theurgy-runtime-manifest/v1",
        "app": runtime.app_id,
        "productIr": runtime.product_ir,
        "runtime": Value::Object(runtime_object),
        "surfaces": Value::Object(surfaces),
        "compatibility": {
            "wizardryAppsShellFirstStillSupported": runtime.compatibility.wizardry_apps_shell_first_still_supported,
            "theurgyRequiredForLegacyWizardryApps": runtime.compatibility.theurgy_required_for_legacy_wizardry_apps
        }
    });
    product_runtime::validate_runtime_manifest_value(&manifest)?;
    serde_json::to_string_pretty(&manifest)
        .map(|json| format!("{json}\n"))
        .map_err(|error| {
            TheurgyError::new(format!("could not serialize runtime manifest: {error}")).into()
        })
}

fn compile_native_with_contract(
    summary: &ProductSummary,
    surface: &str,
    runtime: &RuntimeContract,
    target: &str,
    out_dir: &Path,
    preserve_existing_legacy_desktop_adapter: bool,
    contract_resources: &[(&str, &str)],
) -> Result<()> {
    let surface_summary = validate_surface_ir(surface)?;
    fs::create_dir_all(out_dir)?;
    let product = summary.clone();
    let surface_ir = surface_summary.clone();
    let files =
        product_runtime::native_compile_files(&product, &surface_ir, surface, runtime, target)?;
    if preserve_existing_legacy_desktop_adapter
        && product_runtime::is_desktop_target(target)
        && desktop_adapter_source_exists(target, out_dir)
    {
        for file in files.into_iter().filter(|file| {
            file.path == "theurgy-surface.json" || file.path == "theurgy-runtime.json"
        }) {
            write_or_replace(&out_dir.join(file.path), &file.contents)?;
        }
        write_app_contract_resources(out_dir, target, contract_resources)?;
        return Ok(());
    }
    for file in files {
        write_or_replace(&out_dir.join(file.path), &file.contents)?;
    }
    write_app_contract_resources(out_dir, target, contract_resources)?;
    Ok(())
}

fn write_app_contract_resources(
    out_dir: &Path,
    target: &str,
    contract_resources: &[(&str, &str)],
) -> Result<()> {
    if !product_runtime::is_desktop_target(target) {
        return Ok(());
    }
    for (relative_path, text) in contract_resources {
        validate_generated_relative_path(relative_path)?;
        let text = format!("{}\n", text.trim_end());
        write_or_replace(&out_dir.join(relative_path), &text)?;
        if target == "macos" {
            write_or_replace(
                &out_dir.join("Sources/App/Resources").join(relative_path),
                &text,
            )?;
        }
        if relative_path.ends_with("runtime.manifest.json") {
            write_or_replace(&out_dir.join("runtime.manifest.json"), &text)?;
            if target == "macos" {
                write_or_replace(
                    &out_dir.join("Sources/App/Resources/runtime.manifest.json"),
                    &text,
                )?;
            }
        }
    }
    Ok(())
}

fn stage_app_runtime_binaries(
    app_dir: &Path,
    out_dir: &Path,
    target: &str,
    runtime: &RuntimeContract,
) -> Result<()> {
    let mut app_binaries = BTreeSet::new();
    collect_manifest_binary(&runtime.state_command, &mut app_binaries)?;
    collect_manifest_binary(&runtime.status_command, &mut app_binaries)?;
    collect_manifest_binary(&runtime.subscribe_status_command, &mut app_binaries)?;
    collect_manifest_binary(&runtime.operation_status_command, &mut app_binaries)?;
    collect_manifest_binary(&runtime.action_command, &mut app_binaries)?;
    collect_manifest_binary(&runtime.history_command, &mut app_binaries)?;
    collect_manifest_binary(&runtime.daemon_command, &mut app_binaries)?;

    let cargo_manifest = app_dir.join("Cargo.toml");
    if !cargo_manifest.is_file() {
        return Err(TheurgyError::new(format!(
            "stage-app-runtime requires Cargo.toml at {}",
            cargo_manifest.display()
        ))
        .into());
    }
    for binary in &app_binaries {
        let status = Command::new("cargo")
            .arg("build")
            .arg("--manifest-path")
            .arg(&cargo_manifest)
            .arg("--bin")
            .arg(binary)
            .status()
            .map_err(|error| {
                TheurgyError::new(format!("could not run cargo build for {binary}: {error}"))
            })?;
        if !status.success() {
            return Err(TheurgyError::new(format!(
                "cargo build failed for runtime binary: {binary}"
            ))
            .into());
        }
    }

    let theurgy_runtime = env::current_exe()
        .map_err(|error| TheurgyError::new(format!("could not locate theurgy-runtime: {error}")))?;
    let mut binaries = vec![("theurgy-runtime".to_string(), theurgy_runtime)];
    let cargo_debug_dir = cargo_debug_dir(app_dir);
    for binary in app_binaries {
        let binary_path = cargo_debug_dir.join(&binary);
        if !binary_path.is_file() {
            return Err(TheurgyError::new(format!(
                "built runtime binary not found: {}",
                binary_path.display()
            ))
            .into());
        }
        binaries.push((binary, binary_path));
    }

    for libexec_dir in runtime_libexec_dirs(out_dir, target)? {
        fs::create_dir_all(&libexec_dir)?;
        for (name, source) in &binaries {
            let destination = libexec_dir.join(name);
            fs::copy(source, &destination).map_err(|error| {
                TheurgyError::new(format!(
                    "could not copy {} to {}: {error}",
                    source.display(),
                    destination.display()
                ))
            })?;
            mark_executable(&destination)?;
        }
    }
    Ok(())
}

fn collect_manifest_binary(command: &[String], binaries: &mut BTreeSet<String>) -> Result<()> {
    let Some(binary) = command.first() else {
        return Ok(());
    };
    validate_bare_runtime_binary(binary)?;
    binaries.insert(binary.clone());
    Ok(())
}

fn validate_bare_runtime_binary(binary: &str) -> Result<()> {
    let path = Path::new(binary);
    if path.is_absolute() || path.components().count() != 1 {
        return Err(TheurgyError::new(format!(
            "stage-app-runtime only stages bare runtime binary names, got: {binary}"
        ))
        .into());
    }
    if binary == "theurgy-runtime" {
        return Err(TheurgyError::new(
            "runtime manifests should not declare theurgy-runtime as an app binary",
        )
        .into());
    }
    Ok(())
}

fn validate_generated_relative_path(relative_path: &str) -> Result<()> {
    let path = Path::new(relative_path);
    if path.is_absolute() {
        return Err(TheurgyError::new(format!(
            "generated resource path must be relative: {relative_path}"
        ))
        .into());
    }
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(TheurgyError::new(format!(
            "generated resource path must stay inside artifact: {relative_path}"
        ))
        .into());
    }
    Ok(())
}

fn cargo_debug_dir(app_dir: &Path) -> PathBuf {
    env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| app_dir.join("target"))
        .join("debug")
}

fn runtime_libexec_dirs(out_dir: &Path, target: &str) -> Result<Vec<PathBuf>> {
    match target {
        "macos" => Ok(vec![
            out_dir.join("libexec"),
            out_dir.join("Sources/App/Resources/libexec"),
        ]),
        "linux" => Ok(vec![out_dir.join("libexec")]),
        _ => Err(TheurgyError::new("stage-app-runtime only supports macos and linux").into()),
    }
}

fn mark_executable(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(permissions.mode() | 0o755);
        fs::set_permissions(path, permissions)?;
    }
    Ok(())
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

fn write_or_replace(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
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

fn inspect_manifest(manifest: &str) -> Result<ManifestSummary> {
    product_runtime::validate_project_manifest_text(manifest).map_err(Into::into)
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
        assert_eq!(summary.name, "demo");
        assert_eq!(summary.kind, "desktop");
        assert_eq!(summary.source_root, "src");
        assert_eq!(summary.truth.as_deref(), Some("file-first"));
    }

    #[test]
    fn inspect_app_summarizes_product_runtime_and_surface_contracts() {
        let app = test_root("inspect-app-contract");
        fs::create_dir_all(app.join("app-blueprint")).unwrap();
        write_or_replace(
            &app.join("theurgy.project.toml"),
            "name = \"deployments\"\nkind = \"desktop\"\nsource_root = \"src\"\nproduct_ir = \"app-blueprint/product.ir.json\"\ndesktop_surface_ir = \"app-blueprint/desktop.surface.ir.json\"\nmobile_surface_ir = \"app-blueprint/mobile.surface.ir.json\"\nruntime_manifest = \"app-blueprint/runtime.manifest.json\"\n",
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/product.ir.json"),
            &sample_product_with_state_commands("app-blueprint/desktop.surface.ir.json"),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/runtime.manifest.json"),
            &sample_runtime_manifest().replace(
                "\"desktop\": \"app-blueprint/desktop.surface.ir.json\",\n    \"legacyNativeDesktop\": \"app-blueprint/app.ir.yaml\"",
                "\"desktop\": \"app-blueprint/desktop.surface.ir.json\",\n    \"mobile\": \"app-blueprint/mobile.surface.ir.json\",\n    \"legacyNativeDesktop\": \"app-blueprint/app.ir.yaml\"",
            ),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/desktop.surface.ir.json"),
            &sample_desktop_surface(),
        )
        .unwrap();
        write_or_replace(
            &app.join("app-blueprint/mobile.surface.ir.json"),
            "{\n  \"version\": \"theurgy-mobile-surface-ir/v1\",\n  \"format\": \"json\",\n  \"product\": \"deployments\",\n  \"target\": \"mobile\",\n  \"actions\": [\"refresh_state\", \"publish_changes\"],\n  \"screens\": [\n    {\"id\": \"overview\", \"title\": \"Deployments\", \"node\": {\"id\": \"screen.overview\", \"type\": \"NavigationStack\", \"role\": \"status-overview\"}},\n    {\"id\": \"deployment-detail\", \"title\": \"Deployment\", \"node\": {\"id\": \"screen.deployment-detail\", \"type\": \"Screen\", \"role\": \"focused-action-detail\", \"children\": [{\"id\": \"section.actions\", \"type\": \"Section\", \"role\": \"safe-mobile-actions\"}]}}\n  ]\n}\n",
        )
        .unwrap();

        let lines = product_runtime::load_app_inspection_lines(&app).unwrap();
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
        assert!(lines.contains(&"product_state_projections=deployments".to_string()));
        assert!(lines.contains(&"product_domain_objects=server,deployment".to_string()));
        assert!(lines.contains(&"product_domain_object_server_label=Server".to_string()));
        assert!(lines.contains(&"product_domain_object_deployment_label=Deployment".to_string()));
        assert!(lines.contains(&"product_persistence_roots=headquarters-workspace".to_string()));
        assert!(lines.contains(
            &"product_persistence_root_headquarters-workspace_kind=xdg-state".to_string()
        ));
        assert!(lines.contains(
            &"product_persistence_root_headquarters-workspace_path=${XDG_STATE_HOME:-$HOME/.local/state}/wizardry-apps/headquarters/workspaces/<workspace-key>".to_string()
        ));
        assert!(
            lines.contains(&"product_background_job_server-queue_label=Server Queue".to_string())
        );
        assert!(lines
            .contains(&"product_background_job_server-queue_state=server.queue_mode".to_string()));
        assert!(lines.contains(
            &"product_background_job_server-queue_command=deployments-daemon".to_string()
        ));
        assert!(lines.contains(&"product_release_targets=macos-app,linux-app".to_string()));
        assert!(lines.contains(&"product_release_target_macos-app_target=macos".to_string()));
        assert!(lines.contains(&"product_release_target_macos-app_surface=desktop".to_string()));
        assert!(lines
            .contains(&"product_release_target_macos-app_artifact=generated/macos".to_string()));
        assert!(lines.contains(&"runtime_protocol=deployments-runtime/v1".to_string()));
        assert!(lines
            .contains(&"runtime_legacy_native_desktop_ir=app-blueprint/app.ir.yaml".to_string()));
        assert!(lines.contains(
            &"runtime_operation_status_command=custom-core operation-status".to_string()
        ));
        assert!(lines.contains(&"desktop_surface_actions=2".to_string()));
        assert!(lines.contains(&"mobile_surface_actions=2".to_string()));
        assert!(lines.contains(&"mobile_surface_screens=overview,deployment-detail".to_string()));
        assert!(lines.contains(&"mobile_surface_screen_overview_node=screen.overview".to_string()));
        assert!(lines.contains(
            &"mobile_surface_screen_deployment-detail_role=focused-action-detail".to_string()
        ));
        assert!(lines.contains(
            &"mobile_surface_screen_deployment-detail_roles=focused-action-detail,safe-mobile-actions"
                .to_string()
        ));
        assert!(lines.contains(&"mobile_surface_screen_deployment-detail_actions=".to_string()));
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

        let error = product_runtime::load_app_inspection_lines(&app)
            .unwrap_err()
            .to_string();
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
        assert_eq!(summary.persistence_roots[0].kind, "xdg-state".to_string());
        assert_eq!(
            summary.persistence_roots[0].path,
            Some("${XDG_STATE_HOME:-$HOME/.local/state}/wizardry-apps/headquarters/workspaces/<workspace-key>".to_string())
        );
        assert_eq!(summary.background_job_ids, vec!["server-queue".to_string()]);
        assert_eq!(summary.background_jobs[0].label, "Server Queue".to_string());
        assert_eq!(
            summary.background_jobs[0].state,
            Some("server.queue_mode".to_string())
        );
        assert_eq!(
            summary
                .release_targets
                .iter()
                .map(|release_target| release_target.id.clone())
                .collect::<Vec<_>>(),
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
    fn product_ir_schema_declares_platform_capability_requirements() {
        let schema: Value =
            serde_json::from_str(include_str!("../schemas/theurgy-product-ir-v1.json")).unwrap();
        assert_eq!(
            schema
                .pointer("/properties/app/properties/capabilities/items/minLength")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            schema
                .pointer("/properties/app/allOf/0/if/properties/targets/contains/enum")
                .and_then(Value::as_array)
                .map(|values| { values.iter().filter_map(Value::as_str).collect::<Vec<_>>() }),
            Some(vec!["macos", "linux"])
        );
        assert_eq!(
            schema
                .pointer("/properties/app/allOf/0/then/required/0")
                .and_then(Value::as_str),
            Some("capabilities")
        );
        assert_eq!(
            schema
                .pointer("/properties/app/allOf/0/then/properties/capabilities/contains/const")
                .and_then(Value::as_str),
            Some("native-desktop")
        );
        assert_eq!(
            schema
                .pointer("/properties/app/allOf/1/if/properties/targets/contains/enum")
                .and_then(Value::as_array)
                .map(|values| { values.iter().filter_map(Value::as_str).collect::<Vec<_>>() }),
            Some(vec!["ios", "android"])
        );
        assert_eq!(
            schema
                .pointer("/properties/app/allOf/1/then/required/0")
                .and_then(Value::as_str),
            Some("capabilities")
        );
        assert_eq!(
            schema
                .pointer("/properties/app/allOf/1/then/properties/capabilities/contains/const")
                .and_then(Value::as_str),
            Some("native-mobile")
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
    fn runtime_manifest_schema_rejects_empty_command_entries() {
        let schema: Value =
            serde_json::from_str(include_str!("../schemas/theurgy-runtime-manifest-v1.json"))
                .unwrap();
        assert_eq!(
            schema
                .pointer("/$defs/command/items/minLength")
                .and_then(Value::as_u64),
            Some(1)
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
                .pointer("/properties/schema/const")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action-request/v1")
        );
        assert!(schema
            .pointer("/required")
            .and_then(Value::as_array)
            .is_some_and(|required| required.iter().any(|item| item == "schema")));
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
    fn operation_request_schemas_declare_mobile_bridge_envelopes() {
        let state_schema: Value = serde_json::from_str(include_str!(
            "../schemas/theurgy-runtime-state-request-v1.json"
        ))
        .unwrap();
        assert_eq!(
            state_schema
                .pointer("/properties/schema/const")
                .and_then(Value::as_str),
            Some("theurgy-runtime-state-request/v1")
        );
        assert_eq!(
            state_schema
                .pointer("/properties/protocol/const")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action/v1")
        );
        assert_eq!(
            state_schema
                .pointer("/properties/kind/const")
                .and_then(Value::as_str),
            Some("state")
        );

        let runtime_status_schema: Value = serde_json::from_str(include_str!(
            "../schemas/theurgy-runtime-status-request-v1.json"
        ))
        .unwrap();
        assert_eq!(
            runtime_status_schema
                .pointer("/properties/schema/const")
                .and_then(Value::as_str),
            Some("theurgy-runtime-status-request/v1")
        );
        assert_eq!(
            runtime_status_schema
                .pointer("/properties/kind/const")
                .and_then(Value::as_str),
            Some("status")
        );

        let runtime_subscribe_status_schema: Value = serde_json::from_str(include_str!(
            "../schemas/theurgy-runtime-subscribe-status-request-v1.json"
        ))
        .unwrap();
        assert_eq!(
            runtime_subscribe_status_schema
                .pointer("/properties/schema/const")
                .and_then(Value::as_str),
            Some("theurgy-runtime-subscribe-status-request/v1")
        );
        assert_eq!(
            runtime_subscribe_status_schema
                .pointer("/properties/kind/const")
                .and_then(Value::as_str),
            Some("subscribe-status")
        );

        let status_schema: Value = serde_json::from_str(include_str!(
            "../schemas/theurgy-operation-status-request-v1.json"
        ))
        .unwrap();
        assert_eq!(
            status_schema
                .pointer("/properties/schema/const")
                .and_then(Value::as_str),
            Some("theurgy-operation-status-request/v1")
        );
        assert_eq!(
            status_schema
                .pointer("/properties/protocol/const")
                .and_then(Value::as_str),
            Some("theurgy-runtime-action/v1")
        );
        assert_eq!(
            status_schema
                .pointer("/properties/kind/const")
                .and_then(Value::as_str),
            Some("operation-status")
        );
        assert_eq!(
            status_schema
                .pointer("/properties/operation/minLength")
                .and_then(Value::as_u64),
            Some(1)
        );

        let history_schema: Value = serde_json::from_str(include_str!(
            "../schemas/theurgy-operation-history-request-v1.json"
        ))
        .unwrap();
        assert_eq!(
            history_schema
                .pointer("/properties/schema/const")
                .and_then(Value::as_str),
            Some("theurgy-operation-history-request/v1")
        );
        assert_eq!(
            history_schema
                .pointer("/properties/kind/const")
                .and_then(Value::as_str),
            Some("operation-history")
        );
        assert_eq!(
            history_schema
                .pointer("/properties/subject/minLength")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            history_schema
                .pointer("/properties/limit/minimum")
                .and_then(Value::as_u64),
            Some(1)
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
        assert_eq!(
            mobile_schema
                .pointer("/$defs/screen/properties/actions/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/actionIdList")
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
                .pointer("/properties/surfaceCapabilities/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/stringList")
        );
        assert!(schema
            .pointer("/required")
            .and_then(Value::as_array)
            .is_some_and(|required| required
                .iter()
                .any(|value| value.as_str() == Some("surfaceCapabilities"))));
        assert_eq!(
            schema
                .pointer("/properties/productBackgroundJobContracts/items/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/backgroundJobContract")
        );
        assert_eq!(
            schema
                .pointer("/properties/productPersistenceRootContracts/items/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/persistenceRootContract")
        );
        assert_eq!(
            schema
                .pointer("/properties/productReleaseTargetContracts/items/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/releaseTargetContract")
        );
        assert_eq!(
            schema
                .pointer("/properties/productDomainObjectContracts/items/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/domainObjectContract")
        );
        assert_eq!(
            schema
                .pointer("/$defs/mobileScreenContract/required")
                .and_then(Value::as_array)
                .map(|values| values.iter().filter_map(Value::as_str).collect::<Vec<_>>()),
            Some(vec![
                "id", "title", "nodeId", "nodeType", "roles", "actions"
            ])
        );
        assert_eq!(
            schema
                .pointer("/$defs/mobileScreenContract/properties/actions/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/actionIdList")
        );
        assert_eq!(
            schema
                .pointer("/properties/subscribeStatusCommand/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/command")
        );
        assert_eq!(
            schema
                .pointer("/properties/requestCommand/$ref")
                .and_then(Value::as_str),
            Some("#/$defs/command")
        );
        assert_eq!(
            schema
                .pointer("/$defs/command/items/minLength")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            schema
                .pointer("/allOf/0/then/properties/requestCommand/const")
                .and_then(Value::as_array)
                .map(|values| values.iter().filter_map(Value::as_str).collect::<Vec<_>>()),
            Some(vec!["theurgy-runtime", "run-request"])
        );
        assert_eq!(
            schema
                .pointer("/allOf/0/then/required")
                .and_then(Value::as_array)
                .map(|values| values.iter().filter_map(Value::as_str).collect::<Vec<_>>()),
            Some(vec!["requestCommand", "requestCommandManifest"])
        );
        assert_eq!(
            schema.pointer("/allOf/1/then/properties/requestCommand"),
            Some(&Value::Bool(false))
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
            .any(|value| value.as_str() == Some("productBackgroundJobContracts")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("productPersistenceRootContracts")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("productReleaseTargetContracts")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("productDomainObjectContracts")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("productStateProjections")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("productIr")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("runtimeStateRequestSchema")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("runtimeStatusRequestSchema")));
        assert!(top_level_required
            .iter()
            .any(|value| value.as_str() == Some("runtimeSubscribeStatusRequestSchema")));
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
                .pointer("/properties/runtimeStateRequestSchema/const")
                .and_then(Value::as_str),
            Some("theurgy-runtime-state-request/v1")
        );
        assert_eq!(
            schema
                .pointer("/properties/runtimeStatusRequestSchema/const")
                .and_then(Value::as_str),
            Some("theurgy-runtime-status-request/v1")
        );
        assert_eq!(
            schema
                .pointer("/properties/runtimeSubscribeStatusRequestSchema/const")
                .and_then(Value::as_str),
            Some("theurgy-runtime-subscribe-status-request/v1")
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
            "\"stateCommand\": [\"custom-core\", \"state\"]",
            "\"stateCommand\": [\"custom-core\", \"\"]",
        );
        let error = runtime_contract_from_manifest(&manifest)
            .unwrap_err()
            .to_string();
        assert_eq!(
            error,
            "runtime manifest stateCommand must contain non-empty strings"
        );
        let manifest = sample_runtime_manifest().replace(
            "\"actionCommand\": [\"custom-core\", \"action\"]",
            "\"actionCommand\": [\"custom-core\", \"\"]",
        );
        let error = runtime_contract_from_manifest(&manifest)
            .unwrap_err()
            .to_string();
        assert_eq!(
            error,
            "runtime manifest actionCommand must contain non-empty strings"
        );
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
    fn runtime_manifest_validation_rejects_empty_optional_commands() {
        for (replacement, expected) in [
            (
                "\"statusCommand\": []",
                "runtime manifest statusCommand must be non-empty",
            ),
            (
                "\"statusCommand\": [\"custom-core\", \"status\"],\n    \"subscribeStatusCommand\": []",
                "runtime manifest subscribeStatusCommand must be non-empty",
            ),
            (
                "\"operationStatusCommand\": []",
                "runtime manifest operationStatusCommand must be non-empty",
            ),
            (
                "\"historyCommand\": []",
                "runtime manifest historyCommand must be non-empty",
            ),
            (
                "\"daemonCommand\": []",
                "runtime manifest daemonCommand must be non-empty",
            ),
        ] {
            let manifest = sample_runtime_manifest()
                .replace("\"statusCommand\": [\"custom-core\", \"status\"]", replacement);
            let error = runtime_contract_from_manifest(&manifest)
                .unwrap_err()
                .to_string();
            assert_eq!(error, expected);
        }
    }

    #[test]
    fn runtime_manifest_validation_rejects_empty_optional_command_entries() {
        for (replacement, expected) in [
            (
                "\"statusCommand\": [\"custom-core\", \"\"]",
                "runtime manifest statusCommand must contain non-empty strings",
            ),
            (
                "\"operationStatusCommand\": [\"custom-core\", \"\"]",
                "runtime manifest operationStatusCommand must contain non-empty strings",
            ),
            (
                "\"historyCommand\": [\"custom-core\", \"\"]",
                "runtime manifest historyCommand must contain non-empty strings",
            ),
            (
                "\"daemonCommand\": [\"\"]",
                "runtime manifest daemonCommand must contain non-empty strings",
            ),
        ] {
            let manifest = sample_runtime_manifest().replace(
                "\"statusCommand\": [\"custom-core\", \"status\"]",
                replacement,
            );
            let error = runtime_contract_from_manifest(&manifest)
                .unwrap_err()
                .to_string();
            assert_eq!(error, expected);
        }
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
        let request = "{\n  \"schema\": \"theurgy-runtime-action-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"publish_changes\",\n  \"params\": {\"deployment\": \"site-one\"}\n}";
        let summary = validate_runtime_action_request(request).unwrap();
        assert_eq!(summary.app_id, "deployments");
        assert_eq!(summary.action_id, "publish_changes");

        let error = validate_runtime_action_request(
            "{\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"publish_changes\",\n  \"params\": {}\n}",
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("expected schema = theurgy-runtime-action-request/v1"));

        let error = validate_runtime_action_request(
            "{\n  \"schema\": \"theurgy-runtime-action-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"Deployments\",\n  \"action\": \"publish_changes\",\n  \"params\": {}\n}",
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("runtime action request app must be a lowercase slug"));

        let error = validate_runtime_action_request(
            "{\n  \"schema\": \"theurgy-runtime-action-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"publish_changes\",\n  \"params\": []\n}",
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("missing JSON object key: params"));
    }

    #[test]
    fn validates_operation_request_contracts() {
        let state_request = "{\n  \"schema\": \"theurgy-runtime-state-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"state\"\n}";
        let summary = validate_runtime_state_request(state_request).unwrap();
        assert_eq!(summary.app_id, "deployments");

        let status_request = "{\n  \"schema\": \"theurgy-runtime-status-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"status\"\n}";
        let summary = validate_runtime_status_request(status_request).unwrap();
        assert_eq!(summary.app_id, "deployments");

        let subscribe_status_request = "{\n  \"schema\": \"theurgy-runtime-subscribe-status-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"subscribe-status\"\n}";
        let summary = validate_runtime_subscribe_status_request(subscribe_status_request).unwrap();
        assert_eq!(summary.app_id, "deployments");

        let status_request = "{\n  \"schema\": \"theurgy-operation-status-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"operation-status\",\n  \"operation\": \"op-publish\"\n}";
        let summary = validate_operation_status_request(status_request).unwrap();
        assert_eq!(summary.app_id, "deployments");
        assert_eq!(summary.operation_id, "op-publish");

        let history_request = "{\n  \"schema\": \"theurgy-operation-history-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"operation-history\",\n  \"subject\": \"deployment:site-one\",\n  \"limit\": 20\n}";
        let summary = validate_operation_history_request(history_request).unwrap();
        assert_eq!(summary.app_id, "deployments");
        assert_eq!(summary.subject, "deployment:site-one");
        assert_eq!(summary.limit, 20);

        let error = validate_operation_status_request(
            "{\n  \"schema\": \"theurgy-operation-status-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments_app\",\n  \"kind\": \"operation-status\",\n  \"operation\": \"op-publish\"\n}",
        )
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "operation status request app must be a lowercase slug"
        );

        let error = validate_operation_history_request(
            "{\n  \"schema\": \"theurgy-operation-history-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"operation-history\",\n  \"subject\": \"deployment:site-one\",\n  \"limit\": 0\n}",
        )
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "operation history request limit must be a positive integer"
        );

        let error = validate_runtime_state_request(
            "{\n  \"schema\": \"theurgy-runtime-state-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"Deployments\",\n  \"kind\": \"state\"\n}",
        )
        .unwrap_err()
        .to_string();
        assert_eq!(error, "runtime state request app must be a lowercase slug");

        let error = validate_runtime_status_request(
            "{\n  \"schema\": \"theurgy-runtime-status-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"state\"\n}",
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("expected kind = status"));

        let error = validate_runtime_subscribe_status_request(
            "{\n  \"schema\": \"theurgy-runtime-subscribe-status-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"status\"\n}",
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("expected kind = subscribe-status"));
    }

    #[test]
    fn command_validates_operation_request_files() {
        let root = runtime_fixture_root("validate-operation-requests");
        let state_request = root.join("state-request.json");
        write_or_replace(
            &state_request,
            "{\n  \"schema\": \"theurgy-runtime-state-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"state\"\n}",
        )
        .unwrap();
        command_validate_runtime_state_request(&[state_request.display().to_string()]).unwrap();

        let runtime_status_request = root.join("runtime-status-request.json");
        write_or_replace(
            &runtime_status_request,
            "{\n  \"schema\": \"theurgy-runtime-status-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"status\"\n}",
        )
        .unwrap();
        command_validate_runtime_status_request(&[runtime_status_request.display().to_string()])
            .unwrap();

        let runtime_subscribe_status_request = root.join("runtime-subscribe-status-request.json");
        write_or_replace(
            &runtime_subscribe_status_request,
            "{\n  \"schema\": \"theurgy-runtime-subscribe-status-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"subscribe-status\"\n}",
        )
        .unwrap();
        command_validate_runtime_subscribe_status_request(&[runtime_subscribe_status_request
            .display()
            .to_string()])
        .unwrap();

        let status_request = root.join("status-request.json");
        write_or_replace(
            &status_request,
            "{\n  \"schema\": \"theurgy-operation-status-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"operation-status\",\n  \"operation\": \"op-publish\"\n}",
        )
        .unwrap();
        command_validate_operation_status_request(&[status_request.display().to_string()]).unwrap();

        let history_request = root.join("history-request.json");
        write_or_replace(
            &history_request,
            "{\n  \"schema\": \"theurgy-operation-history-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"operation-history\",\n  \"subject\": \"deployment:site-one\",\n  \"limit\": 10\n}",
        )
        .unwrap();
        command_validate_operation_history_request(&[history_request.display().to_string()])
            .unwrap();

        let invalid_history_request = root.join("invalid-history-request.json");
        write_or_replace(
            &invalid_history_request,
            "{\n  \"schema\": \"theurgy-operation-history-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"kind\": \"operation-status\",\n  \"subject\": \"deployment:site-one\",\n  \"limit\": 10\n}",
        )
        .unwrap();
        let error = command_validate_operation_history_request(&[invalid_history_request
            .display()
            .to_string()])
        .unwrap_err()
        .to_string();
        assert_eq!(error, "expected kind = operation-history");

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn runtime_action_request_validation_uses_manifest_contract() {
        let root = runtime_fixture_root("validate-action-request");
        let manifest = root.join("runtime.manifest.json");
        let valid_request = root.join("valid-action-request.json");
        write_or_replace(
            &valid_request,
            "{\n  \"schema\": \"theurgy-runtime-action-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"refresh_state\",\n  \"params\": {}\n}",
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
            "{\n  \"schema\": \"theurgy-runtime-action-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"other-app\",\n  \"action\": \"refresh_state\",\n  \"params\": {}\n}",
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
            "{\n  \"schema\": \"theurgy-runtime-action-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"not_declared\",\n  \"params\": {}\n}",
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
            "{\n  \"schema\": \"theurgy-runtime-action-request/v1\",\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"app\": \"deployments\",\n  \"action\": \"refresh_state\",\n  \"params\": {\"force\": true}\n}",
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
            "runtime action request not declared in Product IR: delete_everything"
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
        assert_eq!(error.to_string(), "missing JSON object key: params");

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
    fn run_request_dispatches_typed_runtime_envelopes() {
        let root = runtime_fixture_root("run-request");
        let manifest = root.join("runtime.manifest.json");

        let output = run_request_output(
            "{\"schema\":\"theurgy-runtime-state-request/v1\",\"protocol\":\"theurgy-runtime-action/v1\",\"app\":\"deployments\",\"kind\":\"state\"}",
            &manifest,
        )
        .unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.get("schema").and_then(Value::as_str),
            Some("theurgy-state-snapshot/v1")
        );

        let output = run_request_output(
            "{\"schema\":\"theurgy-runtime-status-request/v1\",\"protocol\":\"theurgy-runtime-action/v1\",\"app\":\"deployments\",\"kind\":\"status\"}",
            &manifest,
        )
        .unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.get("schema").and_then(Value::as_str),
            Some("theurgy-runtime-status/v1")
        );
        assert_eq!(
            value.get("state_ready").and_then(Value::as_bool),
            Some(true)
        );

        let output = run_request_output(
            "{\"schema\":\"theurgy-runtime-subscribe-status-request/v1\",\"protocol\":\"theurgy-runtime-action/v1\",\"app\":\"deployments\",\"kind\":\"subscribe-status\"}",
            &manifest,
        )
        .unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.get("schema").and_then(Value::as_str),
            Some("theurgy-runtime-status/v1")
        );

        let output = run_request_output(
            "{\"schema\":\"theurgy-runtime-action-request/v1\",\"protocol\":\"theurgy-runtime-action/v1\",\"app\":\"deployments\",\"action\":\"refresh_state\",\"params\":{}}",
            &manifest,
        )
        .unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.pointer("/data/action").and_then(Value::as_str),
            Some("refresh_state")
        );

        let output = run_request_output(
            "{\"schema\":\"theurgy-operation-status-request/v1\",\"protocol\":\"theurgy-runtime-action/v1\",\"app\":\"deployments\",\"kind\":\"operation-status\",\"operation\":\"op-refresh_state\"}",
            &manifest,
        )
        .unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.pointer("/operation/id").and_then(Value::as_str),
            Some("op-refresh_state")
        );

        let output = run_request_output(
            "{\"schema\":\"theurgy-operation-history-request/v1\",\"protocol\":\"theurgy-runtime-action/v1\",\"app\":\"deployments\",\"kind\":\"operation-history\",\"subject\":\"site-one\",\"limit\":7}",
            &manifest,
        )
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
        assert_eq!(value.get("limit").and_then(Value::as_str), Some("7"));

        let error = run_request_output(
            "{\"schema\":\"theurgy-runtime-state-request/v1\",\"protocol\":\"theurgy-runtime-action/v1\",\"app\":\"other-app\",\"kind\":\"state\"}",
            &manifest,
        )
        .unwrap_err()
        .to_string();
        assert_eq!(
            error,
            "runtime state request app mismatch: expected deployments, got other-app"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_request_resolves_commands_relative_to_manifest() {
        let root = runtime_fixture_root("run-request-manifest-relative");
        let manifest = root.join("runtime.manifest.json");
        write_or_replace(
            &manifest,
            "{\n  \"version\": \"theurgy-runtime-manifest/v1\",\n  \"app\": \"deployments\",\n  \"productIr\": \"app-blueprint/product.ir.json\",\n  \"runtime\": {\n    \"stateCommand\": [\"runtime-fixture\", \"state\"],\n    \"statusCommand\": [\"runtime-fixture\", \"status\"],\n    \"operationStatusCommand\": [\"runtime-fixture\", \"operation-status\"],\n    \"actionCommand\": [\"runtime-fixture\", \"action\"],\n    \"historyCommand\": [\"runtime-fixture\", \"history\"],\n    \"protocol\": \"deployments-runtime/v1\"\n  }\n}\n",
        )
        .unwrap();

        let output = run_request_output(
            "{\"schema\":\"theurgy-runtime-state-request/v1\",\"protocol\":\"theurgy-runtime-action/v1\",\"app\":\"deployments\",\"kind\":\"state\"}",
            &manifest,
        )
        .unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(
            value.get("schema").and_then(Value::as_str),
            Some("theurgy-state-snapshot/v1")
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn command_run_request_dispatches_request_file() {
        let root = runtime_fixture_root("run-request-command");
        let request = root.join("request.json");
        write_or_replace(
            &request,
            "{\"schema\":\"theurgy-runtime-status-request/v1\",\"protocol\":\"theurgy-runtime-action/v1\",\"app\":\"deployments\",\"kind\":\"status\"}",
        )
        .unwrap();
        command_run_request(&[
            request.display().to_string(),
            "--manifest".to_string(),
            root.join("runtime.manifest.json").display().to_string(),
        ])
        .unwrap();

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
        let product = "{\n  \"version\": \"theurgy-product-ir/v1\",\n  \"format\": \"json\",\n  \"app\": {\"id\": \"deployments\", \"name\": \"Deployments\", \"targets\": [\"ios\"], \"capabilities\": [\"native-mobile\"]},\n  \"actions\": [{\"id\": \"refresh_state\", \"label\": \"Refresh\", \"input\": {}, \"output\": {}, \"effect\": \"read\", \"failure\": {}, \"safe\": true, \"mutating\": false, \"longRunning\": false, \"privileged\": false}],\n  \"state\": {\"snapshotSchema\": \"deployments-state/v1\"},\n  \"releaseTargets\": [{\"id\": \"ios-app\", \"target\": \"ios\", \"surface\": \"mobile\", \"artifact\": \"generated/mobile/ios\"}]\n}".to_string();
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
        assert!(root.join("runtime.manifest.json").exists());
        assert!(root.join("app-blueprint/product.ir.json").exists());
        assert!(root.join("app-blueprint/runtime.manifest.json").exists());
        assert!(root.join("app-blueprint/surface.ir.json").exists());
        let staged_manifest =
            fs::read_to_string(root.join("app-blueprint/runtime.manifest.json")).unwrap();
        assert!(staged_manifest.contains("\"version\": \"theurgy-runtime-manifest/v1\""));
        let staged_product =
            fs::read_to_string(root.join("app-blueprint/product.ir.json")).unwrap();
        assert!(staged_product.contains("\"version\": \"theurgy-product-ir/v1\""));
        let staged_surface =
            fs::read_to_string(root.join("app-blueprint/surface.ir.json")).unwrap();
        assert!(staged_surface.contains("\"version\": \"theurgy-desktop-surface-ir/v1\""));
        let runtime = fs::read_to_string(root.join("theurgy-runtime.json")).unwrap();
        let runtime_json: Value = serde_json::from_str(&runtime).unwrap();
        let generated = validate_generated_runtime(&runtime).unwrap();
        assert_eq!(generated.app_id, "deployments");
        assert_eq!(generated.target, "linux");
        assert_eq!(generated.state_snapshot_schema, "deployments-state/v1");
        assert_eq!(
            runtime_json.get("productIr").and_then(Value::as_str),
            Some("app-blueprint/product.ir.json")
        );
        assert_eq!(
            runtime_json.get("runtimeManifest").and_then(Value::as_str),
            Some("app-blueprint/runtime.manifest.json")
        );
        assert_eq!(
            runtime_json.get("sourceSurfaceIr").and_then(Value::as_str),
            Some("app-blueprint/surface.ir.json")
        );
        assert_eq!(
            runtime_json
                .get("productStateSnapshotSchema")
                .and_then(Value::as_str),
            Some("deployments-state/v1")
        );
        assert_eq!(generated.persistence_truth, "file-first");
        assert_eq!(generated.adapter_runtime_transport, "local-process-json");
        assert_eq!(
            generated.runtime_state_request_schema,
            "theurgy-runtime-state-request/v1"
        );
        assert_eq!(
            generated.runtime_status_request_schema,
            "theurgy-runtime-status-request/v1"
        );
        assert_eq!(
            generated.runtime_subscribe_status_request_schema,
            "theurgy-runtime-subscribe-status-request/v1"
        );
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
            generated.operation_status_request_schema,
            "theurgy-operation-status-request/v1"
        );
        assert_eq!(
            generated.operation_history_request_schema,
            "theurgy-operation-history-request/v1"
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
            generated.request_command.as_deref(),
            Some(&["theurgy-runtime".to_string(), "run-request".to_string()][..])
        );
        assert_eq!(
            generated.request_command_manifest.as_deref(),
            Some("app-blueprint/runtime.manifest.json")
        );
        assert_eq!(
            runtime_json.get("requestCommand").unwrap(),
            &serde_json::json!(["theurgy-runtime", "run-request"])
        );
        assert_eq!(
            runtime_json
                .get("requestCommandManifest")
                .and_then(Value::as_str),
            Some("app-blueprint/runtime.manifest.json")
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
                .get("runtimeStateRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-state-request/v1")
        );
        assert_eq!(
            runtime_json
                .get("runtimeStatusRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-status-request/v1")
        );
        assert_eq!(
            runtime_json
                .get("runtimeSubscribeStatusRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-subscribe-status-request/v1")
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
                .get("operationStatusRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-operation-status-request/v1")
        );
        assert_eq!(
            runtime_json
                .get("operationHistoryRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-operation-history-request/v1")
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
            runtime_json.get("surfaceCapabilities").unwrap(),
            &serde_json::json!([])
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
            runtime_json.get("productStateProjections").unwrap(),
            &serde_json::json!(["deployments"])
        );
        assert_eq!(
            runtime_json.get("productDomainObjectContracts").unwrap(),
            &serde_json::json!([
                {"id": "server", "label": "Server"},
                {"id": "deployment", "label": "Deployment"}
            ])
        );
        assert_eq!(
            runtime_json.get("productPersistenceRoots").unwrap(),
            &serde_json::json!(["headquarters-workspace"])
        );
        assert_eq!(
            runtime_json.get("productPersistenceRootContracts").unwrap(),
            &serde_json::json!([{
                "id": "headquarters-workspace",
                "kind": "xdg-state",
                "path": "${XDG_STATE_HOME:-$HOME/.local/state}/wizardry-apps/headquarters/workspaces/<workspace-key>"
            }])
        );
        assert_eq!(
            runtime_json.get("productBackgroundJobs").unwrap(),
            &serde_json::json!(["server-queue"])
        );
        assert_eq!(
            runtime_json
                .pointer("/productBackgroundJobContracts/0/id")
                .and_then(Value::as_str),
            Some("server-queue")
        );
        assert_eq!(
            runtime_json
                .pointer("/productBackgroundJobContracts/0/label")
                .and_then(Value::as_str),
            Some("Server Queue")
        );
        assert_eq!(
            runtime_json
                .pointer("/productBackgroundJobContracts/0/command")
                .unwrap(),
            &serde_json::json!(["deployments-daemon"])
        );
        assert_eq!(
            runtime_json
                .pointer("/productBackgroundJobContracts/0/state")
                .and_then(Value::as_str),
            Some("server.queue_mode")
        );
        assert_eq!(
            runtime_json.get("productReleaseTargets").unwrap(),
            &serde_json::json!(["macos-app", "linux-app"])
        );
        assert_eq!(
            runtime_json.get("productReleaseTargetContracts").unwrap(),
            &serde_json::json!([
                {"id": "macos-app", "target": "macos", "surface": "desktop", "artifact": "generated/macos"},
                {"id": "linux-app", "target": "linux", "surface": "desktop", "artifact": "generated/linux"}
            ])
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
        assert!(main_c.contains("static char *load_operation_status(void)"));
        assert!(main_c.contains("\"runtime-operation-status\", \"default\", NULL"));
        assert!(main_c.contains(
            "GtkWidget *operation_button = gtk_button_new_with_label(\"Operation Status\")"
        ));
        assert!(main_c.contains("G_CALLBACK(refresh_operation_status)"));
        assert!(main_c.contains("static char *load_operation_history(void)"));
        assert!(main_c.contains("\"default\", \"20\", NULL"));
        assert!(
            main_c.contains("GtkWidget *history_button = gtk_button_new_with_label(\"History\")")
        );
        assert!(main_c.contains("G_CALLBACK(refresh_operation_history)"));
        assert!(main_c.contains("g_subprocess_newv"));
        assert!(main_c.contains("json-glib/json-glib.h"));
        assert!(main_c.contains("surface_action_contracts_json"));
        assert!(main_c.contains("\\\"inputShape\\\""));
        assert!(main_c.contains("\\\"deployment\\\":\\\"string\\\""));
        assert!(main_c.contains("Surface action contracts: refresh_state, publish_changes"));
        assert!(main_c
            .contains("contract_string_array_summary(runtime_metadata, \"surfaceCapabilities\")"));
        assert!(main_c.contains("Surface capabilities: %s"));
        assert!(main_c.contains("static char *load_contract_text(const char *name)"));
        assert!(main_c.contains("static char *contract_string_array_summary"));
        assert!(main_c.contains("static char *contract_object_array_summary"));
        assert!(main_c.contains(
            "contract_string_array_summary(runtime_metadata, \"productStateProjections\")"
        ));
        assert!(main_c.contains(
            "contract_object_array_summary(runtime_metadata, \"productDomainObjectContracts\""
        ));
        assert!(main_c.contains(
            "contract_object_array_summary(runtime_metadata, \"productPersistenceRootContracts\""
        ));
        assert!(main_c.contains(
            "contract_object_array_summary(runtime_metadata, \"productReleaseTargetContracts\""
        ));
        assert!(main_c.contains("Product state projections: %s"));
        assert!(main_c.contains("Product domain objects: %s"));
        assert!(main_c.contains("Product persistence roots: %s"));
        assert!(main_c.contains("Product release targets: %s"));
        assert!(main_c.contains("static char *run_default_action(void)"));
        assert!(main_c.contains("static char *run_runtime_request(const char *request)"));
        assert!(main_c.contains("\"run-request\", request_path, \"--manifest\", manifest"));
        assert!(main_c.contains("\"schema\":\"theurgy-runtime-action-request/v1\""));
        assert!(main_c.contains("\"action\":\"refresh_state\",\"params\":%s"));
        assert!(!main_c.contains("\"runtime-action\", \"refresh_state\", \"{}\", NULL"));
        assert!(main_c.contains("GtkWidget *action_button = gtk_button_new_with_label(\"Action\")"));
        assert!(main_c.contains("G_CALLBACK(run_action)"));
        let linux_action_root = test_root("compile-linux-action-defaults");
        let publish_product = sample_product();
        let publish_summary = validate_product_ir(&publish_product).unwrap();
        let publish_runtime = sample_full_runtime_contract();
        let publish_surface = surface_with_actions(
            &project_surface(&publish_product, "linux").unwrap(),
            &["publish_changes"],
        );
        compile_native_with_contract(
            &publish_summary,
            &publish_surface,
            &publish_runtime,
            "linux",
            &linux_action_root,
            false,
            &[],
        )
        .unwrap();
        let publish_main_c = fs::read_to_string(linux_action_root.join("src/main.c")).unwrap();
        assert!(publish_main_c.contains("\"action\":\"publish_changes\",\"params\":%s"));
        assert!(publish_main_c.contains("\"{\\\"deployment\\\":\\\"\\\"}\""));
        assert!(!publish_main_c.contains(
            "\"runtime-action\", \"publish_changes\", \"{\\\"deployment\\\":\\\"\\\"}\", NULL"
        ));
        fs::remove_dir_all(linux_action_root).unwrap();
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
    fn generated_runtime_validation_rejects_background_job_contract_drift() {
        let root = test_root("generated-runtime-background-job-drift");
        compile_native(&sample_product(), "linux", &root).unwrap();
        let mut runtime_json: Value =
            serde_json::from_str(&fs::read_to_string(root.join("theurgy-runtime.json")).unwrap())
                .unwrap();
        *runtime_json
            .pointer_mut("/productBackgroundJobContracts/0/id")
            .unwrap() = Value::String("not_declared".to_string());
        let runtime = serde_json::to_string(&runtime_json).unwrap();
        let error = validate_generated_runtime(&runtime)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("productBackgroundJobContracts order must match productBackgroundJobs")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generated_runtime_validation_rejects_domain_object_contract_drift() {
        let root = test_root("generated-runtime-domain-object-drift");
        compile_native(&sample_product(), "linux", &root).unwrap();
        let mut runtime_json: Value =
            serde_json::from_str(&fs::read_to_string(root.join("theurgy-runtime.json")).unwrap())
                .unwrap();
        *runtime_json
            .pointer_mut("/productDomainObjectContracts/0/id")
            .unwrap() = Value::String("not_declared".to_string());
        let runtime = serde_json::to_string(&runtime_json).unwrap();
        let error = validate_generated_runtime(&runtime)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("productDomainObjectContracts order must match productDomainObjects")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generated_runtime_validation_rejects_persistence_root_contract_drift() {
        let root = test_root("generated-runtime-persistence-root-drift");
        compile_native(&sample_product(), "linux", &root).unwrap();
        let mut runtime_json: Value =
            serde_json::from_str(&fs::read_to_string(root.join("theurgy-runtime.json")).unwrap())
                .unwrap();
        *runtime_json
            .pointer_mut("/productPersistenceRootContracts/0/id")
            .unwrap() = Value::String("not_declared".to_string());
        let runtime = serde_json::to_string(&runtime_json).unwrap();
        let error = validate_generated_runtime(&runtime)
            .unwrap_err()
            .to_string();
        assert!(error
            .contains("productPersistenceRootContracts order must match productPersistenceRoots"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generated_runtime_validation_rejects_release_target_contract_drift() {
        let root = test_root("generated-runtime-release-target-drift");
        compile_native(&sample_product(), "linux", &root).unwrap();
        let mut runtime_json: Value =
            serde_json::from_str(&fs::read_to_string(root.join("theurgy-runtime.json")).unwrap())
                .unwrap();
        *runtime_json
            .pointer_mut("/productReleaseTargetContracts/0/id")
            .unwrap() = Value::String("not_declared".to_string());
        let runtime = serde_json::to_string(&runtime_json).unwrap();
        let error = validate_generated_runtime(&runtime)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("productReleaseTargetContracts order must match productReleaseTargets")
        );
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
        let staged_manifest = fs::read_to_string(out.join("runtime.manifest.json")).unwrap();
        assert!(staged_manifest.contains("\"version\": \"theurgy-runtime-manifest/v1\""));
        let staged_product = fs::read_to_string(out.join("app-blueprint/product.ir.json")).unwrap();
        assert!(staged_product.contains("theurgy-product-ir/v1"));
        let staged_surface =
            fs::read_to_string(out.join("app-blueprint/desktop.surface.ir.json")).unwrap();
        assert!(staged_surface.contains("theurgy-desktop-surface-ir/v1"));
        let runtime_json: Value = serde_json::from_str(&runtime).unwrap();
        let generated = validate_generated_runtime(&runtime).unwrap();
        assert_eq!(generated.adapter_runtime_transport, "local-process-json");
        assert_eq!(
            generated.runtime_state_request_schema,
            "theurgy-runtime-state-request/v1"
        );
        assert_eq!(
            generated.runtime_status_request_schema,
            "theurgy-runtime-status-request/v1"
        );
        assert_eq!(
            generated.runtime_subscribe_status_request_schema,
            "theurgy-runtime-subscribe-status-request/v1"
        );
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
            generated.request_command.as_deref(),
            Some(&["theurgy-runtime".to_string(), "run-request".to_string()][..])
        );
        assert_eq!(
            generated.request_command_manifest.as_deref(),
            Some("app-blueprint/runtime.manifest.json")
        );
        assert_eq!(
            runtime_json.get("requestCommand").unwrap(),
            &serde_json::json!(["theurgy-runtime", "run-request"])
        );
        assert_eq!(
            runtime_json
                .get("requestCommandManifest")
                .and_then(Value::as_str),
            Some("app-blueprint/runtime.manifest.json")
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
                .get("runtimeStateRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-state-request/v1")
        );
        assert_eq!(
            runtime_json
                .get("runtimeStatusRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-status-request/v1")
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
            runtime_json.get("surfaceCapabilities").unwrap(),
            &serde_json::json!([])
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
        let staged_manifest = fs::read_to_string(out.join("runtime.manifest.json")).unwrap();
        assert!(staged_manifest.contains("\"version\": \"theurgy-runtime-manifest/v1\""));
        assert!(
            fs::read_to_string(out.join("app-blueprint/product.ir.json"))
                .unwrap()
                .contains("theurgy-product-ir/v1")
        );
        assert!(
            fs::read_to_string(out.join("app-blueprint/desktop.surface.ir.json"))
                .unwrap()
                .contains("theurgy-desktop-surface-ir/v1")
        );
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
        compile_native_with_contract(&summary, &surface, &runtime, "macos", &root, false, &[])
            .unwrap();

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
        assert!(
            swift.contains("func defaultParamsJson(for action: ProductActionContract) -> String")
        );
        assert!(swift
            .contains("func defaultParams(for action: ProductActionContract) -> [String: Any]"));
        assert!(swift.contains("params[key] = defaultParamValue(for: descriptor)"));
        assert!(swift.contains("struct ProductActionContract"));
        assert!(swift.contains("let actionContracts = [ProductActionContract"));
        assert!(
            swift.contains("let runtimeRequestCommand = [\"theurgy-runtime\", \"run-request\"]")
        );
        assert!(swift.contains("let runtimeRequestManifest = \"runtime.manifest.json\""));
        assert!(swift.contains("func runRuntimeRequest(_ request: [String: Any]) -> String"));
        assert!(swift.contains("\"schema\": \"theurgy-runtime-action-request/v1\""));
        assert!(swift.contains("\"schema\": \"theurgy-runtime-state-request/v1\""));
        assert!(swift.contains(
            "runtimeRequestCommand + [requestURL.path, \"--manifest\", manifestURL.path]"
        ));
        assert!(!swift
            .contains("func command(for action: ProductActionContract, json: String) -> [String]"));
        assert!(!swift.contains("runtimeActionCommand + [action.id, json]"));
        assert!(swift.contains("ForEach(actionContracts, id: \\.id)"));
        assert!(swift.contains("runRuntimeAction(action)"));
        assert!(!swift.contains("runRuntimeCommand(runtimeSubscribeStatusCommand)"));
        assert!(!swift.contains("runRuntimeCommand(runtimeOperationStatusCommand + [\"default\"])"));
        assert!(!swift.contains("command(for: action, json: defaultParamsJson(for: action))"));
        assert!(swift.contains("inputShape: [\"deployment\": \"string\"]"));
        assert!(swift.contains("outputShape: [\"params\": \"object\"]"));
        assert!(swift.contains("let surfaceCapabilities = []"));
        assert!(swift.contains("Surface capabilities: \\(surfaceCapabilities.joined"));
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
        compile_native_with_contract(
            &summary,
            &ios_surface,
            &runtime,
            "ios",
            &ios_root,
            false,
            &[],
        )
        .unwrap();
        compile_native_with_contract(
            &summary,
            &android_surface,
            &runtime,
            "android",
            &android_root,
            false,
            &[],
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
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"runtimeStateRequestSchema\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"runtimeStatusRequestSchema\")"));
        assert!(ios.contains(
            "runtimeString(runtimeMetadata, key: \"runtimeSubscribeStatusRequestSchema\")"
        ));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"runtimeStatusSchema\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"runtimeActionRequestSchema\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"runtimeActionResultSchema\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"operationStatusSchema\")"));
        assert!(ios.contains("runtimeString(runtimeMetadata, key: \"operationHistorySchema\")"));
        assert!(ios.contains("runtimeStringArray(runtimeMetadata, key: \"surfaceActions\")"));
        assert!(ios.contains("runtimeStringArray(runtimeMetadata, key: \"surfaceCapabilities\")"));
        assert!(
            ios.contains("runtimeStringArray(runtimeMetadata, key: \"productStateProjections\")")
        );
        assert!(ios.contains(
            "runtimeObjectSummaries(runtimeMetadata, key: \"productDomainObjectContracts\", fields: [\"label\", \"source\"])"
        ));
        assert!(ios.contains(
            "runtimeObjectSummaries(runtimeMetadata, key: \"productPersistenceRootContracts\", fields: [\"kind\", \"path\"])"
        ));
        assert!(ios.contains(
            "runtimeObjectSummaries(runtimeMetadata, key: \"productReleaseTargetContracts\", fields: [\"target\", \"surface\", \"artifact\"])"
        ));
        assert!(ios.contains("func surfaceScreenSummaries(_ json: String) -> [String]"));
        assert!(ios.contains("func runtimeObjectSummaries(_ json: String, key: String, fields: [String]) -> [String]"));
        assert!(ios.contains("contractObject(json)?[\"screens\"] as? [[String: Any]]"));
        assert!(ios.contains(
            "var surfaceSchema: String { runtimeString(surfaceMetadata, key: \"version\") }"
        ));
        assert!(ios.contains(
            "var surfaceTarget: String { runtimeString(surfaceMetadata, key: \"target\") }"
        ));
        assert!(ios
            .contains("var surfaceScreens: [String] { surfaceScreenSummaries(surfaceMetadata) }"));
        assert!(ios.contains("Runtime app: \\(contract.runtimeApp)"));
        assert!(ios.contains("Runtime target: \\(contract.runtimeTarget)"));
        assert!(ios.contains("Runtime transport: \\(contract.runtimeTransport)"));
        assert!(
            ios.contains("Runtime state request schema: \\(contract.runtimeStateRequestSchema)")
        );
        assert!(
            ios.contains("Runtime status request schema: \\(contract.runtimeStatusRequestSchema)")
        );
        assert!(ios.contains(
            "Runtime subscribe status request schema: \\(contract.runtimeSubscribeStatusRequestSchema)"
        ));
        assert!(ios.contains("Runtime status schema: \\(contract.runtimeStatusSchema)"));
        assert!(
            ios.contains("Runtime action request schema: \\(contract.runtimeActionRequestSchema)")
        );
        assert!(
            ios.contains("Runtime action result schema: \\(contract.runtimeActionResultSchema)")
        );
        assert!(ios.contains("Operation status schema: \\(contract.operationStatusSchema)"));
        assert!(ios.contains("Operation history schema: \\(contract.operationHistorySchema)"));
        assert!(ios.contains("Surface capabilities: \\(contract.surfaceCapabilities.joined"));
        assert!(ios.contains("Runtime surface actions: \\(contract.runtimeSurfaceActions.joined"));
        assert!(
            ios.contains("Product state projections: \\(contract.productStateProjections.joined")
        );
        assert!(ios.contains("Surface schema: \\(contract.surfaceSchema)"));
        assert!(ios.contains("Surface target: \\(contract.surfaceTarget)"));
        assert!(ios.contains("Surface screens: \\(contract.surfaceScreens.joined"));
        assert!(ios.contains("Section(\"Product\")"));
        assert!(ios.contains("Domain objects: \\(contract.productDomainObjects.joined"));
        assert!(ios.contains("Persistence roots: \\(contract.productPersistenceRoots.joined"));
        assert!(ios.contains("Release targets: \\(contract.productReleaseTargets.joined"));
        assert!(ios.contains("Section(\"Mobile Workflow\")"));
        assert!(ios.contains("Text(\"status-overview\")"));
        assert!(ios.contains("Text(\"focused-action-detail\")"));
        assert!(ios.contains("struct ProductActionContract"));
        assert!(ios.contains("let actionContracts = [ProductActionContract"));
        assert!(ios.contains("protocol MobileRuntimeRequestBroker"));
        assert!(ios.contains("struct ExternalJsonAbiPreviewBroker: MobileRuntimeRequestBroker"));
        assert!(ios.contains("struct MobileRuntimeHandoff"));
        assert!(ios.contains("Section(\"Request Handoff\")"));
        assert!(ios.contains("Button(\"Hydrate State\")"));
        assert!(ios.contains("Button(\"Operation Status\")"));
        assert!(
            ios.contains("broker.submit(kind: \"state\", requestJson: contract.stateEnvelope())")
        );
        assert!(ios.contains(
            "broker.submit(kind: \"operation-status\", requestJson: contract.operationStatusEnvelope"
        ));
        assert!(
            ios.contains("broker.submit(kind: \"action\", requestJson: contract.actionEnvelope")
        );
        assert!(!ios.contains("func command(for action: ProductActionContract"));
        assert!(!ios.contains("actionCommand + [action.id, json]"));
        assert!(ios.contains("func stateEnvelope() -> String"));
        assert!(ios.contains("\"schema\": runtimeStateRequestSchema"));
        assert!(ios.contains("\"kind\": \"state\""));
        assert!(ios.contains("func statusEnvelope() -> String"));
        assert!(ios.contains("\"schema\": runtimeStatusRequestSchema"));
        assert!(ios.contains("\"kind\": \"status\""));
        assert!(ios.contains("func subscribeStatusEnvelope() -> String"));
        assert!(ios.contains("\"schema\": runtimeSubscribeStatusRequestSchema"));
        assert!(ios.contains("\"kind\": \"subscribe-status\""));
        assert!(ios.contains("Text(contract.stateEnvelope())"));
        assert!(ios.contains("Text(contract.statusEnvelope())"));
        assert!(ios.contains("Text(contract.subscribeStatusEnvelope())"));
        assert!(ios.contains(
            "func actionEnvelope(for action: ProductActionContract, params: [String: Any]) -> String"
        ));
        assert!(ios.contains("\"schema\": runtimeActionRequestSchema"));
        assert!(ios.contains("\"protocol\": protocolName"));
        assert!(ios.contains("\"app\": runtimeApp"));
        assert!(ios.contains("\"action\": action.id"));
        assert!(ios.contains("\"params\": params"));
        assert!(ios.contains("JSONSerialization.data(withJSONObject: envelope"));
        assert!(ios.contains("func defaultParamsJson(for action: ProductActionContract) -> String"));
        assert!(
            ios.contains("func defaultParams(for action: ProductActionContract) -> [String: Any]")
        );
        assert!(ios.contains(
            "contract.actionEnvelope(for: action, params: contract.defaultParams(for: action))"
        ));
        assert!(ios.contains("id: \"publish_changes\""));
        assert!(ios.contains("inputKeys: [\"deployment\"]"));
        assert!(ios.contains("outputKeys: [\"params\"]"));
        assert!(ios.contains("failureKeys: []"));
        assert!(ios.contains("inputShape: [\"deployment\": \"string\"]"));
        assert!(ios.contains("outputShape: [\"params\": \"object\"]"));
        assert!(!ios.contains("id: \"refresh_state\""));
        let ios_package = fs::read_to_string(ios_root.join("Package.swift")).unwrap();
        assert!(ios_package.contains("platforms: [.iOS(.v16), .macOS(.v13)]"));
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
        assert_eq!(ios_generated.request_command, None);
        assert_eq!(ios_generated.request_command_manifest, None);
        assert_eq!(
            ios_generated.runtime_state_request_schema,
            "theurgy-runtime-state-request/v1"
        );
        assert_eq!(
            ios_generated.runtime_status_request_schema,
            "theurgy-runtime-status-request/v1"
        );
        assert_eq!(
            ios_generated.runtime_subscribe_status_request_schema,
            "theurgy-runtime-subscribe-status-request/v1"
        );
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
                .get("runtimeStateRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-state-request/v1")
        );
        assert_eq!(
            ios_runtime
                .get("runtimeStatusRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-status-request/v1")
        );
        assert_eq!(
            ios_runtime
                .get("runtimeSubscribeStatusRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-subscribe-status-request/v1")
        );
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
        let mut screen_action_drift = ios_runtime.clone();
        screen_action_drift["surfaceScreenContracts"][0]["actions"][0] =
            Value::String("not_in_mobile_surface".to_string());
        let error = product_runtime::validate_generated_runtime_value(&screen_action_drift)
            .unwrap_err()
            .to_string();
        assert_eq!(
            error,
            "generated runtime mobile screen action not declared in surfaceActions: not_in_mobile_surface"
        );
        let mut invalid_screen_action = ios_runtime.clone();
        invalid_screen_action["surfaceScreenContracts"][0]["actions"][0] =
            Value::String("Not Stable".to_string());
        let error = product_runtime::validate_generated_runtime_value(&invalid_screen_action)
            .unwrap_err()
            .to_string();
        assert_eq!(
            error,
            "generated runtime mobile screen contract action must be a stable action id"
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
        assert!(android.contains("jsonString(runtimeMetadata, \"runtimeStateRequestSchema\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"runtimeStatusRequestSchema\")"));
        assert!(android
            .contains("jsonString(runtimeMetadata, \"runtimeSubscribeStatusRequestSchema\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"runtimeStatusSchema\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"runtimeActionRequestSchema\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"runtimeActionResultSchema\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"operationStatusSchema\")"));
        assert!(android.contains("jsonString(runtimeMetadata, \"operationHistorySchema\")"));
        assert!(android.contains("jsonStringArray(runtimeMetadata, \"surfaceCapabilities\")"));
        assert!(android.contains("jsonStringArray(runtimeMetadata, \"surfaceActions\")"));
        assert!(android.contains("jsonStringArray(runtimeMetadata, \"productStateProjections\")"));
        assert!(android.contains(
            "jsonObjectArraySummary(runtimeMetadata, \"productDomainObjectContracts\", new String[] {\"label\", \"source\"})"
        ));
        assert!(android.contains(
            "jsonObjectArraySummary(runtimeMetadata, \"productPersistenceRootContracts\", new String[] {\"kind\", \"path\"})"
        ));
        assert!(android.contains(
            "jsonObjectArraySummary(runtimeMetadata, \"productReleaseTargetContracts\", new String[] {\"target\", \"surface\", \"artifact\"})"
        ));
        assert!(android.contains("private static String surfaceScreens(String json)"));
        assert!(android.contains(
            "private static String jsonObjectArraySummary(String json, String key, String[] fields)"
        ));
        assert!(android.contains("new JSONObject(json).optJSONArray(\"screens\")"));
        assert!(android.contains("screen.optString(\"title\", id)"));
        assert!(android.contains("Runtime app: "));
        assert!(android.contains("Runtime target: "));
        assert!(android.contains("Runtime transport: "));
        assert!(android.contains("Runtime state request schema: "));
        assert!(android.contains("Runtime status request schema: "));
        assert!(android.contains("Runtime subscribe status request schema: "));
        assert!(android.contains("Runtime status schema: "));
        assert!(android.contains("Runtime action request schema: "));
        assert!(android.contains("Runtime action result schema: "));
        assert!(android.contains("Operation status schema: "));
        assert!(android.contains("Operation history schema: "));
        assert!(android.contains("Surface capabilities: "));
        assert!(android.contains("Runtime surface actions: "));
        assert!(android.contains("Product state projections: "));
        assert!(android.contains("Product domain objects: "));
        assert!(android.contains("Product persistence roots: "));
        assert!(android.contains("Product release targets: "));
        assert!(android.contains("Surface schema: "));
        assert!(android.contains("Surface target: "));
        assert!(android.contains("Surface screens: "));
        assert!(android.contains("Mobile workflow: status-overview, focused-action-detail"));
        assert!(android.contains("private static final ProductActionContract[] ACTION_CONTRACTS"));
        assert!(android.contains("private interface MobileRuntimeRequestBroker"));
        assert!(android.contains(
            "private static final class ExternalJsonAbiPreviewBroker implements MobileRuntimeRequestBroker"
        ));
        assert!(android
            .contains("MobileRuntimeRequestBroker broker = new ExternalJsonAbiPreviewBroker();"));
        assert!(android.contains("Request handoff transport: "));
        assert!(android.contains("State handoff: "));
        assert!(android.contains("broker.submit(\"state\", stateEnvelope"));
        assert!(android.contains("broker.submit(\"action\", actionEnvelope"));
        assert!(!android.contains("private static String[] commandFor"));
        assert!(!android.contains("command[ACTION_COMMAND.length] = action.id;"));
        assert!(android
            .contains("private static String stateEnvelope(String app, String requestSchema)"));
        assert!(android.contains("envelope.put(\"kind\", \"state\");"));
        assert!(android
            .contains("private static String statusEnvelope(String app, String requestSchema)"));
        assert!(android.contains("envelope.put(\"kind\", \"status\");"));
        assert!(android.contains(
            "private static String subscribeStatusEnvelope(String app, String requestSchema)"
        ));
        assert!(android.contains("envelope.put(\"kind\", \"subscribe-status\");"));
        assert!(android.contains("State handoff: "));
        assert!(android.contains("Status handoff: "));
        assert!(android.contains("Subscribe handoff: "));
        assert!(android.contains("Operation status handoff: "));
        assert!(android.contains("broker.submit(\"operation-status\", operationStatusEnvelope"));
        assert!(android.contains("History handoff: "));
        assert!(android.contains("broker.submit(\"operation-history\", operationHistoryEnvelope"));
        assert!(android.contains(
            "private static String actionEnvelope(String app, String requestSchema, ProductActionContract action, JSONObject params)"
        ));
        assert!(android.contains("envelope.put(\"schema\", requestSchema);"));
        assert!(android.contains("envelope.put(\"protocol\", PROTOCOL);"));
        assert!(android.contains("envelope.put(\"app\", app);"));
        assert!(android.contains("envelope.put(\"action\", action.id);"));
        assert!(android.contains("envelope.put(\"params\", params);"));
        assert!(android.contains("String runtimeApp = jsonString(runtimeMetadata, \"app\");"));
        assert!(android
            .contains("private static JSONObject defaultParams(ProductActionContract action)"));
        assert!(android.contains("params.put(shape[0], defaultParamValue(shape[1]));"));
        assert!(android.contains("JSONObject params = defaultParams(action);"));
        assert!(android
            .contains("actionEnvelope(runtimeApp, runtimeActionRequestSchema, action, params)"));
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
        assert_eq!(android_generated.request_command, None);
        assert_eq!(android_generated.request_command_manifest, None);
        assert_eq!(
            android_generated.runtime_state_request_schema,
            "theurgy-runtime-state-request/v1"
        );
        assert_eq!(
            android_generated.runtime_status_request_schema,
            "theurgy-runtime-status-request/v1"
        );
        assert_eq!(
            android_generated.runtime_subscribe_status_request_schema,
            "theurgy-runtime-subscribe-status-request/v1"
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
                .get("runtimeStateRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-state-request/v1")
        );
        assert_eq!(
            android_runtime
                .get("runtimeStatusRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-status-request/v1")
        );
        assert_eq!(
            android_runtime
                .get("runtimeSubscribeStatusRequestSchema")
                .and_then(Value::as_str),
            Some("theurgy-runtime-subscribe-status-request/v1")
        );
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
        "{\n  \"version\": \"theurgy-product-ir/v1\",\n  \"format\": \"json\",\n  \"app\": {\n    \"id\": \"deployments\",\n    \"name\": \"Deployments\",\n    \"targets\": [\"macos\", \"linux\"],\n    \"capabilities\": [\"native-desktop\", \"runtime-actions\"],\n    \"permissions\": [\"files\"]\n  },\n  \"domain\": {\n    \"objects\": [\n      {\"id\": \"server\", \"label\": \"Server\"},\n      {\"id\": \"deployment\", \"label\": \"Deployment\"}\n    ]\n  },\n  \"actions\": [\n    {\"id\": \"refresh_state\", \"label\": \"Refresh\", \"input\": {}, \"output\": {\"params\": \"object\"}, \"effect\": \"read\", \"failure\": {}, \"safe\": true, \"mutating\": false, \"longRunning\": false, \"privileged\": false},\n    {\"id\": \"publish_changes\", \"label\": \"Push to Production\", \"input\": {\"deployment\": \"string\"}, \"output\": {\"params\": \"object\"}, \"effect\": \"release\", \"failure\": {}, \"safe\": false, \"mutating\": true, \"longRunning\": true, \"privileged\": true}\n  ],\n  \"state\": {\n    \"snapshotSchema\": \"deployments-state/v1\",\n    \"projections\": [\"deployments\"],\n    \"roots\": [{\"id\": \"headquarters-workspace\", \"kind\": \"xdg-state\", \"path\": \"${XDG_STATE_HOME:-$HOME/.local/state}/wizardry-apps/headquarters/workspaces/<workspace-key>\"}]\n  },\n  \"backgroundJobs\": [\n    {\"id\": \"server-queue\", \"label\": \"Server Queue\", \"command\": [\"deployments-daemon\"], \"state\": \"server.queue_mode\"}\n  ],\n  \"releaseTargets\": [\n    {\"id\": \"macos-app\", \"target\": \"macos\", \"surface\": \"desktop\", \"artifact\": \"generated/macos\"},\n    {\"id\": \"linux-app\", \"target\": \"linux\", \"surface\": \"desktop\", \"artifact\": \"generated/linux\"}\n  ],\n  \"persistence\": {\n    \"truth\": \"file-first\"\n  },\n  \"audit\": {\n    \"operationHistory\": true,\n    \"cliParity\": true\n  }\n}".to_string()
    }

    fn sample_mobile_product() -> String {
        sample_product()
            .replace(
                "\"targets\": [\"macos\", \"linux\"]",
                "\"targets\": [\"ios\", \"android\"]",
            )
            .replace(
                "\"capabilities\": [\"native-desktop\", \"runtime-actions\"]",
                "\"capabilities\": [\"native-mobile\", \"runtime-actions\"]",
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
            compatibility: product_runtime::RuntimeCompatibility::shell_first_default(),
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
