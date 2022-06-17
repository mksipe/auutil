use std::process::Command;
const ERRMSG:&str = "Something went wrong";

fn update() -> i32 {

    let cmd = Command::new("rustup").arg("update").status().expect(ERRMSG);
    return cmd.code().unwrap();
}

pub fn checkinstallation(input: bool) -> Vec<i32> {
    if input == true {
        return vec![update()];
    } else {
        return vec![-5,-5,-5,-5];
    }
}