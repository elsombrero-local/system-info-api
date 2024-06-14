use model::Memory;
use rocket::{futures::lock::Mutex, get, http::Status, serde::json::Json, State};
use sysinfo::System;

pub mod model;

#[get("/memory")]
pub async fn info(state: &State<Mutex<System>>) -> (Status, Json<Memory>) {
  let mut sys = state.lock().await;
  sys.refresh_memory();
  (Status::Ok, Json(Memory::new(&sys)))
}
