use std::thread;

use model::SystemInformation;
use rocket::{futures::lock::Mutex, get, http::Status, serde::json::Json, State};
use sysinfo::{Disks, System, MINIMUM_CPU_UPDATE_INTERVAL};

pub mod model;

#[get("/info")]
pub async fn info(sys_state: &State<Mutex<System>>, disks_state: &State<Mutex<Disks>>) -> (Status, Json<SystemInformation>) {
  let mut sys = sys_state.lock().await;
  let mut disks = disks_state.lock().await;
  thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
  sys.refresh_cpu();
  sys.refresh_memory();
  disks.refresh_list();
  (Status::Ok, Json(SystemInformation::new(&sys, &disks)))
}
