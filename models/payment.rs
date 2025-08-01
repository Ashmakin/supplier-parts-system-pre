// src/models/payment.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CheckoutSessionResponse {
    pub session_id: String,
}