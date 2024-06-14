use http::{api, health};
use rocket::{futures::lock::Mutex, launch, routes};
use sysinfo::{Disks, System};

mod http;

#[launch]
fn rocket() -> _ {
    let sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    rocket::build()
    .mount("/", routes![health::health])
    .mount("/api", routes![
        api::cpu::info,
        api::disk::disks,
        api::memory::info,
        api::process::processes,
        api::system_information::info,
    ])
    .manage(Mutex::new(sys))
    .manage(Mutex::new(disks))
}
