use serde::{Serialize, de::DeserializeOwned};
use tracing::{debug, warn};

use crate::{CacheResult, Pool, redis};

const DEFAULT_CACHE_TTL: u64 = 5 * 60; // 5 min

/// ### Wrapper for caching
///
/// Allows you to wrap an asynchronous function and cache its result,
/// or get an already cached result by key
pub async fn cache_wrapper<T, E, F, Fut>(
    pool: &Pool,
    key: impl AsRef<str>,
    fetch: F,
) -> Result<T, E>
where
    T: Serialize + DeserializeOwned + Clone + Send + 'static,
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = Result<T, E>> + Send + 'static,
{
    let key = key.as_ref();

    let get_from_cache_result = redis::get(key, pool).await;
    if let Ok(Some(cached_str)) = get_from_cache_result {
        match serde_json::from_str::<T>(&cached_str) {
            Ok(val) => {
                return Ok(val);
            }
            Err(e) => {
                warn!("serde_json error: [{e:}]");
            }
        }
    } else if let Err(e) = get_from_cache_result {
        warn!("redis error: [{e:}]");
    }

    let val = fetch().await;
    if let Ok(value_to_cache) = &val {
        match serde_json::to_string(&value_to_cache) {
            Ok(serialized) => {
                if let Err(e) = redis::set(key, serialized, DEFAULT_CACHE_TTL, pool).await {
                    warn!("redis error: [{e:}]");
                }
            }
            Err(e) => {
                warn!("serde_json error: [{e:}]")
            }
        }
    }
    val
}

/// Get entity from cache
pub async fn get_from_cache<T>(key: impl AsRef<str>, pool: &Pool) -> CacheResult<Option<T>>
where
    T: DeserializeOwned,
{
    let key = key.as_ref();

    if let Some(json) = redis::get(key, pool).await? {
        let value = serde_json::from_str::<T>(&json)?;

        debug!("\"{key}\" was obtained");
        return Ok(Some(value));
    };

    debug!("\"{key}\" not found");
    Ok(None)
}

/// Save entity in cache
pub async fn save_in_cache<T>(
    key: impl AsRef<str>,
    value: &T,
    ttl: u64,
    pool: &Pool,
) -> CacheResult<()>
where
    T: Serialize,
{
    let json = serde_json::to_string(value)?;
    let key = key.as_ref();

    redis::set(key, json, ttl, pool).await?;

    debug!("\"{key}\" was saved");
    Ok(())
}

/// Remove entity from cache by key
pub async fn remove_from_cache(key: impl AsRef<str>, pool: &Pool) -> CacheResult<()> {
    let key = key.as_ref();

    redis::remove(key, pool).await?;

    debug!("\"{key}\" was deleted");
    Ok(())
}
