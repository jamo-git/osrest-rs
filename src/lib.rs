use regex::Regex;
use rocket::serde::Serialize;
use chrono::{offset, NaiveDateTime};

#[derive(Serialize, Debug)]
pub struct OsInfo {
    pub os_name: String,
    pub os_version: String
}

#[derive(Serialize, Debug)]
pub struct Uptime {
    pub uptime: String,
    pub value_type: String
}

pub fn uptime_windows(windata: String) -> Uptime {
    let mut uptime_data: Uptime = Uptime { 
        uptime: String::from(""), 
        value_type: String::from("")
    };
    let re_boottime = Regex::new(r"(?m)^System Boot Time:\s+(.+)\r\n").unwrap();
    let boottime = re_boottime.captures(windata.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let naivedatetime_from = NaiveDateTime::parse_from_str(boottime,
        "%d/%m/%Y, %H.%M.%S").unwrap();
    let nowtime = offset::Local::now().naive_local();
    let duration = nowtime.signed_duration_since(naivedatetime_from);
    uptime_data.uptime = duration.num_minutes().to_string();
    uptime_data.value_type = String::from("minutes");
    return uptime_data;
}

pub fn uptime_linux(linuxdata: String) -> Uptime {
    let mut uptime_data: Uptime = Uptime { 
        uptime: String::from(""), 
        value_type: String::from("")
    };

    let data_vec: Vec<&str> = linuxdata.split(" ").collect();
    let raw_seconds_float: f32 = data_vec[0].parse().unwrap();
    let minutes_int: i64 = (raw_seconds_float / 60.0) as i64;

    uptime_data.uptime = minutes_int.to_string();
    uptime_data.value_type = String::from("minutes");

    return  uptime_data;
}

pub fn parse_os_windows(windata: String) -> OsInfo {
    let mut sys_data = OsInfo { 
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

pub fn parse_os_linux(linuxdata: String) -> OsInfo {
    let mut sys_data = OsInfo { 
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

pub fn empty_return_osinfo() -> OsInfo {
    let sys_data = OsInfo { 
        os_name: String::from("n/a"),
        os_version: String::from("n/a")
    };
    return sys_data;
}

pub fn empty_return_uptime() -> Uptime {
    let uptime_data = Uptime { 
        uptime: String::from("n/a"),
        value_type: String::from("n/a")
    };
    return uptime_data;
}