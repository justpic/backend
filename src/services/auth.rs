use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use time::OffsetDateTime;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    AppState,
    error::Error,
    models::{
        api::{SelfUserOut, UserRegisterDto},
        database::{Role, User},
    },
};

fn create_new_user(email: String, username: String, password_hash: String) -> User {
    User {
        id: Uuid::new_v4(),
        created: OffsetDateTime::now_utc(),
        username,
        email,
        password_hash,
        role: Role::Regular,
        avatar_url: None,
        email_confirmed: false,
        nsfw_allowed: false,
    }
}

async fn hash_password(password: String) -> Result<String, Error> {
    let hashed_password = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        argon
            .hash_password(password.as_bytes(), &salt)
            .map(|v| v.to_string())
    })
    .await??;

    Ok(hashed_password)
}

pub async fn register(state: &AppState, payload: UserRegisterDto) -> Result<SelfUserOut, Error> {
    let register_id = Uuid::new_v4().to_string();
    info!(
        "[reg_id: {}] User registration request received",
        &register_id[0..8]
    );

    if state
        .repos
        .users
        .check_exist(&payload.username, &payload.email)
        .await?
    {
        warn!(
            "[reg_id: {}] New user registration cancelled (Reason: user with specified data already exists)",
            &register_id[0..8]
        );
        return Err(Error::Conflict);
    }

    let password_hash = hash_password(payload.password).await?;

    let new_user = create_new_user(payload.email, payload.username, password_hash);
    state.repos.users.insert(&new_user).await.inspect_err(|e| {
        warn!(
            "[reg_id: {}] New user registration failed (Reason: {:?})",
            &register_id[0..8],
            e
        );
    })?;

    info!("[reg_id: {}] New user registered", &register_id[0..8]);
    Ok(new_user.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_user() {
        let email = "test@example.com".to_string();
        let username = "tester".to_string();
        let password = "hunter42".to_string();

        let user = create_new_user(email.clone(), username.clone(), password.clone());

        // Checking username
        assert_eq!(user.username, username);

        // Checking email
        assert_eq!(user.email, email);

        // Checking role
        assert!(matches!(user.role, Role::Regular))
    }

    #[tokio::test]
    async fn should_correct_hash_password() {
        let password = "hunter42".to_string();

        let hashed = hash_password(password).await.unwrap();
    }
}
