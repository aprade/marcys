#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

mod cpu;

#[get("/cpu")]
fn route_cpu() -> Json<cpu::CPUInfo> {
    let info = cpu::get_cpu_info();
    Json(info)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![route_cpu])
}