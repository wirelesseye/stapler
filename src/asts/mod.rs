mod block;
mod stmt;
mod program;
mod r#type;
mod param;

use std::fmt::Debug;

pub use block::*;
pub use stmt::*;
pub use program::*;
pub use r#type::*;
pub use param::*;

pub trait ASTTrait: Debug {}