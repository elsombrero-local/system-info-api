use serde::{Deserialize, Serialize};
use sysinfo::Disks;

#[derive(Serialize, Deserialize, Debug)]
pub struct Disk {
  pub name: String,
  pub available_space: u64,
  pub total_space: u64,
  pub kind: String,
  pub mount_point: String,

}

pub fn get_disk_list(disk: &Disks) -> Vec<Disk> {
  disk.into_iter().map(|d| {
    Disk {
      name: d.name().to_string_lossy().to_string(),
      available_space: d.available_space(),
      total_space: d.total_space(),
      kind: d.kind().to_string(),
      mount_point: d.mount_point().to_string_lossy().to_string(),

    }
  }).collect::<Vec<Disk>>()
}