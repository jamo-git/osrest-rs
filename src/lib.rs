use regex::Regex;
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SysInfo {
    pub os_name: String,
    pub os_version: String
}

pub fn parse_windows(windata: String) -> SysInfo {
    let mut sys_data = SysInfo { 
        os_name: String::from(""),
        os_version: String::from("")
    };
    let re_osname = Regex::new(r"(?m)^OS Name:\s+(.+)\r\n").unwrap();
    let re_osversion = Regex::new(r"(?m)^OS Version:\s+(.+)\r\n").unwrap();
    sys_data.os_name = re_osname.captures(windata.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();
    sys_data.os_version = re_osversion.captures(windata.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();
    return sys_data;
}

pub fn parse_linux(linuxdata: String) -> SysInfo {
    let mut sys_data = SysInfo { 
        os_name: String::from(""),
        os_version: String::from("")
    };
    let re_osname = Regex::new(r"(?m)^NAME=(.+)\n").unwrap();
    let re_osversion = Regex::new(r"(?m)^VERSION=(.+)\n").unwrap();
    sys_data.os_name = re_osname.captures(linuxdata.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
        .replace("\"", "");
    sys_data.os_version = re_osversion.captures(linuxdata.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
        .replace("\"", "");
    return sys_data;
}

pub fn empty_return() -> SysInfo {
    let sys_data = SysInfo { 
        os_name: String::from("n/a"),
        os_version: String::from("n/a")
    };
    return sys_data;
}