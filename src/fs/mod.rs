pub mod local;

use result::RustyPlatterResult;

/// Basic filesystem trait
pub trait Filesystem {
    fn mkdir(&self, path: &str) -> RustyPlatterResult<()>;
    fn mv(&self, from: &str, to: &str) -> RustyPlatterResult<()>;
    fn rm(&self, path: &str) -> RustyPlatterResult<()>;
    fn exists(&self, path: &str) -> bool;
}