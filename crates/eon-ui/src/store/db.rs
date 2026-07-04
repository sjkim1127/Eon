use crate::store::FormState;
use chrono::Utc;
use indexed_db_futures::prelude::*;
use serde::{Deserialize, Serialize};
use std::future::IntoFuture;
use uuid::Uuid;
use wasm_bindgen::JsValue;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub form_state: FormState,
    pub created_at: i64,
}

const DB_NAME: &str = "eon_db";
const STORE_NAME: &str = "profiles";

pub async fn init_db() -> Result<IdbDatabase, String> {
    let mut db_req =
        IdbDatabase::open_f64(DB_NAME, 1.0).map_err(|e| format!("Failed to open DB: {:?}", e))?;

    db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
        if let None = evt.db().object_store_names().find(|n| n == STORE_NAME) {
            let mut params = IdbObjectStoreParameters::new();
            let key_path = IdbKeyPath::str("id");
            params.key_path(Some(&key_path));
            evt.db()
                .create_object_store_with_params(STORE_NAME, &params)?;
        }
        Ok(())
    }));

    db_req
        .into_future()
        .await
        .map_err(|e| format!("DB init failed: {:?}", e))
}

pub async fn save_profile(name: String, form_state: FormState) -> Result<UserProfile, String> {
    let db = init_db().await?;
    let tx = db
        .transaction_on_one_with_mode(STORE_NAME, IdbTransactionMode::Readwrite)
        .map_err(|e| format!("Transaction error: {:?}", e))?;
    let store = tx
        .object_store(STORE_NAME)
        .map_err(|e| format!("Store error: {:?}", e))?;

    let profile = UserProfile {
        id: Uuid::new_v4().to_string(),
        name,
        form_state,
        created_at: Utc::now().timestamp(),
    };

    let js_val = serde_wasm_bindgen::to_value(&profile)
        .map_err(|e| format!("Serialization error: {:?}", e))?;

    store
        .add_val(&js_val)
        .map_err(|e| format!("Failed to save: {:?}", e))?;

    tx.await
        .into_result()
        .map_err(|e| format!("Transaction failed: {:?}", e))?;

    Ok(profile)
}

pub async fn load_all_profiles() -> Result<Vec<UserProfile>, String> {
    let db = init_db().await?;
    let tx = db
        .transaction_on_one_with_mode(STORE_NAME, IdbTransactionMode::Readonly)
        .map_err(|e| format!("Transaction error: {:?}", e))?;
    let store = tx
        .object_store(STORE_NAME)
        .map_err(|e| format!("Store error: {:?}", e))?;

    let values = store
        .get_all()
        .map_err(|e| format!("Get all error: {:?}", e))?
        .await
        .map_err(|e| format!("Get all async error: {:?}", e))?;

    let mut profiles: Vec<UserProfile> = Vec::new();
    for js_val in values {
        if let Ok(profile) = serde_wasm_bindgen::from_value(js_val) {
            profiles.push(profile);
        }
    }

    // Sort by created_at desc
    profiles.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(profiles)
}

#[allow(dead_code)]
pub async fn delete_profile(id: &str) -> Result<(), String> {
    let db = init_db().await?;
    let tx = db
        .transaction_on_one_with_mode(STORE_NAME, IdbTransactionMode::Readwrite)
        .map_err(|e| format!("Transaction error: {:?}", e))?;
    let store = tx
        .object_store(STORE_NAME)
        .map_err(|e| format!("Store error: {:?}", e))?;

    store
        .delete(&JsValue::from_str(id))
        .map_err(|e| format!("Delete error: {:?}", e))?;

    tx.await
        .into_result()
        .map_err(|e| format!("Transaction failed: {:?}", e))?;

    Ok(())
}
