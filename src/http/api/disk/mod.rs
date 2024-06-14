use rocket::{futures::lock::Mutex, get, http::Status, serde::json::Json, State};
use sysinfo::Disks;
use model::{get_disk_list, Disk};

pub mod model;

#[get("/disks")]
pub async fn disks(state: &State<Mutex<Disks>>) -> (Status, Json<Vec<Disk>>) {
  let mut disks = state.lock().await;
  disks.refresh_list();
  (Status::Ok, Json(get_disk_list(&disks)))
}