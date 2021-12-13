
use std::process::Command;

// Linux only command no need for windows compatibility.

fn update() -> i8 {
    match Command::new("sh").args(&["-c","pip3 install --upgrade pip"]).status() {
        Err(_e)         => return 1,
        Ok(_process)    => return 0,
    };
}


pub fn checkinstallation(input: bool) {
    if input == true {
        capresult();
    }
}


fn capresult() -> [i8; 1] {
    let piparr: [i8; 1] = [update()];
    return piparr
}
