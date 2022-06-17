
use std::process::Command;

// Linux only command no need for windows compatibility.

fn update() -> i8 {

    if cfg!(windows) {
        match Command::new("cmd").args(&["/c","python3 -m pip3 install --upgrade pip3"]).status() {
            Err(_e)         => return 1,
            Ok(_process)    => return 0,
        };
    } else {
        match Command::new("sh").args(&["-c","pip3 install --upgrade pip3"]).status() {
            Err(_e)         => return 1,
            Ok(_process)    => return 0,
        };
    }
}


pub fn checkinstallation(input: bool) -> Vec<i8> {
    if input == true {
        return vec![update()];
    } else {
        return vec![-5,-5,-5,-5];
    }
}