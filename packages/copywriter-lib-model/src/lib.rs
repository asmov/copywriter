pub mod article;

use asmov_copywriter_lib;
use validator;

pub mod prelude {
    pub use asmov_copywriter_lib::modeling::{Model, ModelType, Content};
    pub  use validator::Validate;
}

pub use crate::{
    asmov_copywriter_lib::modeling::*,
    article::*
};
