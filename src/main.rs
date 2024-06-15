use std::env;

use http::{api::{self, process::model::get_all_processes, system_information::model::SystemInformation}, health};
use rocket::{futures::lock::Mutex, launch, routes, serde::json};
use rumqttc::QoS;
use sysinfo::{Disks, System};

mod http;
mod mqtt;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    let sys = Mutex::new(System::new_all());
    let disks = Mutex::new(Disks::new_with_refreshed_list());
    
    let app = rocket::build()
    .mount("/", routes![health::health])
    .mount("/api", routes![
        api::cpu::info,
        api::disk::disks,
        api::memory::info,
        api::process::processes,
        api::system_information::info,
    ])
    .manage(sys)
    .manage(disks);

    mqtt::execute(
        env::var("CRON_EXPRESSION").unwrap_or_default(), 
        |sys, disks, client| {
            sys.refresh_all();
            disks.refresh_list();
            let si = SystemInformation::new(sys, disks);
            let processes = get_all_processes(sys.processes());
            let _ = client.publish("rasp/stats", QoS::AtMostOnce, false, json::to_string(&si).unwrap_or_default());
            let _ = client.publish("rasp/processes", QoS::AtMostOnce, false, json::to_string(&processes).unwrap_or_default());
        }
    );

    app
}
