use std::thread;
use rocket::{futures::lock::Mutex, get, http::Status, serde::json::Json, State};
use sysinfo::{System, MINIMUM_CPU_UPDATE_INTERVAL};
use model::Cpu;

pub mod model;

#[get("/cpu")]
pub async fn info(state: &State<Mutex<System>>) -> (Status, Json<Cpu>) {
  let mut sys = state.lock().await;

  thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
  sys.refresh_cpu();
  
  (Status::Ok, Json(Cpu::new(&sys)))
}