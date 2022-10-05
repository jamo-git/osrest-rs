use osrest::{SysInfo, parse_linux, parse_windows, empty_return};

#[test]
fn test_windows_parse() {
    let test_string: String = String::from("\r\nOS Name:   Testing\r\nOS Version:    0.1\r\n");
    let input: SysInfo = parse_windows(test_string);
    println!("{:?}", input);
    assert_eq!("Testing", input.os_name);
    assert_eq!("0.1", input.os_version);
}

#[test]
fn test_linux_parse() {
    let test_string: String = String::from("\nNAME=Testing\nVERSION=0.1\n");
    let input: SysInfo = parse_linux(test_string);
    println!("{:?}", input);
    assert_eq!("Testing", input.os_name);
    assert_eq!("0.1", input.os_version);
}

#[test]
fn test_empty_return() {
    let input: SysInfo = empty_return();
    println!("{:?}", input);
    assert_eq!("n/a", input.os_name);
    assert_eq!("n/a", input.os_version);
}