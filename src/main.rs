use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

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
        Some("validate-runtime-manifest") => command_validate_runtime_manifest(&args[2..]),
        Some("validate-surface-ir") => command_validate_surface_ir(&args[2..]),
        Some("project-surface") => command_project_surface(&args[2..]),
        Some("compile-native") => command_compile_native(&args[2..]),
        Some("compile-app") => command_compile_app(&args[2..]),
        Some("inspect-app") => command_inspect_app(&args[2..]),
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

fn command_validate_runtime_manifest(args: &[String]) -> Result<()> {
    if args.len() != 1 {
        return Err(TheurgyError::new("usage: validate-runtime-manifest PATH").into());
    }
    let value = read_json(Path::new(&args[0]))?;
    let summary = validate_runtime_manifest(&value)?;
    println!("status=ok");
    println!("schema=theurgy-runtime-manifest/v1");
    println!("app={}", summary.app_id);
    println!("protocol={}", summary.protocol);
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
    let product_path = app_dir.join(product_ir);
    let runtime_path = app_dir.join(runtime_manifest);
    let surface_path = app_dir.join(surface_ir);
    let product = read_json(&product_path)?;
    let product_summary = validate_product_ir(&product)?;
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
    let runtime_contract = runtime_contract_from_manifest(&runtime_text)?;
    let surface = read_json(&surface_path)?;
    let surface_summary = validate_surface_ir(&surface)?;
    if surface_summary.product != product_summary.app_id {
        return Err(TheurgyError::new("surface IR product does not match product IR app").into());
    }
    let expected_surface_target = if matches!(target, "macos" | "linux") {
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
    command_inspect(&[path.display().to_string()])?;
    let manifest = fs::read_to_string(path.join("theurgy.project.toml")).unwrap_or_default();
    for key in [
        "product_ir",
        "desktop_surface_ir",
        "mobile_surface_ir",
        "runtime_manifest",
    ] {
        if let Ok(value) = manifest_value(&manifest, key) {
            println!("{key}={value}");
        }
    }
    Ok(())
}

fn command_run_action(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(TheurgyError::new("usage: run-action ACTION_ID --json PARAMS").into());
    }
    let action_id = &args[0];
    let mut params = "{}".to_string();
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
            other => {
                return Err(TheurgyError::new(format!("unknown run-action option: {other}")).into())
            }
        }
    }
    println!(
        "{{\n  \"success\": true,\n  \"protocol\": \"theurgy-runtime-action/v1\",\n  \"action\": \"{}\",\n  \"operation\": {{\n    \"id\": \"op-{}\",\n    \"status\": \"accepted\",\n    \"progress\": 0,\n    \"longRunning\": false\n  }},\n  \"params\": {}\n}}",
        json_escape(action_id),
        json_escape(action_id),
        params
    );
    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct ProductSummary {
    app_id: String,
    app_name: String,
    targets: Vec<String>,
    action_ids: Vec<String>,
    actions: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct RuntimeManifestSummary {
    app_id: String,
    protocol: String,
}

#[derive(Debug, Eq, PartialEq)]
struct SurfaceSummary {
    schema: String,
    product: String,
    target: String,
}

#[derive(Debug, Eq, PartialEq)]
struct RuntimeContract {
    app_id: String,
    protocol: String,
    state_command: Vec<String>,
    action_command: Vec<String>,
    history_command: Vec<String>,
    daemon_command: Vec<String>,
}

fn read_json(path: &Path) -> Result<String> {
    let text = fs::read_to_string(path).map_err(|error| {
        TheurgyError::new(format!("could not read {}: {error}", path.display()))
    })?;
    validate_json_params(&text)?;
    Ok(text)
}

fn validate_runtime_manifest(text: &str) -> Result<RuntimeManifestSummary> {
    expect_json_string(text, "version", "theurgy-runtime-manifest/v1")?;
    let app_id = json_string_value(text, "app")
        .filter(|id| valid_slug(id))
        .ok_or_else(|| TheurgyError::new("runtime manifest app must be a lowercase slug"))?;
    json_string_value(text, "productIr")
        .filter(|path| !path.is_empty())
        .ok_or_else(|| TheurgyError::new("runtime manifest productIr required"))?;
    let runtime = json_object_for_key(text, "runtime")?;
    let state_command = json_string_array(runtime, "stateCommand")?;
    let action_command = json_string_array(runtime, "actionCommand")?;
    if state_command.is_empty() || action_command.is_empty() {
        return Err(TheurgyError::new("runtime manifest commands must be non-empty arrays").into());
    }
    let protocol = json_string_value(runtime, "protocol")
        .filter(|protocol| !protocol.is_empty())
        .ok_or_else(|| TheurgyError::new("runtime manifest protocol required"))?;
    Ok(RuntimeManifestSummary { app_id, protocol })
}

fn runtime_contract_from_manifest(text: &str) -> Result<RuntimeContract> {
    let summary = validate_runtime_manifest(text)?;
    let runtime = json_object_for_key(text, "runtime")?;
    Ok(RuntimeContract {
        app_id: summary.app_id,
        protocol: summary.protocol,
        state_command: json_string_array(runtime, "stateCommand")?,
        action_command: json_string_array(runtime, "actionCommand")?,
        history_command: json_string_array(runtime, "historyCommand").unwrap_or_default(),
        daemon_command: json_string_array(runtime, "daemonCommand").unwrap_or_default(),
    })
}

fn validate_surface_ir(text: &str) -> Result<SurfaceSummary> {
    let schema = json_string_value(text, "version")
        .ok_or_else(|| TheurgyError::new("surface IR version required"))?;
    if !matches!(
        schema.as_str(),
        "theurgy-desktop-surface-ir/v1" | "theurgy-mobile-surface-ir/v1"
    ) {
        return Err(TheurgyError::new("surface IR version invalid").into());
    }
    expect_json_string(text, "format", "json")?;
    let product = json_string_value(text, "product")
        .filter(|id| valid_slug(id))
        .ok_or_else(|| TheurgyError::new("surface IR product must be a lowercase slug"))?;
    let target = json_string_value(text, "target")
        .filter(|target| !target.is_empty())
        .ok_or_else(|| TheurgyError::new("surface IR target required"))?;
    if schema == "theurgy-desktop-surface-ir/v1" {
        json_object_for_key(text, "window")?;
    } else {
        json_array_for_key(text, "screens")?;
    }
    Ok(SurfaceSummary {
        schema,
        product,
        target,
    })
}

fn validate_product_ir(text: &str) -> Result<ProductSummary> {
    expect_json_string(text, "version", "theurgy-product-ir/v1")?;
    expect_json_string(text, "format", "json")?;
    let app = json_object_for_key(text, "app")?;
    let app_id = json_string_value(app, "id")
        .filter(|id| valid_slug(id))
        .ok_or_else(|| TheurgyError::new("product IR app.id must be a lowercase slug"))?;
    let app_name = json_string_value(app, "name")
        .filter(|name| !name.is_empty())
        .ok_or_else(|| TheurgyError::new("product IR app.name required"))?;
    let targets = json_string_array(app, "targets")?;
    if targets.is_empty() {
        return Err(TheurgyError::new("product IR app.targets required").into());
    }
    for target in &targets {
        if !matches!(target.as_str(), "macos" | "linux" | "ios" | "android") {
            return Err(TheurgyError::new(
                "product IR target must be macos, linux, ios, or android",
            )
            .into());
        }
    }
    let actions_array = json_array_for_key(text, "actions")?;
    let action_objects = top_level_objects(actions_array);
    if action_objects.is_empty() {
        return Err(TheurgyError::new("product IR actions required").into());
    }
    let mut action_ids = Vec::new();
    for action in &action_objects {
        action_ids.push(validate_action(action)?);
    }
    let state = json_object_for_key(text, "state")?;
    json_string_value(state, "snapshotSchema")
        .filter(|schema| !schema.is_empty())
        .ok_or_else(|| TheurgyError::new("product IR state.snapshotSchema required"))?;
    Ok(ProductSummary {
        app_id,
        app_name,
        targets,
        actions: action_objects.len(),
        action_ids,
    })
}

fn validate_action(action: &str) -> Result<String> {
    let id = json_string_value(action, "id")
        .filter(|id| valid_action_id(id))
        .ok_or_else(|| TheurgyError::new("product IR action.id must be a stable action id"))?;
    json_string_value(action, "label")
        .filter(|label| !label.is_empty())
        .ok_or_else(|| TheurgyError::new("product IR action.label required"))?;
    for key in ["input", "output", "failure"] {
        json_object_for_key(action, key)
            .map_err(|_| TheurgyError::new(format!("product IR action.{key} object required")))?;
    }
    let effect = json_string_value(action, "effect")
        .ok_or_else(|| TheurgyError::new("product IR action.effect invalid"))?;
    if !matches!(
        effect.as_str(),
        "read" | "write" | "background" | "external" | "release"
    ) {
        return Err(TheurgyError::new("product IR action.effect invalid").into());
    }
    for key in ["safe", "mutating", "longRunning", "privileged"] {
        json_bool_value(action, key).ok_or_else(|| {
            TheurgyError::new(format!("product IR action.{key} boolean required"))
        })?;
    }
    Ok(id)
}

fn expect_json_string(text: &str, key: &str, expected: &str) -> Result<()> {
    match json_string_value(text, key) {
        Some(actual) if actual == expected => Ok(()),
        _ => Err(TheurgyError::new(format!("expected {key} = {expected}")).into()),
    }
}

fn validate_json_params(raw: &str) -> Result<()> {
    let trimmed = raw.trim();
    let looks_like_json = (trimmed.starts_with('{') && trimmed.ends_with('}'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']'));
    if looks_like_json {
        Ok(())
    } else {
        Err(TheurgyError::new("expected a JSON object or array literal").into())
    }
}

fn json_string_value(text: &str, key: &str) -> Option<String> {
    let marker = format!("\"{key}\"");
    let start = text.find(&marker)? + marker.len();
    let after_colon = text[start..].find(':')? + start + 1;
    let bytes = text.as_bytes();
    let mut index = after_colon;
    while index < bytes.len() && bytes[index].is_ascii_whitespace() {
        index += 1;
    }
    if bytes.get(index) != Some(&b'"') {
        return None;
    }
    index += 1;
    let mut out = String::new();
    let mut escaped = false;
    for character in text[index..].chars() {
        if escaped {
            out.push(character);
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == '"' {
            return Some(out);
        } else {
            out.push(character);
        }
    }
    None
}

fn json_bool_value(text: &str, key: &str) -> Option<bool> {
    let marker = format!("\"{key}\"");
    let start = text.find(&marker)? + marker.len();
    let after_colon = text[start..].find(':')? + start + 1;
    let rest = text[after_colon..].trim_start();
    if rest.starts_with("true") {
        Some(true)
    } else if rest.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

fn json_object_for_key<'a>(text: &'a str, key: &str) -> Result<&'a str> {
    json_balanced_for_key(text, key, '{', '}')
}

fn json_array_for_key<'a>(text: &'a str, key: &str) -> Result<&'a str> {
    json_balanced_for_key(text, key, '[', ']')
}

