use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System};

use crate::http::api::{cpu::model::Cpu, disk::model::{get_disk_list, Disk}, memory::model::Memory};

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInformation {
  pub cpu: Cpu,
  pub memory: Memory,
  pub disks: Vec<Disk>,
}

impl SystemInformation {
  pub fn new(sys: &System, disks: &Disks) -> Self {
    SystemInformation {
      cpu: Cpu::new(sys),
      memory: Memory::new(sys),
      disks: get_disk_list(disks),
    }
  }
}
