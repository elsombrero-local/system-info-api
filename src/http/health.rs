use rocket::{get, http::Status, serde::json::Json};
use super::response::MessageResponse;

#[get("/")]
pub async fn health() -> (Status, Json<MessageResponse>) {
  (Status::Ok, Json(
    MessageResponse{
      message: String::from("Service Healthy") 
    }
  ))
}