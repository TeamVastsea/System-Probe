use chrono::{Local, NaiveDateTime, TimeZone};
use sysinfo::{CpuExt, DiskExt, NetworkExt, System, SystemExt};
use crate::monitor::{MachineInfo};

pub fn refresh() -> MachineInfo {
    let mut sys = System::new_all();

    sys.refresh_all();

    let mut total_received = 0;
    let mut total_transmitted = 0;

    for (_, data) in sys.networks() {
        total_received += data.received();
        total_transmitted += data.transmitted()
    }

    let mut available_space: f32 = 0_f32;
    for disk in sys.disks() {
        available_space += disk.available_space() as f32;
    }
    available_space /= 1024_f32 * 1024_f32 * 1024_f32;

    let memory = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100_f32;

    let mut cpu_total = 0 as f32;
    for cpu in sys.cpus() {
        cpu_total += cpu.cpu_usage();
    }
    cpu_total /= sys.cpus().len() as f32;

    MachineInfo {
        free_space: available_space,
        cpu: cpu_total,
        memory,
        upload: total_transmitted,
        download: total_received,
        boot_time: Local.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(sys.boot_time() as i64, 0).unwrap()).format("%Y-%m-%d %H:%M:%S").to_string(),
    }
}