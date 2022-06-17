
use std::process::Command;


// Cross system. 
const ERRMSG:&str = "Something went wrong";


fn update() -> i32 {
    let cmd = Command::new("gem").args(&["update","--verbose"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

pub fn checkinstallation(input: bool) -> Vec<i32> {
    if input == true {
        return vec![update()];
    } else {
        return vec![-5,-5,-5,-5];
    }
}