use std::env;

use http::{api, health};
use mqtt::task;
use rocket::{futures::lock::Mutex, launch, routes};
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

    mqtt::execute(env::var("CRON_EXPRESSION").unwrap_or_default(), task::run);

    app
}
