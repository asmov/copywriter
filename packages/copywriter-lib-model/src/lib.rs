pub mod article;

use asmov_copywriter_lib;
use garde;

pub mod prelude {
    pub use asmov_copywriter_lib::modeling::{Model, ModelTypeAssoc};
    pub use garde::Validate;
}

pub use crate::{
    asmov_copywriter_lib::modeling::*,
    article::*
};
