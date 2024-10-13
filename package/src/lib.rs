//! # RowID
//!
//! A time-based unique ID solution.
//!
//! ## Quick Start
//!
//! Create an ID with the following code:
//!
//! ```no_run
//! use rowid::rowid;
//!
//! let id: String = rowid();
//! ```
//!
//! Or start a customization with the following code:
//!
//! ```no_run
//! use rowid::{RowIDWithConfig, RowIDWithConfigResult};
//!
//! let rwc: RowIDWithConfigResult = RowIDWithConfig::new()
//!     .char_list("0123456789ABCDEFGHJKMNPQRSTVWXYZ")
//!     .randomness_length(22)
//!     .done()
//!     .unwrap();
//!
//! let id: String = rwc.rowid();
//! ```

mod base;
mod common;
mod functions;
mod utils;

// basics

pub use crate::functions::generate::GenerateResult;
pub use crate::functions::verify::VerifyResult;

pub use crate::base::rowid::{
    decode, encode, generate, get_randomness, rowid, verify,
};

pub use crate::base::rowid_with_config::{
    RowIDWithConfig, RowIDWithConfigResult, RowIDWithConfigState,
};

pub use crate::utils::system_time::{
    system_time_to_timestamp, timestamp_to_system_time,
};

pub use crate::common::errors::RowIDError;
