pub mod math_utils {
    pub mod extra {
        pub fn sq2_vec(l: f64) -> String {
            let l_pow_in: u64 = 1 + (l as u64);
            let l_pow: u64 = 2_u64.pow(l_pow_in as u32);

            let mut length_list = [l_pow.to_string().len(), l_pow_in.to_string().len()];

            let [l_pow_name, l_pow_in_name] = ["l_pow", "l_pow_in"];
            let name_list = [l_pow_name, l_pow_in_name];

            for x in 0..name_list.len() {
                if name_list[x].len() >= length_list[x] {
                    length_list[x] = name_list[x].len();
                }
            }
            let [l_pow_len, l_pow_in_len] = length_list;

            let print_string_values: String =
                format!("| {: ^l_pow_len$} | {: ^l_pow_in_len$} |", l_pow, l_pow_in);
            let print_string_names = format!(
                "| {: ^l_pow_len$} | {: ^l_pow_in_len$} |",
                "l_pow", "l_pow_in"
            );
            let print_string_len: usize = print_string_values.len();
            let print_string_divider = format!("{:-<print_string_len$}", "");
            let print_string = format!(
                "{}\n{}\n{}\n",
                print_string_names, print_string_values, print_string_divider
            );
            return print_string;
        }
    }
}

pub mod vec_map_utils {
    pub mod table {
        fn _generate_table_values(
            vals: Vec<&Vec<String>>,
            pad: usize,
            wrap_at: usize,
        ) -> String {
            let mut str_out: String = String::new();
            for x in 0..vals.len() {
                let temp_str: String = format!(
                    "| {: ^pad$} ",
                    crate::zephyros1938_util::list_utils::l_or_i(
                        vals[x].to_vec(),
                        "_".to_string()
                    )
                    .unwrap()
                );
                str_out.insert_str(str_out.len(), &temp_str);
                if x as i32 % wrap_at as i32 == 0_i32 {
                    str_out.insert_str(str_out.len(), "\n");
                }
            }
            str_out.insert_str(str_out.len(), "|");
            str_out
        }
    }
}
