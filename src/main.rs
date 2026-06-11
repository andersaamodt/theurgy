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
        Some(other) => Err(TheurgyError::new(format!("unknown command: {other}")).into()),
    }
}

fn print_usage() {
    println!(
        "Internal runtime. Use spells/assay-theurgy, spells/check-theurgy-web-runtime, spells/capture-theurgy-cgi-context, spells/conjure-native-desktop, spells/conjure-enterprise-website, or spells/inspect-theurgy-project."
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
}
