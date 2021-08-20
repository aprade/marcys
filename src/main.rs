#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

mod cpu;
mod memory;

#[get("/cpu")]
fn route_cpu() -> Json<cpu::CPUInfo> {
    let info = cpu::CPUInfo::new();
    Json(info)
}

#[get("/memory")]
fn route_memory() -> Json<memory::Memory> {
    let info = memory::new();
    Json(info)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![route_cpu, route_memory])
}
