use serde::{Deserialize, Serialize};

use crate::http::api::{cpu::model::Cpu, disk::model::Disk, memory::model::Memory};

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInformation {
  pub cpu: Cpu,
  pub memory: Memory,
  pub disks: Vec<Disk>,
}
