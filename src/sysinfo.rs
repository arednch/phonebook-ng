use reqwest::blocking::get;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Host {
    name: String,
    ip: String,
}

#[derive(Debug, Deserialize)]
pub struct SysInfo {
    hosts: Vec<Host>,
}

impl SysInfo {
    // Function to create a HashMap of host names to IPs for easy lookup
    pub fn create_host_map(&self) -> HashMap<String, String> {
        let mut host_map = HashMap::new();
        for host in &self.hosts {
            if !is_numeric_only(&host.name) {
                continue; // skip regular hosts not looking like a phone
            }
            host_map.insert(host.name.clone().to_lowercase(), host.ip.clone());
        }
        host_map
    }
}

fn is_numeric_only(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10)) // 10 is the radix for decimal digits
}

pub fn load_sysinfo(source: &str) -> Result<SysInfo, Box<dyn Error>> {
    let data: String = if source.contains("://") {
        println!("loading sysinfo from web address {:?}", source);
        let response = get(source)?;
        response.text()?
    } else {
        println!("loading sysinfo from file {:?}", source);
        let data = fs::read_to_string(source)?;
        data
    };

    let sysinfo: SysInfo = serde_json::from_str(&data)?;
    Ok(sysinfo)
}
