// config.rs
// Configuration file structure for hbsd-update
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

// pub const BACKUPKERNEL: &str = "kernel.old";
// pub const MOUNTPOINT: &str = "/";

pub const CONFIG_FILE_PATH: &str = "/etc/hbsd-update.conf";
pub const TRUSTED_CAPATH: &str = "/usr/share/keys/hbsd-update/trusted";

#[derive(Debug, Clone)]
pub struct ConfigFile {
    pub dnsrec: String,
    pub kernel: String,
    pub capath: PathBuf,
    pub branch: String,
    pub baseurl: String,
    pub dnssec: bool,
    pub force_ipv4: bool,
    pub force_ipv6: bool,
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            dnsrec: format!(
                "{}.master.14-stable.hardened.hardenedbsd.updates.hardenedbsd.org",
                std::env::consts::ARCH
            ),
            kernel: "HARDENEDBSD".to_string(),
            capath: Path::new(TRUSTED_CAPATH).to_path_buf(),
            branch: "hardened/14-stable/master".to_string(),
            baseurl: format!(
                "https://updates.hardenedbsd.org/pub/HardenedBSD/updates/{}/{}",
                "hardened/14-stable/master",
                std::env::consts::ARCH
            ),
            dnssec: true,
            force_ipv4: false,
            force_ipv6: false,
        }
    }
}

/// Read and parse /etc/hbsd-update.conf
///
/// Semantics:
/// - Start with defaults
/// - Ignore comments and blank lines
/// - key=value (first '=' only)
/// - Unknown keys are ignored with a warning
/// - Values are NOT shell-expanded
pub fn read_config_file<P: AsRef<Path>>(path: P) -> io::Result<ConfigFile> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut cfg = ConfigFile::default();

    for (lineno, line) in reader.lines().enumerate() {
        let line = line?;
        let line = expand_uname_m(line.trim());
        let line = expand_branch(&line, &cfg.branch);

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, raw_value) = match line.split_once('=') {
            Some(v) => v,
            None => {
                eprintln!("Warning: ignoring malformed line {}: {}", lineno + 1, line);
                continue;
            }
        };

        let key = key.trim();
        let value = strip_quotes(raw_value.trim());

        match key {
            "capath" => cfg.capath = value.to_string().into(),
            "dnsrec" => cfg.dnsrec = value.to_string(),
            "kernel" => cfg.kernel = value.to_string(),
            "branch" => cfg.branch = value.to_string(),
            "baseurl" => cfg.baseurl = value.to_string(),
            "dnssec" => cfg.dnssec = parse_bool(value, cfg.dnssec),
            "force_ipv4" => cfg.force_ipv4 = parse_bool(value, cfg.force_ipv4),
            "force_ipv6" => cfg.force_ipv6 = parse_bool(value, cfg.force_ipv6),
            _ => {
                eprintln!(
                    "Warning: unknown config key '{}' on line {}",
                    key,
                    lineno + 1
                );
            }
        }
    }

    Ok(cfg)
}

/// Remove surrounding double quotes, if present
fn strip_quotes(s: &str) -> &str {
    s.strip_prefix('"')
        .and_then(|v| v.strip_suffix('"'))
        .unwrap_or(s)
}

/// Parse yes/no/true/false/1/0 with fallback
fn parse_bool(value: &str, default: bool) -> bool {
    match value {
        "yes" | "true" | "1" => true,
        "no" | "false" | "0" => false,
        _ => {
            eprintln!("Warning: invalid boolean '{}', using default", value);
            default
        }
    }
}

// Helper functions to expand
fn expand_uname_m(s: &str) -> String {
    s.replace("$(uname -m)", std::env::consts::ARCH)
}

fn expand_branch(s: &str, branch: &str) -> String {
    s.replace("${branch}", branch)
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Dnssec {
    pub dnssec_key: &'static Path,
    pub revoke_dir: &'static Path,
}

impl Default for Dnssec {
    fn default() -> Self {
        Dnssec {
            dnssec_key: Path::new("/usr/share/keys/hbsd-update/trusted/dnssec.key"),
            revoke_dir: Path::new("/usr/share/keys/hbsd-update/revoked"),
        }
    }
}
