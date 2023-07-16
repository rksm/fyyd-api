pub use fyyd_types as types;

mod result;
pub mod v2;

pub use result::{Error, Result};

mod lib {
    pub use super::v2;
}
