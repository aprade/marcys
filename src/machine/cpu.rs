use rocket::serde::Serialize;
use sys_metrics::cpu::*;

#[derive(Serialize, Debug)]
pub struct CPUInfo {
    pub frequency: f64,
    pub logical_cores: u32,
    pub physical_cores: u32,
    pub load_average: LoadAvg,
}

impl CPUInfo {
    pub fn new() -> CPUInfo {
        CPUInfo {
            frequency: get_cpufreq().unwrap(),
            logical_cores: get_logical_count().unwrap(),
            physical_cores: get_physical_count().unwrap(),
            load_average: get_loadavg().unwrap(),
        }
    }
}
