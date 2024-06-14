use model::{get_all_processes, Process};
use rocket::{futures::lock::Mutex, get, http::Status, serde::json::Json, State};
use sysinfo::System;

pub mod model;

#[get("/processes")]
pub async fn processes(state: &State<Mutex<System>>) -> (Status, Json<Vec<Process>>) {
  let mut sys = state.lock().await;
  sys.refresh_processes(); 
  (Status::Ok, Json(get_all_processes(sys.processes())))
}