use std::process::Command;

// Linux only command no need for windows compatibility.
const ERRMSG:&str = "Something went wrong";

fn update() -> i32 {
    let cmd = Command::new("sh").args(&["-c","dnf update --refresh"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

fn cache() -> i32 {
    let cmd = Command::new("sh").args(&["-c","dnf makecache"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

fn upgrade() -> i32 {
    let cmd = Command::new("sh").args(&["-c","dnf upgrade --refresh"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

pub fn checkinstallation(input: bool) -> Vec<i32> {
    if input == true {
        return vec![update(), cache(), upgrade()];
    } else {
        return vec![-5,-5,-5,-5];
    }
}