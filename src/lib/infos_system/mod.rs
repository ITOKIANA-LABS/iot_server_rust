use serde::{Serialize, Deserialize};
use mac_address::get_mac_address;

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoSystem {
    pub ip: String,
    pub mac: String,
    pub sys_infos: String,
}

pub fn gather_system_info() -> Result<InfoSystem, Box<dyn std::error::Error>> {
    let command_ip = "curl";
    let args_ip = Some(&["ipinfo.io/ip"][..]);

    let command_os_version = "hostnamectl";
    let args_os_version = None;

    let mut infos_system = InfoSystem { ip: String::new(), mac: String::new(), sys_infos: String::new() };

    infos_system.ip = crate::lib::cmd_linux::function(command_ip, args_ip)?.trim().to_string();
    infos_system.sys_infos = crate::lib::cmd_linux::function(command_os_version, args_os_version)?.trim().to_string();

    match get_mac_address()? {
        Some(ma) => infos_system.mac = ma.to_string(),
        None => println!("No MAC address found."),
    }

    Ok(infos_system)
}