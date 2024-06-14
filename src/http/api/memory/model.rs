use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Serialize, Deserialize, Debug)]
pub struct Memory {
  pub total: u64,
  pub available: u64,
  pub used: u64,
}

impl Memory {
  pub fn new(sys: &System) -> Self {
    Memory{
      total: sys.total_memory(),
      available: sys.available_memory(),
      used: sys.used_memory(),
    }
  }
}