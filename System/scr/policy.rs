pub fn should_protect(path: &str) -> bool {
    path.starts_with("/home")
}
