pub fn _runner_1() {
    for n in 0_i32..63_i32 {
        let n2: f64 = n as f64;
        let nm2: String = crate::zephyros1938_util_old::math_utils::extra::sq2_vec(n2);
        println!("{}", nm2);
    }
}

pub fn _runner_2() {
    let dist: usize = 63;
    let mut vec: Vec<(Vec<String>, Vec<String>)> = vec![];
    for i in 1..dist {
        let f1a: String = format!("{}", i);
        let f2a: String = format!("{}", crate::zephyros1938_util::math_utils::extra::sq2_str(i as f64));
        let string: (Vec<String>, Vec<String>) = (vec![f1a], vec![f2a]);
        vec.insert(i - 1, string);
    }
    crate::zephyros1938_util::vec_map_utils::table::table_vec_ss(vec,20,3);
}