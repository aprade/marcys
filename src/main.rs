#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;

use crate::cpu::CPUInfo;
mod cpu;

#[get("/cpu")]
fn route_cpu() -> Json<CPUInfo> {
    let info = Json(cpu::get_cpu_info)();
    info
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![route_cpu])
}

/*fn main() {
    cpu::get_cpu_info();
}*/