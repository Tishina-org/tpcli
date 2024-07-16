use std::{fmt::Error, io};
use regex::{Match, Regex};
use bstr::ByteSlice;
use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    println!("\x1b[93mWelcome to the KP CLI\x1b[0m");

    let stdin = io::stdin();
    let re = Regex::new(r"^(https?:///*)?([a-z\d-]+\.[a-z\d]+)([a-z\d-]+\.)*/?(\S+)?$").unwrap();

    println!("Enter URL: ");
    let mut input = String::new();
    
    stdin.read_line(&mut input);

    let mut url = String::from("");

    // bruh moment but &input without trim function doesnt work
    if let Some(caps) = re.captures(&input.trim()) {
        if let Some(domain) = caps.get(2) {
            url = get_url(domain);
        }
    } else {
        panic!("Invalid URL");
    }

    modify_gd(&url.as_str());
}

fn get_url(domain: Match) -> String {
    let mut url = format!("https://{}/", domain.as_str());
    let url_chars = url.chars().count();

    if url_chars < 26 {
        url.push_str(format!("{}/database", "/".repeat(25 - url_chars)).as_str());
    } else {
        url.push_str("database");
    }

    return url;
}

fn modify_gd(url: &str) {
    let gd = File::open("./GeometryDash.exe");
    
    match gd {
        Ok(mut file) => {            
            let mut bts: Vec<u8> = vec![];
            file.read_to_end(&mut bts).unwrap();
            let new = &bts.replace("https://www.boomlings.com/database", url);
            fs::write("./Patched.exe", &new).unwrap();
            println!("Successfully patched!");
        },
        Err(_) => panic!("GeometryDash.exe not found!"),
    };
}
