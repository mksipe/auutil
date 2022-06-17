use std::process::Command;

// Cross system no need for windows support.

fn update() -> i8 {
    match Command::new("sh").args(&["-c","brew update"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}

fn upgrade() -> i8 {
    match Command::new("sh").args(&["-c","brew updgrade"]).status() {
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