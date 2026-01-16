// System command paths for hbsd-update
use std::{path::Path, str};

pub static AWK: &'static Path = Path::new("/usr/bin/awk");
pub static BEADM: &'static Path = Path::new("/usr/local/sbin/beadm");
pub static CAT: &'static Path = Path::new("/bin/cat");
pub static CHFLAGS: &'static Path = Path::new("/bin/chflags");
pub static CERTCTL: &'static Path = Path::new("/usr/sbin/certctl");
pub static DRILL: &'static Path = Path::new("/usr/bin/drill");
pub static ETCUPDATE: &'static Path = Path::new("/usr/sbin/etcupdate");
pub static FETCH: &'static Path = Path::new("/usr/bin/fetch");
pub static FIND: &'static Path = Path::new("/usr/bin/find");
pub static GREP: &'static Path = Path::new("/usr/bin/grep");
pub static JLS: &'static Path = Path::new("/usr/sbin/jls");
pub static KLDXREF: &'static Path = Path::new("/usr/sbin/kldxref");
pub static MKTEMP: &'static Path = Path::new("/usr/bin/mktemp");
pub static OPENSSL: &'static Path = Path::new("/usr/bin/openssl");
pub static PWD_MKDB: &'static Path = Path::new("/usr/sbin/pwd_mkdb");
pub static SED: &'static Path = Path::new("/usr/bin/sed");
pub static SHA256: &'static Path = Path::new("/sbin/sha256");
pub static SHA512: &'static Path = Path::new("/sbin/sha512");
pub static SYSCTL: &'static Path = Path::new("/sbin/sysctl");
pub static TAIL: &'static Path = Path::new("/usr/bin/tail");
pub static TAR: &'static Path = Path::new("/usr/bin/tar");
pub static UNBOUND_HOST: &'static Path = Path::new("/usr/sbin/unbound-host");
