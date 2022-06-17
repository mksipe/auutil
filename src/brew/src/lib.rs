use std::process::Command;
const ERRMSG:&str = "Something went wrong";

// Cross system no need for windows support.

fn update() -> i32 {
    let cmd = Command::new("sh").args(&["-c","brew update"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

fn upgrade() -> i32 {
    let cmd =  Command::new("sh").args(&["-c","brew upgrade"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

pub fn checkinstallation(input: bool) -> Vec<i32> {
    if input == true {
        return vec![update(), upgrade()];
    } else {
        return vec![-5,-5,-5,-5];
    }
}