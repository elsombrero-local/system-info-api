use rocket::serde::json;
use rumqttc::{Client, QoS};
use sysinfo::{Disks, System};

use crate::http::api::{process::model::get_all_processes, system_information::model::SystemInformation};

pub fn run(sys: &mut System, disks: &mut Disks, client: &mut Client) {
  sys.refresh_all();
  disks.refresh_list();
  let si = SystemInformation::new(sys, disks);
  let processes = get_all_processes(sys.processes());
  let _ = client.publish("rasp/stats", QoS::AtMostOnce, false, json::to_string(&si).unwrap_or_default());
  let _ = client.publish("rasp/processes", QoS::AtMostOnce, false, json::to_string(&processes).unwrap_or_default());
}