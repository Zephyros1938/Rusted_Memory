pub fn _l_or_i_debug(v: Vec<String>) {
    let db: String =
        crate::zephyros1938_util::list_utils::l_or_i(Vec::from(v), ", ".to_string()).unwrap();
    println!("{:}", db)
}