# hbsd-update Functions

## Helper Functions

### is_true(val)

- Treats a value as boolean true if it is "1" or "yes".
- Returns shell success (0) if true
- Used for config and CLI flags like force_ipv4

### debug_print(msg)

- Prints a message to stderr.
- Uses echo -e, so escape sequences are interpreted
- Used for status messages and errors

### usage()

- Prints the command-line help text and exits with status 1.

### sigint_handler([destroybe])

- Handles SIGINT (Ctrl-C).
- Cleans up temporary files
- Optionally destroys a ZFS Boot Environment if destroybe=1
- Exits immediately if downloadonly=1

### get_tmpdir()

- Creates a temporary working directory using mktemp -d.
- Sets and prints tmpdir
- Returns 0 on success

### get_last_field(data)

- Returns the last whitespace-delimited field from a string.
- Used to verify DNSSEC output ends in (secure)
- Version Discovery / DNS / Fallback Logic

### dnssec_check()

- Determines the latest available update version using DNS TXT records.
- With DNSSEC enabled
- Uses unbound-host
- Verifies (secure) status
- Without DNSSEC
- Uses drill
- Returns:
		- TXT payload on success
		- Exit code 2 on failure

### get_version()

- Determines the update version and optional hash.
- Order of precedence:
- CLI -v override
- DNS TXT record
- HTTP fallback (update-latest.txt)

- Additional behavior:
	- Parses buildver and update_hash
	- Validates hash format and algorithm
	- Refuses unsigned updates unless explicitly allowed

### check_version()

- Compares remote version with cached local version.
- Returns 1 if system is already up to date
- Returns 0 if update should proceed
- Bypassed if ignorever=1

### check_jailname()

- Handles updates for jails.
- Uses jls -j <jailname> path
- Sets mountpoint to jail root
- Errors if jail does not exist
- Downloading and Verification

### fetch_update()

- Downloads and extracts the update archive.

- Steps:
1. Fetch update-<version>.tar
2. Validate SHA256/SHA512 hash (if present)
3. Exit early if downloadonly=1
4. Extract update files into tmpdir

### check_securelevel()

- Checks kern.securelevel.
- Refuses to continue if securelevel > 0

### check_pubkey_validity()

- Validates the update signing key.
- Rejects revoked keys
- Verifies certificate chain via OpenSSL
- Supports CA file or CA path

### validate_file(file, [defaulterror])

- Validates a file’s cryptographic signature.
- Uses RSA pubkey + SHA512
- Compares recovered hash to actual file hash
- Optional behavior for missing files

### check_set_validity()

- Validates the full update set.
- Required files:

		base.txz
		etcupdate.tbz
		skip.txt
		mtree.tar
		kernel-<kernel>.txz (if kernel enabled)

- Optional signed files:

		ObsoleteFiles.txt
		ObsoleteDirs.txt
		script.sh
		src.txz

## Integriforce rules
Applying the Update Payload

### apply_mtree()

- Applies filesystem structure and permissions using mtree.
- Normalizes ownership and permissions
- Ensures required directories exist

### apply_base()

- Applies the core system update.
- Includes:
		Clearing immutable flags
		Extracting base system files
		Updating /boot safely
		Installing source tree (optional)
		Running etcupdate
		Rebuilding passwd DB
		Rehashing certificates

### set_kernel_config()

- Determines which kernel package to install.
- Honors -k CLI override
- Attempts auto-detection via uname -v
- Defaults to HARDENEDBSD

### apply_kernel()

- Installs the kernel update.
- Optionally backs up existing kernel
- Updates /boot contents
- Runs kldxref

### apply_integriforce()

- Installs Integriforce security rules.
- Writes to /etc/secadm.d
- Enforces strict permissions
- Sets immutable flag

### remove_obsolete()

- Removes obsolete files and directories.
- Reads from ObsoleteFiles.txt
- Supports interactive confirmation
- Cleans up deprecated paths
- ZFS Boot Environment Handling

### create_be()

- Creates a new ZFS Boot Environment.
- Uses beadm
- Mounts BE to temporary mountpoint

### activate_be()

- Activates the newly updated Boot Environment.

### destroy_be()

- Destroys a Boot Environment.
- Used for cleanup on failure or interrupt

## State Management / Hooks

### cache_version()

- Writes the installed version to:
- /var/db/hbsd-update/version
- Used for future version comparisons.

### cleanup()

- Removes temporary files unless keeptmp=1.

### check_sanity()

- Loads configuration and validates state.
- Sources config file
- Verifies required variables
- Enforces flag exclusions
- Selects BE tooling

### pre_install_hook() / post_install_hook()

- Currently empty extension points.
- Intended for customization without patching core logic

### report_version_machine_readable()

- Intended to output version info in machine-readable form.
- ⚠ Current issue in repo
- Function recursively calls itself

## Orchestration
### main()

- Coordinates the entire update process.

## High-level flow:

1. Parse CLI options
2. Load and validate config
3. Determine network mode
4. Version check (-C)
5. Setup tempdir and signal handlers
6. Determine update version
7. Download and verify update
8. Create BE (optional)
9. Apply kernel and base system
10. Remove obsolete files
11. Activate BE
12. Cache version
13. Cleanup
