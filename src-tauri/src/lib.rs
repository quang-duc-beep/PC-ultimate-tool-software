// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use os_info;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use sysinfo::{Disk, Disks, System};
use wmi::WMIConnection;

const BYTES_PER_GB_F64: f64 = 1024.0 * 1024.0 * 1024.0;
const BYTES_PER_GB_U64: u64 = 1024 * 1024 * 1024;

#[derive(Serialize)]
pub struct Sysinfo {
    // -- Cpu
    pub cpu_usage: f64,
    pub cpu_freq: f64,
    pub cpu_core: usize,
    pub cpu_processor: usize,
    pub cpu_uptime: u64,

    // -- Memory-RAM
    pub memory_total: u64,
    pub memory_usage: u64,
    pub memory_usage_pre: f64,

    // -- Disk OS
    pub disk_kind: String,
    pub free_space: f64,
    pub space_used: f64,

    // -- PC information
    pub name_os: String,
    pub name_pc: String,
    pub sys_type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_DiskDrive")]
#[serde(rename_all = "PascalCase")]
pub struct WmiDiskDrive {
    pub model: Option<String>,
    pub size: Option<u64>, // Byte
    pub interface_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiskWmi {
    pub model: String,
    pub size_gb: u64,
    pub interface_system: String,
}

#[tauri::command]
fn sys_info() -> Result<Sysinfo, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // -- Data CPU
    let usage_c = sys.global_cpu_usage() as f64;
    let freq_c = sys
        .cpus()
        .first()
        .map(|cpu| cpu.frequency() as f64 / 1_000.0)
        .unwrap_or(0.0);
    let core = System::physical_core_count().unwrap_or(0);
    let processor = sys.cpus().len();
    let uptime = System::uptime();

    // -- Data Memory
    let mem_total = sys.total_memory();
    let mem_usage = sys.used_memory();
    let mem_usage_pr = if mem_total > 0 {
        (mem_usage as f64 / mem_total as f64) * 100.0
    } else {
        0.0
    };

    let system_drive = std::env::var("SystemDrive")
        .unwrap_or_else(|_| "C:".to_string())
        .to_lowercase();

    let disks = Disks::new_with_refreshed_list();

    let diskinfo: (String, f64, f64, f64) = disks
        .iter()
        .find(|disk| {
            let mount: Cow<'_, str> = disk.mount_point().to_string_lossy();
            mount == "/" || mount.to_lowercase().starts_with(&system_drive)
        })
        .map(|disk: &Disk| {
            let disk_kind: String = format!("{:?}", disk.kind());
            let free_space: f64 = disk.available_space() as f64 / BYTES_PER_GB_F64;
            let total_space: f64 = disk.total_space() as f64 / BYTES_PER_GB_F64;
            let space_used: f64 = total_space - free_space;

            (disk_kind, free_space, space_used, total_space)
        })
        .unwrap_or_else(|| ("Unknown".to_string(), 0.0, 0.0, 0.0));

    // -- PC Information
    let os = os_info::get();
    let os_name: String = format!("{} {}", os.os_type(), os.edition().unwrap_or(""));
    let system_type = match os.bitness() {
        os_info::Bitness::X32 => "x86-based PC".to_string(),
        os_info::Bitness::X64 => "x64-based PC".to_string(),
        _ => "UNKNOWN".to_string(),
    };

    let pc_name = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "UNKNOWN".to_string());

    Ok(Sysinfo {
        // -- Cpu
        cpu_usage: usage_c,
        cpu_freq: freq_c,
        cpu_core: core,
        cpu_processor: processor,
        cpu_uptime: uptime,

        // -- Memory-RAM
        memory_total: mem_total,
        memory_usage: mem_usage,
        memory_usage_pre: mem_usage_pr,

        // -- Disk
        disk_kind: diskinfo.0,
        free_space: diskinfo.1,
        space_used: diskinfo.2,

        // -- PC information
        name_os: os_name,
        name_pc: pc_name,
        sys_type: system_type,
    })
}

#[tauri::command]
fn disk_wmi() -> Result<Vec<DiskWmi>, String> {
    let wmi_con = WMIConnection::new().map_err(|e| e.to_string())?;

    let raw_disks: Vec<WmiDiskDrive> = wmi_con.query().map_err(|e| e.to_string())?;

    let message: Vec<DiskWmi> = raw_disks
        .into_iter()
        .map(|item| DiskWmi {
            model: item
                .model
                .map(|m| m.trim().to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            size_gb: item.size.unwrap_or(0) / BYTES_PER_GB_U64,
            interface_system: item
                .interface_type
                .unwrap_or_else(|| "N/A".to_string()),
        })
        .collect();

    Ok(message)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![sys_info, disk_wmi])
        .run(tauri::generate_context!())
        .expect("error while running application");
}