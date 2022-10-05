use std::process::Command;

use rocket::response::content;
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

mod lib;

#[get("/")]
fn uri_main() -> content::RawJson<&'static str> {
    content::RawJson("{ \"/sysinfo\": \"Check /sysinfo for System Information\" }")
}

#[get("/")]
fn sysinfo() -> Json<lib::SysInfo> {
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
                .args(["/C", "systeminfo"])
                .output()
                .expect("Failed to check systeminfo");
        let output_string = String::from_utf8(output.stdout).expect("Not valid utf-8");
        let sys_data = lib::parse_windows(output_string);
        Json(sys_data)
    }
    else if cfg!(target_os = "linux") {
        let output = Command::new("sh")
                .arg("-c")
                .arg("cat /etc/os-release")
                .output()
                .expect("Failed to check os-release");
        let output_string = String::from_utf8(output.stdout).expect("Not valid utf-8");
        let sys_data = lib::parse_linux(output_string);
        Json(sys_data)
    } 
    else {
        Json(lib::empty_return())
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![uri_main])
    .mount("/sysinfo", routes![sysinfo])
}