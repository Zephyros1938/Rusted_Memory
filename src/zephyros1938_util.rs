macro_rules! _var_name {
    ($var:ident) => {
        stringify!($var)
    };
}
pub mod vec_map_utils {
    pub mod vec {
        use std::iter;
        pub fn fill_vec(fill: i32, size: usize) -> Vec<i32> {
            return iter::repeat(fill).take(size).collect();
        }
        pub fn sum_vec(vec: Vec<i32>) -> i32 {
            let mut sum: i32 = 0;
            for n in vec.iter() {
                sum = sum + n;
            }
            return sum;
        }
    }
    pub mod table {
        use crate::zephyros1938_util::list_utils;

        pub fn table_vec_ss(a: Vec<(Vec<String>, Vec<String>)>, min_pad: usize, wrap_at: usize) {
            let mut largest_pad: usize = min_pad;
            let mut out_str: String = String::new();
            let mut keys: Vec<&Vec<String>> = vec![];
            let mut vals: Vec<&Vec<String>> = vec![];

            for (key, val) in a.iter() {
                vals.push(val);
                keys.push(key);
            }

            for x in 0..keys.len() {
                let x_val = list_utils::l_or_i(vals[x].to_vec(), "_".to_string()).unwrap();
                if x_val.len() > largest_pad {
                    largest_pad = x_val.len();
                }
            }

            let content_str = generate_table_content(
                keys.clone(),
                vals.clone(),
                largest_pad,
                wrap_at,
                keys.len(),
            );

            out_str.insert_str(out_str.len(), &content_str);
            out_str = out_str[0..out_str.len() - 2].to_string();
            out_str.insert_str(out_str.len(), "\n");
            println!("{}", out_str)
        }

        fn generate_table_content(
            keys: Vec<&Vec<String>>,
            vals: Vec<&Vec<String>>,
            pad: usize,
            wrap_at: usize,
            size: usize,
        ) -> String {
            let mut str_out: String = String::new();
            let mut val_str: String = String::new();
            let mut can_break_first: bool = true;
            for x in 0..keys.len() {
                let temp_str: String = format!(
                    "| {: ^pad$}",
                    list_utils::l_or_i(keys[x].to_vec(), ", ".to_string()).unwrap()
                );
                str_out.insert_str(str_out.len(), &temp_str);

                val_str.insert_str(
                    val_str.len(),
                    &format!(
                        "| {: ^pad$}",
                        list_utils::l_or_i(vals[x].to_vec(), ", ".to_string()).unwrap()
                    ),
                );
                if x as i32 % wrap_at as i32 == wrap_at as i32 - 1 || x == (size) - 1 {
                    let padl = val_str.len() + 1;
                    let break_str: String = format!("|\n{:-^padl$}\n", "");
                    if can_break_first {
                        let break_str: String = format!("{:-^padl$}\n", "");
                        str_out.insert_str(0, &break_str);
                        can_break_first = false;
                    }
                    str_out.insert_str(str_out.len(), "|\n");
                    str_out.insert_str(str_out.len(), &val_str);
                    val_str.clear();
                    str_out.insert_str(str_out.len(), &break_str);
                }
            }
            str_out.insert_str(str_out.len(), "|");
            str_out
        }
    }
}
pub mod math_utils {
    pub mod extra {
        pub fn sq2_str(l: f64) -> String {
            let l_pow_in: u64 = 1 + (l as u64);
            let l_pow: u64 = 2_u64.pow(l_pow_in as u32);

            return l_pow.to_string();
        }
    }
}

pub mod list_utils {
    use std::io::{Error, ErrorKind};

    pub fn l_or_i(a: Vec<String>, b: String) -> Result<String, Error> {
        if a.len() <= 0 {
            Err(Error::new(ErrorKind::InvalidData, "Input"))
        } else if a.len() == 1 {
            Ok(a.iter().map(|f| f.to_string()).collect::<String>())
        } else {
            let string_out: String = a
                .iter()
                .map(|f: &String| f.to_string() + &b.to_string())
                .collect::<String>();
            Ok(string_out[0..string_out.len() - b.len()].to_string())
        }
    }
}
