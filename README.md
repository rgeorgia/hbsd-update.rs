# hbsd-update.rs
HBSD: hbsd-update in rust

```bash
./target/debug/hbsd-update --help                          Thu Jan 15 22:58:07 2026
HardenedBSD binary update utility

Usage: hbsd-update [OPTIONS]

Options:
      --fetchonly                  Fetch only (do not install)
      --downloadonly               Download only (fetch payloads but do not proceed to install/apply)
      --ignorever                  Ignore version check
      --install-src                Install sources (if present)
      --integriforce               Force integrity verification (enabled by default; use --no-integriforce to disable)
      --no-integriforce            Disable integrity verification
      --interactive                Interactively remove obsolete files
      --keeptmp                    Keep temporary directory/files
      --remote-resolver            Use system resolver for remote checks (do not use the DNSSEC path)
      --machine-readable           Machine-readable output (e.g., JSON-friendly)
      --no-kernel                  Do not install kernel
      --no-obsolete                Do not remove obsolete files/directories
      --nobase                     Do not update base system
      --nodownload                 Do not download (assume artifacts already present)
      --unsigned                   Skip signature checks / allow unsigned updates
      --verbose                    Verbose output
      --force-ipv4                 Force IPv4, Conflicts with IPv6
      --force-ipv6                 Force IPv6, Conflicts with IPv4
      --dnssec                     Enable DNSSEC validation (enabled by default; use --no-dnssec to disable)
      --no-dnssec                  Disable DNSSEC validation
      --update-hash <HASH>         Override / pin the expected update hash (empty means "auto") [default: ]
      --net-flag <FLAG>            Extra network flag string (empty means "none") [default: ]
      --config-file <HBSD-CONFIG>  Path to custom configuration file [default: /etc/hbsd-update.conf]
  -h, --help                       Print help
  -V, --version                    Print version
```
