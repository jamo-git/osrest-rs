use std::process::Command;

use rocket::response::content;
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

mod lib;

#[get("/")]
fn uri_main() -> content::RawJson<&'static str> {
    content::RawJson("{ \"/osinfo\": \"Check /osinfo for System Information\", \"/uptime\": \"Check /uptime for Uptime\" }")
}

#[get("/")]
fn osinfo() -> Json<lib::OsInfo> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
                .args(["/C", "systeminfo"])
                .output()
                .expect("Failed to check systeminfo");
        let output_string = String::from_utf8(output.stdout).expect("Not valid utf-8");
        let os_data = lib::parse_os_windows(output_string);
        Json(os_data)
    }
    else if cfg!(target_os = "linux") {
        let output = Command::new("sh")
                .arg("-c")
                .arg("cat /etc/os-release")
                .output()
                .expect("Failed to check os-release");
        let output_string = String::from_utf8(output.stdout).expect("Not valid utf-8");
        let os_data = lib::parse_os_linux(output_string);
        Json(os_data)
    } 
    else {
        Json(lib::empty_return_osinfo())
    }
}

#[get("/")]
fn uptime() -> Json<lib::Uptime> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
                .args(["/C", "systeminfo"])
                .output()
                .expect("Failed to check systeminfo");
        let output_string = String::from_utf8(output.stdout).expect("Not valid utf-8");
        let uptime_data = lib::uptime_windows(output_string);
        Json(uptime_data)
    }
    else if cfg!(target_os = "linux") {
        let output = Command::new("sh")
                .arg("-c")
                .arg("cat /proc/uptime")
                .output()
                .expect("Failed to check os-release");
        let output_string = String::from_utf8(output.stdout).expect("Not valid utf-8");
        let os_data = lib::uptime_linux(output_string);
        Json(os_data)
    } 
    else {
        Json(lib::empty_return_uptime())
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![uri_main])
    .mount("/osinfo", routes![osinfo])
    .mount("/uptime", routes![uptime])
}