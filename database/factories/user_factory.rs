use lumina::prelude::*;
use crate::app::models::user::User;

// =========================================================================
//  UserFactory (Lumina Model Factory)
// =========================================================================

pub struct UserFactory;

impl UserFactory {
    pub fn new() -> Self {
        Self
    }
}

impl Factory<User> for UserFactory {
    /// Definisikan nilai dummy default untuk model User
    fn definition(&self) -> User {
        User {
            name: Faker::name(),
            email: Faker::unique_email(),
            password: "password".to_string(),
            role: "user".to_string(),
            ..Default::default()
        }
    }
}

impl HasFactory<UserFactory> for User {
    fn factory() -> UserFactory {
        UserFactory::new()
    }
}
