use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};

pub fn encrypt(password: &str) -> Result<String, BcryptError> {
    let hashed_password = hash(password, DEFAULT_COST)?;

    Ok(hashed_password)
}

fn compare<'a>(plain: &'a str) -> impl Fn(&'a str) -> Result<bool, BcryptError> {
    move |hash| {
        let verified = verify(plain, hash)?;

        Ok(verified)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_password() {
        let password = "Super_Secret_Password";
        let hashed = encrypt(password).unwrap();
        println!("> {hashed}");

        assert!(compare(password)(&hashed).unwrap());
    }

    #[test]
    fn incorrect_password() {
        let hashed_password = "$2b$12$Fs6SKnh1cG5jL4PdSEMZ/.SK5pVMYrq9kPhtEh2dsiqjXGX4ybuPq";
        let plain_password = "Super_Secret_Password";

        assert!(!compare(plain_password)(hashed_password).unwrap());
    }
}