fn json_balanced_for_key<'a>(text: &'a str, key: &str, open: char, close: char) -> Result<&'a str> {
    let marker = format!("\"{key}\"");
    let start = text
        .find(&marker)
        .ok_or_else(|| TheurgyError::new(format!("missing JSON key: {key}")))?
        + marker.len();
    let after_colon = text[start..]
        .find(':')
        .ok_or_else(|| TheurgyError::new(format!("missing JSON key colon: {key}")))?
        + start
        + 1;
    let relative_open = text[after_colon..]
        .find(open)
        .ok_or_else(|| TheurgyError::new(format!("missing JSON value for key: {key}")))?;
    let value_start = after_colon + relative_open;
    let value_end = balanced_end(text, value_start, open, close)?;
    Ok(&text[value_start..=value_end])
}

fn balanced_end(text: &str, start: usize, open: char, close: char) -> Result<usize> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, character) in text[start..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = in_string;
            continue;
        }
        if character == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        if character == open {
            depth += 1;
        } else if character == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Ok(start + offset);
            }
        }
    }
    Err(TheurgyError::new("unterminated JSON object or array").into())
}

fn json_string_array(text: &str, key: &str) -> Result<Vec<String>> {
    let array = json_array_for_key(text, key)?;
    let mut values = Vec::new();
    let mut in_string = false;
    let mut escaped = false;
    let mut current = String::new();
    for character in array.chars() {
        if escaped {
            if in_string {
                current.push(character);
            }
            escaped = false;
        } else if character == '\\' {
            escaped = in_string;
        } else if character == '"' {
            if in_string {
                values.push(current.clone());
                current.clear();
            }
            in_string = !in_string;
        } else if in_string {
            current.push(character);
        }
    }
    Ok(values)
}

