use std::{env, str::FromStr, thread, time::Duration};
use rumqttc::{Client, Connection, MqttOptions};
use sysinfo::{Disks, System};
use cron::Schedule;
use chrono::Utc;

type TaskFunction = fn(sys: &mut System, disks: &mut Disks, client: &mut Client);

pub fn init_mqtt(host: String, port: u16, username: String, password: String) -> (Client, Connection) {
  let mut options = MqttOptions::new("rumqtt-sync", host, port);
  options.set_credentials(username, password);
  Client::new(options, 10)
}

pub fn execute(expr: String, task: TaskFunction) {

  let (mut client, mut conn) = init_mqtt(
    env::var("MQTT_HOST").unwrap_or_default(),
    env::var("MQTT_PORT").unwrap_or(String::from("1883")).parse::<u16>().unwrap_or(1883),
    env::var("MQTT_USERNAME").unwrap_or_default(),
    env::var("MQTT_PASSWORD").unwrap_or_default()
  );
  
  thread::spawn(move || {
    for _notif in conn.iter() {
      // doStuff()
    }
  });

  thread::spawn(move || {
    if let Ok(schedule) = Schedule::from_str(expr.as_str()) {
      let mut sys = System::new_all();
      let mut disks = Disks::new_with_refreshed_list();
      loop {
        let now = Utc::now();
        if let Some(next) = schedule.upcoming(Utc).take(1).next() {
            let pause = next - now;
            thread::sleep(pause.to_std().unwrap_or(Duration::from_millis(0)));
            task(&mut sys, &mut disks, &mut client);
        }
      }
    }
  });
}