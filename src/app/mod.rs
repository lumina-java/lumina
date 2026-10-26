pub mod controllers;
pub mod models;
pub mod services;
pub mod requests;

/// Application Prelude Hub — Satu baris import untuk seluruh controller & service:
/// `use crate::app::prelude::*;`
#[allow(unused_imports)]
pub mod prelude {
    pub use lumina::prelude::*;
    pub use crate::app::models::*;
    pub use crate::app::requests::*;
}
