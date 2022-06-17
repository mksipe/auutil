use std::process::Command;

// Linux only command no need for windows compatibility.

fn update() -> i8 {
    match Command::new("sh").args(&["-c","yum check-update -y"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

fn upgrade() -> i8 {
    match Command::new("sh").args(&["-c","yum update -y"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

pub fn checkinstallation(input: bool) -> Vec<i8> {
    if input == true {
        return vec![update(), upgrade()];
    } else {
        return vec![-5,-5,-5,-5];
    }
}