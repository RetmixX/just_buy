pub fn hash_password(password: &String) -> String {
    pwhash::bcrypt::hash(password).unwrap()
}

pub fn verify_password(password: &String, hash: &String) -> bool {
    pwhash::unix::verify(password, hash)
}