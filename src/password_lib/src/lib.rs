use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

use sha2::{Digest, Sha512};

pub fn generate_sha512_string(string: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(string.as_bytes());
    let result = hasher.clone().finalize();
    format!("{:x}", result)
}

#[derive(Debug)]
pub enum Errors {
    TokenError,
    PasswordFailure,
    NoSuchUser,
    BadRequest,
    UserAlreadyExists,
    InternalServerError(String),
}

impl std::error::Error for Errors {}
impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<std::io::Error> for Errors {
    fn from(value: std::io::Error) -> Self {
        Errors::InternalServerError(format!("{}", value))
    }
}

pub async fn add_user(user: &str, hash: &str) -> Result<(), Errors> {
    let users = get_user_pass_map().await?;
    if users.contains_key(user) {
        return Err(Errors::UserAlreadyExists);
    }

    let mut file: std::fs::File = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("./password_list.txt")?;
    writeln!(
        file,
        "// {}:{}",
        user,
        generate_sha512_string(hash.to_string())
    )?;
    Ok(())
}

async fn get_user_pass_map() -> Result<HashMap<String, String>, Errors> {
    Ok(std::fs::read_to_string("password_list.txt")?
        .lines()
        .map(|line| {
            let entry = line
                .split_once(':')
                .expect("Infallible state reached. Cant split db entry");
            (entry.0.to_string(), entry.1.to_string())
        })
        .collect::<HashMap<String, String>>())
}

pub async fn authenticate_with_password(user: &str, pass: &str) -> Result<String, Errors> {
    let password_hash = generate_sha512_string(pass.to_string());

    let user_names = get_user_pass_map().await;
    if let Err(e) = user_names {
        return Err(e);
    }
    let user_names = user_names.unwrap();

    match user_names.get(user) {
        Some(password) => {
            if *password == password_hash {
                Ok(password_hash)
            } else {
                Err(Errors::PasswordFailure)
            }
        }
        None => Err(Errors::NoSuchUser),
    }
}

pub async fn authenticate_token(token: String) -> Result<(), Errors> {
    let x = std::fs::read_to_string("password_list.txt")?
        .lines()
        .filter(|line| !line.starts_with("//"))
        .map(|line| {
            if let Some((user, pass)) = line.split_once(":") {
                let user_pass = dbg!(format!("{}+mc+{}", user, pass));
                generate_sha512_string(user_pass)
            } else {
                panic!("Unable to read database");
            }
        })
        .collect::<HashSet<String>>();

    if x.contains(&token){
        return Ok(())
    }
    else{
        return Err(Errors::TokenError);
    }
}
