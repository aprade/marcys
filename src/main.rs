#[macro_use] extern crate rocket;

mod cpu;

use rocket::serde::{json::Json};

#[get("/cpu")]
fn route_cpu() -> Json<cpu::CPUInfo> {
    let info = cpu::get_cpu_info();
    Json(info);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![route_cpu])
}

/*fn main() {
    cpu::get_cpu_info();
}*/