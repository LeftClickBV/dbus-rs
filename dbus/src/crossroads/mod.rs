//! Will eventually superseed the "tree" module. It's unstable and experimental for now.
#![allow(unused_imports, dead_code, missing_docs)]

mod info;
mod handlers;
mod crossroads;
mod stdimpl;
mod path;

pub use crate::tree::MethodErr as MethodErr;

pub use self::info::{IfaceInfo, MethodInfo, PropInfo};

pub use self::crossroads::{Crossroads};

pub use self::path::{Path, PathData};

pub use self::handlers::{Handlers, Par, ParInfo};
