use std::process::Command;

// Linux only command no need for windows compatibility.
const ERRMSG:&str = "Something went wrong";

fn update() -> i32 {
    let cmd = Command::new("sh").args(&["-c","apt update -y"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

fn upgrade() -> i32 {
    let cmd = Command::new("sh").args(&["-c","apt install metasploit-framework -y"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

fn reinitdb() -> i32 {
    let cmd = Command::new("sh").args(&["-c","msfdb reinit"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

fn autoremove() -> i32 {
    let cmd = Command::new("sh").args(&["-c","apt autoremove -y"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}


fn autoclean() -> i32 {
    let cmd = Command::new("sh").args(&["-c","apt autoclean -y"]).status().expect(ERRMSG);
    return cmd.code().unwrap();
}

pub fn checkinstallation(input: bool) -> Vec<i32> {
    if input == true {
        return vec![update(), upgrade(), reinitdb() ,autoremove(),autoclean()];
    } else {
        return vec![-5,-5,-5,-5];
    }
}