use axum::{
    extract::{State, Json},
    http::StatusCode,
    routing::get,
    Router,
};
use serde::{Serialize, Deserialize};
use crate::db::Database;
use crate::models::{AuditLog, AuditAction};
use crate::middleware::auth::AuthenticatedUser;

// 这是一个空的模块，将在未来实现审计日志相关功能
// 目前仅作为占位符，以解决模块导入错误
