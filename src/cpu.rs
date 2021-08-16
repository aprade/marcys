use sys_metrics::cpu::*;

#[derive(Debug)]
struct CPUInfo {
    frequency: f64,
    logical_cores: u32,
    physical_cores: u32,
    load_average: LoadAvg,
}

pub fn get_cpu_info() {
    let info = CPUInfo {
        frequency: get_cpufreq().unwrap(),
        logical_cores: get_logical_count().unwrap(),
        physical_cores: get_physical_count().unwrap(),
        load_average: get_loadavg().unwrap(),
    };

    println!("{:#?}", info);
}