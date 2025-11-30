pub const FIRST: i32 = 1;
pub const SECOND: i32 = 2;
pub const THIRD: i32 = 3;

pub mod sound {
    pub mod tame;
    pub mod wild;

    pub use tame::{dog, cat};
    pub use wild::fox;
}
pub mod prelude;