#![allow(dead_code)]
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use lumina::prelude::*;

//                            USER MODEL
// Di Laravel, ini adalah class Eloquent (`User extends Model`).
// Cukup tambahkan `#[derive(LuminaModel)]` untuk mendapatkan fungsi CRUD otomatis:
// `User::find(id)`, `User::all()`, dll.
#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
#[table("users")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl User {
    pub fn new() -> Self {
        Self::default()
    }
}
