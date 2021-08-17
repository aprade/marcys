use serde::{Serialize};
use rocket_contrib::json::Json;
use sys_metrics::cpu::*;


#[derive(Serialize, Debug)]
pub struct CPUInfo {
    pub frequency: f64,
    pub logical_cores: u32,
    pub physical_cores: u32,
    pub load_average: LoadAvg,
}

pub fn get_cpu_info() -> Json<CPUInfo> {
    let info = CPUInfo {
        frequency: get_cpufreq().unwrap(),
        logical_cores: get_logical_count().unwrap(),
        physical_cores: get_physical_count().unwrap(),
        load_average: get_loadavg().unwrap(),
    };
    println!("{:#?}", info);

    let serilized_info= serde_json::to_string(&info).unwrap();
    serilized_info
}