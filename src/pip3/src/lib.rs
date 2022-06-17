
use std::process::Command;

// Linux only command no need for windows compatibility.
const ERRMSG:&str = "Something went wrong";

fn update() -> i32 {

    if cfg!(windows) {
        let cmd = Command::new("cmd").args(&["/c","python3 -m pip3 install --upgrade pip3"]).status().expect(ERRMSG);
        return cmd.code().unwrap();
    } else {
        let cmd = Command::new("sh").args(&["-c","pip3 install --upgrade pip3"]).status().expect(ERRMSG);
        return cmd.code().unwrap();
    }
}


pub fn checkinstallation(input: bool) -> Vec<i32> {
    if input == true {
        return vec![update()];
    } else {
        return vec![-5,-5,-5,-5];
    }
}