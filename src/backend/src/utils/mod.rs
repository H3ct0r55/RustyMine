pub fn clean_error<E: ToString>(err: E) -> String {
    err.to_string().replace('\n', " ").trim().to_string()
}
