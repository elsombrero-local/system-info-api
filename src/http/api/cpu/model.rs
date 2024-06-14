use std::process::Command;

use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Serialize, Deserialize, Debug)]
pub struct Core {
  pub usage: f32,
  pub frequency: u64,
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cpu {
  cpu: usize,
  usage: f32,
  brand: String,
  cpus: Vec<Core>,
  temperature: f32,
}

impl Cpu {
  pub fn new(sys: &System) -> Self {
    let cpu_info = sys.global_cpu_info();
    println!("get_temperature");  
    Self::get_temperature();
    Cpu {
      cpu: sys.cpus().len(),
      usage: cpu_info.cpu_usage(),
      brand: if let Some(core) = sys.cpus().first() { core.brand().to_string() } else { String::from("Unknown") },
      temperature: Self::get_temperature(),
      cpus: sys.cpus().iter().map(|cpu|{
        Core {
          usage: cpu.cpu_usage(),
          frequency: cpu.frequency(),
          name: cpu.name().to_string(),
        }
      }).collect::<Vec<Core>>(),
    }
  }

  // Only available in Pi OS
  fn get_temperature() -> f32 {
    match Command::new("vcgencmd")
    .arg("measure_temp")
    .output() {
        Ok(output) => {
          let str = String::from_utf8(output.stdout).unwrap_or(String::from("temp=0.0"));
          let temp: f32 = str.split("=").last().unwrap_or("0.0").parse().unwrap_or(0.0);
          return temp
        },
        Err(err) => {
          dbg!(err);
          return 0.0
        }
    }
  }
}