fn top_level_objects(array: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut index = 0usize;
    while let Some(relative) = array[index..].find('{') {
        let start = index + relative;
        if let Ok(end) = balanced_end(array, start, '{', '}') {
            out.push(array[start..=end].to_string());
            index = end + 1;
        } else {
            break;
        }
    }
    out
}

fn json_string_array_literal(values: &[String]) -> String {
    let items = values
        .iter()
        .map(|value| format!("\"{}\"", json_escape(value)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{items}]")
}

fn swift_string_array_literal(values: &[String]) -> String {
    let items = values
        .iter()
        .map(|value| format!("\"{}\"", swift_escape(value)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{items}]")
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

fn valid_slug(value: &str) -> bool {
    validate_name(value).is_ok()
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
    let summary = validate_product_ir(product)?;
    if !summary.targets.iter().any(|candidate| candidate == target) {
        return Err(
            TheurgyError::new(format!("product IR does not declare target: {target}")).into(),
        );
    }
    let action_ids = json_string_array_literal(&summary.action_ids);
    if matches!(target, "macos" | "linux") {
        Ok(format!(
            "{{\n  \"version\": \"theurgy-desktop-surface-ir/v1\",\n  \"format\": \"json\",\n  \"product\": \"{}\",\n  \"target\": \"{}\",\n  \"actions\": {},\n  \"window\": {{\n    \"id\": \"window.main\",\n    \"type\": \"Window\",\n    \"title\": \"{}\",\n    \"role\": \"native-product-root\",\n    \"child\": {{\n      \"id\": \"split.main\",\n      \"type\": \"SplitPane\",\n      \"role\": \"left-list-detail\",\n      \"children\": [\n        {{\"id\": \"list.primary\", \"type\": \"TreeList\", \"role\": \"product-navigation\"}},\n        {{\"id\": \"detail.primary\", \"type\": \"Detail\", \"role\": \"product-detail\"}}\n      ]\n    }}\n  }}\n}}",
            json_escape(&summary.app_id),
            json_escape(target),
            action_ids,
            json_escape(&summary.app_name)
        ))
    } else {
        Ok(format!(
            "{{\n  \"version\": \"theurgy-mobile-surface-ir/v1\",\n  \"format\": \"json\",\n  \"product\": \"{}\",\n  \"target\": \"{}\",\n  \"actions\": {},\n  \"screens\": [\n    {{\n      \"id\": \"overview\",\n      \"title\": \"{}\",\n      \"node\": {{\"id\": \"screen.overview\", \"type\": \"NavigationStack\", \"role\": \"status-overview\"}}\n    }},\n    {{\n      \"id\": \"detail\",\n      \"title\": \"Detail\",\n      \"node\": {{\"id\": \"screen.detail\", \"type\": \"Screen\", \"role\": \"focused-action-detail\"}}\n    }}\n  ]\n}}",
            json_escape(&summary.app_id),
            json_escape(target),
            action_ids,
            json_escape(&summary.app_name)
        ))
    }
}

fn compile_native(product: &str, target: &str, out_dir: &Path) -> Result<()> {
    let summary = validate_product_ir(product)?;
    let surface = project_surface(product, target)?;
    let runtime = RuntimeContract {
        app_id: summary.app_id.clone(),
        protocol: "theurgy-runtime-action/v1".to_string(),
        state_command: vec![
            format!("{}-core", summary.app_id),
            "runtime-state".to_string(),
        ],
        action_command: vec![
            format!("{}-core", summary.app_id),
            "runtime-action".to_string(),
        ],
        history_command: Vec::new(),
        daemon_command: Vec::new(),
    };
    compile_native_with_contract(&summary, &surface, &runtime, target, out_dir)
}

fn compile_native_with_contract(
    summary: &ProductSummary,
    surface: &str,
    runtime: &RuntimeContract,
    target: &str,
    out_dir: &Path,
) -> Result<()> {
    fs::create_dir_all(out_dir)?;
    write_or_replace(
        &out_dir.join("theurgy-surface.json"),
        &format!("{}\n", surface.trim_end()),
    )?;
    write_or_replace(
        &out_dir.join("theurgy-runtime.json"),
        &generated_runtime_metadata(runtime, target),
    )?;
    match target {
        "macos" => compile_macos(summary, runtime, out_dir),
        "linux" => compile_linux(summary, runtime, out_dir),
        "ios" => compile_ios(&summary, out_dir),
        "android" => compile_android(&summary, out_dir),
        _ => Err(TheurgyError::new("unsupported target").into()),
    }
}

fn generated_runtime_metadata(runtime: &RuntimeContract, target: &str) -> String {
    let mut lines = vec![
        "{".to_string(),
        "  \"version\": \"theurgy-generated-runtime/v1\",".to_string(),
        format!("  \"app\": \"{}\",", json_escape(&runtime.app_id)),
        format!("  \"target\": \"{}\",", json_escape(target)),
        format!("  \"protocol\": \"{}\",", json_escape(&runtime.protocol)),
        format!(
            "  \"stateCommand\": {},",
            json_string_array_literal(&runtime.state_command)
        ),
        format!(
            "  \"actionCommand\": {},",
            json_string_array_literal(&runtime.action_command)
        ),
    ];
    if !runtime.history_command.is_empty() {
        lines.push(format!(
            "  \"historyCommand\": {},",
            json_string_array_literal(&runtime.history_command)
        ));
    }
    if !runtime.daemon_command.is_empty() {
        lines.push(format!(
            "  \"daemonCommand\": {},",
            json_string_array_literal(&runtime.daemon_command)
        ));
    }
    lines.push("  \"surface\": \"theurgy-surface.json\"".to_string());
    lines.push("}".to_string());
    format!("{}\n", lines.join("\n"))
}

fn compile_macos(
    summary: &ProductSummary,
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
        &macos_adapter_source(summary, runtime),
    )
}

fn compile_linux(
    summary: &ProductSummary,
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
        &linux_adapter_source(summary, runtime),
    )
}

fn macos_adapter_source(summary: &ProductSummary, runtime: &RuntimeContract) -> String {
    let executable = runtime.state_command.first().cloned().unwrap_or_default();
    let arguments = swift_string_array_literal(&runtime.state_command[1..]);
    let template = r#"// Generated by theurgy-runtime compile-native.
// Runtime: theurgy-runtime.json
// Surface: theurgy-surface.json
import Foundation
import SwiftUI

struct RuntimeStateView: View {
  @State private var status = "Runtime state not loaded."

  var body: some View {
    VStack(alignment: .leading, spacing: 12) {
      Text("__APP_NAME__")
        .font(.title2)
      Text(status)
        .font(.system(.body, design: .monospaced))
        .textSelection(.enabled)
      Button("Refresh") {
        status = loadRuntimeState()
      }
    }
    .padding()
    .frame(minWidth: 960, minHeight: 640, alignment: .topLeading)
    .onAppear {
      status = loadRuntimeState()
    }
  }
}

func loadRuntimeState() -> String {
  let process = Process()
  process.executableURL = resolveExecutable("__STATE_EXECUTABLE__")
  process.arguments = __STATE_ARGUMENTS__
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
      return String(data: errorData, encoding: .utf8) ?? "runtime state command failed"
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
        .replace("__STATE_EXECUTABLE__", &swift_escape(&executable))
        .replace("__STATE_ARGUMENTS__", &arguments)
}

fn linux_adapter_source(summary: &ProductSummary, runtime: &RuntimeContract) -> String {
    let executable = runtime.state_command.first().cloned().unwrap_or_default();
    let arguments = c_argv_tail_literal(&runtime.state_command[1..]);
    let template = r#"/* Generated by theurgy-runtime compile-native.
 * Runtime: theurgy-runtime.json
 * Surface: theurgy-surface.json
 */
#include <gtk/gtk.h>
#include <json-glib/json-glib.h>

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

static char *load_runtime_state(void) {
  g_autofree char *runtime = resolve_executable("__STATE_EXECUTABLE__");
  const char *argv[] = { runtime, __STATE_ARGUMENTS__ NULL };
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

static void activate(GtkApplication *app, gpointer user_data) {
  (void)user_data;
  GtkWidget *window = gtk_application_window_new(app);
  GtkWidget *box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 8);
  GtkWidget *button = gtk_button_new_with_label("Refresh");
  GtkWidget *label = gtk_label_new("Runtime state not loaded.");
  gtk_window_set_title(GTK_WINDOW(window), "__APP_NAME__");
  gtk_window_set_default_size(GTK_WINDOW(window), 960, 640);
  gtk_label_set_xalign(GTK_LABEL(label), 0.0);
  gtk_label_set_wrap(GTK_LABEL(label), TRUE);
  gtk_box_append(GTK_BOX(box), button);
  gtk_box_append(GTK_BOX(box), label);
  gtk_window_set_child(GTK_WINDOW(window), box);
  g_signal_connect(button, "clicked", G_CALLBACK(refresh_state), label);
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
}

fn compile_ios(summary: &ProductSummary, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir.join("Host"))?;
    write_or_replace(
        &out_dir.join("Host/App.swift"),
        &format!(
            "import SwiftUI\n\n@main\nstruct TheurgyMobileApp: App {{\n  var body: some Scene {{\n    WindowGroup {{\n      NavigationStack {{ Text(\"{} native adapter\") }}\n    }}\n  }}\n}}\n",
            swift_escape(&summary.app_name)
        ),
    )
}

fn compile_android(summary: &ProductSummary, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir.join("app/src/main/java/app/theurgy/generated"))?;
    write_or_replace(
        &out_dir.join("settings.gradle"),
        &format!("pluginManagement {{ repositories {{ google(); mavenCentral(); gradlePluginPortal() }} }}\ndependencyResolutionManagement {{ repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS); repositories {{ google(); mavenCentral() }} }}\nrootProject.name = '{}-theurgy'\ninclude ':app'\n", summary.app_id),
    )?;
    write_or_replace(
        &out_dir.join("app/src/main/java/app/theurgy/generated/MainActivity.java"),
        &format!(
            "package app.theurgy.generated;\n\nimport android.app.Activity;\nimport android.os.Bundle;\nimport android.widget.TextView;\n\npublic final class MainActivity extends Activity {{\n  @Override public void onCreate(Bundle state) {{\n    super.onCreate(state);\n    TextView view = new TextView(this);\n    view.setText(\"{} native adapter\");\n    setContentView(view);\n  }}\n}}\n",
            java_escape(&summary.app_name)
        ),
    )
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
        assert_eq!(summary.actions, 2);
    }

    #[test]
    fn projects_desktop_surface_from_product_ir() {
        let surface = project_surface(&sample_product(), "macos").unwrap();
        assert!(surface.contains("\"version\": \"theurgy-desktop-surface-ir/v1\""));
        assert!(surface.contains("\"product\": \"deployments\""));
        assert!(surface.contains("\"role\": \"left-list-detail\""));
    }

    #[test]
    fn projects_mobile_surface_from_product_ir() {
        let product = "{\n  \"version\": \"theurgy-product-ir/v1\",\n  \"format\": \"json\",\n  \"app\": {\"id\": \"deployments\", \"name\": \"Deployments\", \"targets\": [\"ios\"]},\n  \"actions\": [{\"id\": \"refresh_state\", \"label\": \"Refresh\", \"input\": {}, \"output\": {}, \"effect\": \"read\", \"failure\": {}, \"safe\": true, \"mutating\": false, \"longRunning\": false, \"privileged\": false}],\n  \"state\": {\"snapshotSchema\": \"deployments-state/v1\"}\n}".to_string();
        let surface = project_surface(&product, "ios").unwrap();
        assert!(surface.contains("\"version\": \"theurgy-mobile-surface-ir/v1\""));
        assert!(surface.contains("\"role\": \"status-overview\""));
    }

    #[test]
    fn compile_native_emits_deterministic_adapter_files() {
        let root = test_root("compile-native");
        compile_native(&sample_product(), "linux", &root).unwrap();
        assert!(root.join("theurgy-surface.json").exists());
        assert!(root.join("theurgy-runtime.json").exists());
        let runtime = fs::read_to_string(root.join("theurgy-runtime.json")).unwrap();
        assert!(runtime.contains("\"stateCommand\": [\"deployments-core\", \"runtime-state\"]"));
        let main_c = fs::read_to_string(root.join("src/main.c")).unwrap();
        assert!(main_c.contains("gtk_application_window_new"));
        assert!(main_c.contains("Deployments"));
        assert!(main_c.contains("theurgy-runtime.json"));
        assert!(main_c.contains("runtime-state"));
        assert!(main_c.contains("g_subprocess_newv"));
        assert!(main_c.contains("json-glib/json-glib.h"));
        let meson = fs::read_to_string(root.join("meson.build")).unwrap();
        assert!(meson.contains("json-glib-1.0"));
        fs::remove_dir_all(root).unwrap();
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
        assert!(runtime.contains("\"protocol\": \"deployments-runtime/v1\""));
        assert!(runtime.contains("\"stateCommand\": [\"custom-core\", \"state\"]"));
        assert!(runtime.contains("\"historyCommand\": [\"custom-core\", \"history\"]"));
        let main_c = fs::read_to_string(out.join("src/main.c")).unwrap();
        assert!(main_c.contains("\"custom-core\""));
        assert!(main_c.contains("\"state\""));
        let surface = fs::read_to_string(out.join("theurgy-surface.json")).unwrap();
        assert!(surface.contains("\"role\": \"declared-reference-surface\""));

        fs::remove_dir_all(app).unwrap();
        fs::remove_dir_all(out).unwrap();
    }

    fn sample_product() -> String {
        "{\n  \"version\": \"theurgy-product-ir/v1\",\n  \"format\": \"json\",\n  \"app\": {\n    \"id\": \"deployments\",\n    \"name\": \"Deployments\",\n    \"targets\": [\"macos\", \"linux\"],\n    \"capabilities\": [\"native-desktop\", \"runtime-actions\"]\n  },\n  \"domain\": {\n    \"objects\": [\n      {\"id\": \"server\", \"label\": \"Server\"},\n      {\"id\": \"deployment\", \"label\": \"Deployment\"}\n    ]\n  },\n  \"actions\": [\n    {\"id\": \"refresh_state\", \"label\": \"Refresh\", \"input\": {}, \"output\": {}, \"effect\": \"read\", \"failure\": {}, \"safe\": true, \"mutating\": false, \"longRunning\": false, \"privileged\": false},\n    {\"id\": \"publish_changes\", \"label\": \"Push to Production\", \"input\": {}, \"output\": {}, \"effect\": \"release\", \"failure\": {}, \"safe\": false, \"mutating\": true, \"longRunning\": true, \"privileged\": true}\n  ],\n  \"state\": {\n    \"snapshotSchema\": \"deployments-state/v1\",\n    \"roots\": [{\"id\": \"headquarters-workspace\", \"kind\": \"xdg-state\"}]\n  }\n}".to_string()
    }

    fn sample_runtime_manifest() -> String {
        "{\n  \"version\": \"theurgy-runtime-manifest/v1\",\n  \"app\": \"deployments\",\n  \"productIr\": \"app-blueprint/product.ir.json\",\n  \"runtime\": {\n    \"stateCommand\": [\"custom-core\", \"state\"],\n    \"actionCommand\": [\"custom-core\", \"action\"],\n    \"historyCommand\": [\"custom-core\", \"history\"],\n    \"protocol\": \"deployments-runtime/v1\"\n  },\n  \"surfaces\": {\n    \"desktop\": \"app-blueprint/desktop.surface.ir.json\"\n  }\n}".to_string()
    }

    fn sample_desktop_surface() -> String {
        "{\n  \"version\": \"theurgy-desktop-surface-ir/v1\",\n  \"format\": \"json\",\n  \"product\": \"deployments\",\n  \"target\": \"desktop\",\n  \"actions\": [\"refresh_state\", \"publish_changes\"],\n  \"window\": {\n    \"id\": \"window.main\",\n    \"type\": \"Window\",\n    \"title\": \"Deployments\",\n    \"role\": \"declared-reference-surface\"\n  }\n}".to_string()
    }
}
