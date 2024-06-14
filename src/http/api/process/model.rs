use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sysinfo::Pid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
  pub cpu_usage: f32,
  pub pid: String,
  pub name: String,
  pub memory: u64,
  pub virtual_memory: u64,
  pub status: String,
  pub start_time: u64,
  pub run_time: u64,
}

pub fn get_all_processes(processes: &HashMap<Pid, sysinfo::Process>) -> Vec<Process> {
  processes.iter().map(|(pid, process)| {  
    Process {
      cpu_usage: process.cpu_usage(),
      pid: pid.to_string(),
      name: process.name().to_string(),
      memory: process.memory(),
      virtual_memory: process.virtual_memory(),
      status: process.status().to_string(),
      start_time: process.start_time(),
      run_time: process.run_time(),
    }
  })
  .collect::<Vec<Process>>()
}