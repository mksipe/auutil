use std::process::Command;

// Linux only command no need for windows compatibility.
const ERRMSG:&str = "Something went wrong";

fn update() -> i32 {
    let cmd = Command::new("sh").args(&["-c","emaint --auto sync"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

fn upgrade() -> i32 {
    let cmd = Command::new("sh").args(&["-c","emerge --ask --verbose --update --deep --changed-use @world"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

pub fn checkinstallation(input: bool) -> Vec<i32> {
    if input == true {
        return vec![update(), upgrade()];
    } else {
        return vec![-5,-5,-5,-5];
    }
}
