use actix_web::{HttpResponse, Responder, get, post, web};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

// 用于存储秘密消息的结构体
#[derive(Debug, Clone, Serialize)]
pub struct Secret {
    pub message: String,
    #[serde(skip_serializing)]
    pub expires_at: SystemTime,
}

// 创建秘密消息时，从客户端接收的数据结构
#[derive(Debug, Deserialize)]
pub struct CreateSecretPayload {
    pub message: String,
    pub expires_in_secs: Option<u64>,
}

// 应用的状态，用于在多个线程间共享数据
pub struct AppState {
    pub secrets: Arc<Mutex<HashMap<String, Secret>>>,
}

// 创建秘密消息的处理函数
#[post("/api/secrets")]
pub async fn create_secret(
    payload: web::Json<CreateSecretPayload>,
    data: web::Data<AppState>,
) -> impl Responder {
    if payload.message.chars().count() > 500 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Message is too long. Maximum 500 characters allowed."
        }));
    }

    const DEFAULT_EXPIRATION_SECS: u64 = 600; // 10 minutes
    const ALLOWED_EXPIRATIONS: &[u64] = &[60, 300, 600, 1800, 3600, 21600, 43200, 86400];

    let expiration_secs = payload.expires_in_secs.unwrap_or(DEFAULT_EXPIRATION_SECS);

    if !ALLOWED_EXPIRATIONS.contains(&expiration_secs) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid expiration time provided."
        }));
    }

    let expires_at = SystemTime::now() + Duration::from_secs(expiration_secs);
    
    let mut secrets = data.secrets.lock().unwrap();
    let id = nanoid!(10);
    let secret = Secret {
        message: payload.message.clone(),
        expires_at,
    };
    secrets.insert(id.clone(), secret);
    HttpResponse::Ok().json(serde_json::json!({ "id": id }))
}

// 获取并销毁秘密消息的处理函数
#[get("/api/secrets/{id}")]
pub async fn get_secret(id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut secrets = data.secrets.lock().unwrap();
    let id = id.into_inner();
    let error_message = serde_json::json!({"error": "Oops.. 消息不存在或已被销毁。"});

    if let Some(secret) = secrets.get(&id) {
        // 检查是否过期
        if SystemTime::now() > secret.expires_at {
            // 如果过期，则移除
            secrets.remove(&id);
            return HttpResponse::NotFound().json(error_message);
        }
    }
    
    // 如果未过期或获取失败，都尝试移除（阅后即焚）
    if let Some(secret) = secrets.remove(&id) {
        HttpResponse::Ok().json(secret)
    } else {
        HttpResponse::NotFound().json(error_message)
    }
}
