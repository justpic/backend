use argon2::{
    Argon2, Params, PasswordHash, PasswordVerifier,
    password_hash::{
        PasswordHasher, SaltString,
        rand_core::{OsRng, RngCore},
    },
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use base64::{Engine, prelude::BASE64_STANDARD};
use time::{Duration, OffsetDateTime};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    AppState,
    error::Error,
    models::{
        api::{
            REDIS_SESSION_PREFIX, SESSION_COOKIE_NAME, SelfUserOut, SessionOut, SessionWithRole,
        },
        database::{Role, Session, User},
    },
    routes::v1::auth::{LoginDto, UserRegisterDto},
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

fn generate_session_key() -> String {
    let mut raw_key = [0; 64];
    OsRng.fill_bytes(&mut raw_key);
    BASE64_STANDARD.encode(raw_key)
}

fn create_new_session(user_id: Uuid, user_agent: Option<String>) -> Session {
    let now = OffsetDateTime::now_utc();

    Session {
        session_id: Uuid::new_v4(),
        session_key: generate_session_key(),
        user_id,
        created: now,
        expires: now + Duration::days(28),
        os: None,
        device: None,
        user_agent,
    }
}

async fn hash_password(password: String) -> Result<String, Error> {
    let hashed_password = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(8 * 1024, 2, 1, Some(32)).unwrap();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|v| v.to_string())
    })
    .await??;

    Ok(hashed_password)
}

async fn verify_password(pwd: String, hash: String) -> Result<bool, Error> {
    tokio::task::spawn_blocking(move || -> Result<bool, Error> {
        let parsed_hash = PasswordHash::new(&hash)?;
        let res = Argon2::default().verify_password(pwd.as_bytes(), &parsed_hash).is_ok();

        Ok(res)
    })
    .await?
}

pub async fn register(state: &AppState, payload: UserRegisterDto) -> Result<SelfUserOut, Error> {
    let register_id = Uuid::new_v4().to_string();
    info!(
        "[reg_id: {}] User registration request received",
        &register_id[0..8]
    );

    if state
        .db
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
    state.db.users.insert(&new_user).await.inspect_err(|e| {
        warn!(
            "[reg_id: {}] New user registration failed (Reason: {:?})",
            &register_id[0..8],
            e
        );
    })?;

    info!("[reg_id: {}] New user registered", &register_id[0..8]);
    Ok(new_user.into())
}

pub async fn login(
    state: &AppState,
    payload: LoginDto,
    jar: CookieJar,
    user_agent: &str,
) -> Result<(CookieJar, SessionOut), Error> {
    let user = state
        .db
        .users
        .get_by_email::<User>(&payload.email)
        .await?
        .ok_or(Error::InvalidCredentials)?;

    if !verify_password(payload.password, user.password_hash).await? {
        return Err(Error::InvalidCredentials);
    }

    let session = create_new_session(user.id, Some(user_agent.to_string()));

    state.db.sessions.insert(&session).await?;

    let session_with_role = SessionWithRole::from_session_with_role(session, user.role);
    let session_key = session_with_role.session.session_key.clone();
    let redis_key = [REDIS_SESSION_PREFIX, &session_key].join(":");
    state
        .redis
        .set_json(&redis_key, &session_with_role, 28 * 24 * 3600)
        .await?;

    let cookie = Cookie::build((SESSION_COOKIE_NAME, session_key))
        .max_age(Duration::days(28))
        .path("/")
        .http_only(true);

    Ok((jar.add(cookie), session_with_role.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_create_new_user() {
        let email = "test@example.com".to_string();
        let username = "tester".to_string();
        let password = "hunter42".to_string();

        let hashed = hash_password(password.clone()).await.unwrap();
        let user = create_new_user(email.clone(), username.clone(), hashed.clone());

        // Checking username
        assert_eq!(user.username, username);

        // Checking email
        assert_eq!(user.email, email);

        // Checking role
        assert!(matches!(user.role, Role::Regular));

        // Checking password
        assert!(verify_password(password, user.password_hash).await.unwrap());
    }

    #[tokio::test]
    async fn should_correct_hash_password() {
        let password = "hunter42".to_string();

        let hashed = hash_password(password.clone()).await.unwrap();

        assert!(verify_password(password, hashed.clone()).await.unwrap());

        let invalid_password = "not_a_hunter".to_string();
        assert!(!verify_password(invalid_password, hashed).await.unwrap())
    }
}
