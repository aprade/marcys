mod machine;

#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use machine::{
    cpu,
    memory
};

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

#[get("/")]
fn route_root() -> Json<&'static str> {
    Json("{\"status\": \"Operating!\"}")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        route_root,
        route_cpu,
        route_memory
    ])
}
