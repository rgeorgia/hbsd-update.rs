// cli.rs
//
// clap v4 CLI definitions for an hbsd-update style utility.
// This maps directly to your internal flags/defaults:
//
// fetchonly=false
// downloadonly=false
// ignorever=false
// install_src=false
// integriforce=true
// interactive=false
// keeptmp=false
// remote_resolver=false
// machine_readable=false
// no_kernel=false
// no_obsolete=false
// nobase=false
// nodownload=false
// unsigned=false
// verbose=false
// force_ipv4=false
// force_ipv6=false
// dnssec=true
// update_hash=""
// net_flag=""

use clap::{ArgAction, Parser};

#[derive(Debug, Clone, Parser)]
#[command(
    name = "hbsd-update",
    version,
    about = "HardenedBSD binary update utility",
    disable_help_flag = false
)]
pub struct Cli {
    /// Fetch only (do not install)
    #[arg(long, action = ArgAction::SetTrue)]
    pub fetchonly: bool,

    /// Download only (fetch payloads but do not proceed to install/apply)
    #[arg(long, action = ArgAction::SetTrue)]
    pub downloadonly: bool,

    /// Ignore version check
    #[arg(long, action = ArgAction::SetTrue)]
    pub ignorever: bool,

    /// Install sources (if present)
    #[arg(long, action = ArgAction::SetTrue)]
    pub install_src: bool,

    /// Force integrity verification (enabled by default; use --no-integriforce to disable)
    #[arg(long, action = ArgAction::SetTrue, default_value_t = true)]
    pub integriforce: bool,

    /// Disable integrity verification
    #[arg(long = "no-integriforce", action = ArgAction::SetFalse)]
    pub _integriforce_off: bool,

    /// Interactively remove obsolete files
    #[arg(long, action = ArgAction::SetTrue)]
    pub interactive: bool,

    /// Keep temporary directory/files
    #[arg(long, action = ArgAction::SetTrue)]
    pub keeptmp: bool,

    /// Use system resolver for remote checks (do not use the DNSSEC path)
    #[arg(long, action = ArgAction::SetTrue)]
    pub remote_resolver: bool,

    /// Machine-readable output (e.g., JSON-friendly)
    #[arg(long, action = ArgAction::SetTrue)]
    pub machine_readable: bool,

    /// Do not install kernel
    #[arg(long, action = ArgAction::SetTrue)]
    pub no_kernel: bool,

    /// Do not remove obsolete files/directories
    #[arg(long, action = ArgAction::SetTrue)]
    pub no_obsolete: bool,

    /// Do not update base system
    #[arg(long, action = ArgAction::SetTrue)]
    pub nobase: bool,

    /// Do not download (assume artifacts already present)
    #[arg(long, action = ArgAction::SetTrue)]
    pub nodownload: bool,

    /// Skip signature checks / allow unsigned updates
    #[arg(long, action = ArgAction::SetTrue)]
    pub unsigned: bool,

    /// Verbose output
    #[arg(long, action = ArgAction::SetTrue)]
    pub verbose: bool,

    /// Force IPv4, Conflicts with IPv6
    #[arg(long, action = ArgAction::SetTrue, conflicts_with = "force_ipv6")]
    pub force_ipv4: bool,

    /// Force IPv6, Conflicts with IPv4
    #[arg(long, action = ArgAction::SetTrue, conflicts_with = "force_ipv4")]
    pub force_ipv6: bool,

    /// Enable DNSSEC validation (enabled by default; use --no-dnssec to disable)
    #[arg(long, action = ArgAction::SetTrue, default_value_t = true)]
    pub dnssec: bool,

    /// Disable DNSSEC validation
    #[arg(long = "no-dnssec", action = ArgAction::SetFalse)]
    pub _dnssec_off: bool,

    /// Override / pin the expected update hash (empty means "auto")
    #[arg(long, value_name = "HASH", default_value = "")]
    pub update_hash: String,

    /// Extra network flag string (empty means "none")
    #[arg(long, value_name = "FLAG", default_value = "")]
    pub net_flag: String,

    /// Path to custom configuration file
    #[arg(long, value_name = "HBSD-CONFIG", default_value = "/etc/hbsd-update.conf")]
    pub config_file: String,

}

impl Cli {
    /// Normalize "negated" flags into the primary booleans.
    ///
    /// clap doesn't let one field cleanly represent both `--foo` and `--no-foo`
    /// without additional setup, so we accept the `--no-*` flags as helper fields
    /// and compute the final effective values here.
    pub fn normalize(mut self) -> Self {
        // If user passed --no-integriforce, then _integriforce_off will be true.
        // We want integriforce = false in that case.
        if self._integriforce_off {
            self.integriforce = false;
        }

        // If user passed --no-dnssec, then _dnssec_off will be true.
        // We want dnssec = false in that case.
        if self._dnssec_off {
            self.dnssec = false;
        }

        // Hide helper fields from downstream usage by resetting them (optional).
        self._integriforce_off = false;
        self._dnssec_off = false;

        self
    }
}
