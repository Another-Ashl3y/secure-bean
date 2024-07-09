use std::io::{stdin,stdout,Write};
use std::{fs, fs::File, path::Path};
use pwhash::bcrypt;
use rpassword::read_password;

const P_DIR: &str = "../manager/bobby_002.shy";
const P_ADD: &str = "catcatcutecat";
const CK_DIR: &str = "/manager/bobby_003.shy";
const C_DIR: &str = "/manager/bobby_004.shy";
const CK_ADD: &str = "asdwjijsaidujwcascknsadowqjdsakldjhfeaowqlsakjdxlksjcosafqhwrqiwd";

fn main() {
    let real_password = get_password();
    print!("Welcome\nEnter password>");
    stdout().flush().unwrap();
    let inp = P_ADD.to_owned()+&read_password().unwrap();
    if bcrypt::verify(inp.clone(), &real_password) {
        println!("Logged in.");
        loop {
            print!("> ");
            let command = &input() as &str;
            match command {
                "change_password" => {
                    print!("Enter new password> ");
                    stdout().flush().unwrap();
                    let new_password = read_password().unwrap();
                    print!("Confirm new password> ");
                    stdout().flush().unwrap();
                    let confirm_password = read_password().unwrap();
                    if new_password == confirm_password {
                        let mut p_file = File::create(P_DIR).unwrap();
                        p_file.write_all(&bcrypt::hash(P_ADD.to_owned()+&new_password).unwrap().into_bytes()).unwrap();
                    }
                },
                "exit" => break,
                _=>println!("Unknown command")
            }
        }
    } else {
        println!("Password incorrect. Exiting now...");
    }
}

fn get_ck() {
    let mut contents = "";
    if Path::new(CK_DIR).exists() {
        contents = fs::read_to_string(CK_DIR).unwrap();
    } else {
        println!("Car keys not found. Generating new car keys.");
    }
}





fn get_password() -> String {
    let mut contents = "$2b$10$rzlLhyNfyXhVsf0TdHbgYO3Zgi08Gmr7kdmTvAbTERbQernEnJmWO".to_string();
    if Path::new(P_DIR).exists() {
        contents = fs::read_to_string(P_DIR).unwrap();
    } else {
        println!(" --Warning--\nPotential security breach.\nNo password lock found.");
    }
    contents
}


fn input() -> String {
    stdout().flush().unwrap();
    let mut s=String::new();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}



