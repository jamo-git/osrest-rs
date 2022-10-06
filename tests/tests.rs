use osrest::{
    OsInfo, 
    parse_os_linux, 
    parse_os_windows, 
    empty_return_osinfo,
    Uptime,
    uptime_windows,
    uptime_linux,
    empty_return_uptime,
};

use regex::Regex;

#[test]
fn test_windows_parse() {
    let test_string: String = String::from("\r\nOS Name:   Testing\r\nOS Version:    0.1\r\n");
    let input: OsInfo = parse_os_windows(test_string);
    println!("{:?}", input);
    assert_eq!("Testing", input.os_name);
    assert_eq!("0.1", input.os_version);
}

#[test]
fn test_linux_parse() {
    let test_string: String = String::from("\nNAME=Testing\nVERSION=0.1\n");
    let input: OsInfo = parse_os_linux(test_string);
    println!("{:?}", input);
    assert_eq!("Testing", input.os_name);
    assert_eq!("0.1", input.os_version);
}

#[test]
fn test_empty_osinfo() {
    let input: OsInfo = empty_return_osinfo();
    println!("{:?}", input);
    assert_eq!("n/a", input.os_name);
    assert_eq!("n/a", input.os_version);
}

#[test]
fn test_win_uptime() {
    let test_string: String = String::from("\r\nSystem Boot Time:   01/01/2000, 12.00.00\r\n");
    let input: Uptime = uptime_windows(test_string);
    println!("{:?}", input);
    let re_minutes = Regex::new(r"^\d+$").unwrap();
    assert!(re_minutes.is_match(input.uptime.as_str()));
    assert_eq!("minutes", input.value_type);
}

#[test]
fn test_linux_uptime() {
    let test_string: String = String::from("80121.012 1381301.12");
    let input: Uptime = uptime_linux(test_string);
    println!("{:?}", input);
    let re_minutes = Regex::new(r"^\d+$").unwrap();
    assert!(re_minutes.is_match(input.uptime.as_str()));
    assert_eq!("minutes", input.value_type);
}


#[test]
fn test_empty_uptime() {
    let input: Uptime = empty_return_uptime();
    println!("{:?}", input);
    assert_eq!("n/a", input.uptime);
    assert_eq!("n/a", input.value_type);
}
