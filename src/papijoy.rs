pub mod papijoy {
    use std::{
        fs::{self, OpenOptions},
        io::{LineWriter, Write},
        path::PathBuf,
        thread,
    };

    use colored::Colorize;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref ALPHABETS: [&'static str; 26] = [
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "k", "j", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z"
        ];
        pub static ref SPLITOR: String = String::from("@");
    }

    pub fn set_property(key: &str, value: &str, path_name: &str) {
        if !PathBuf::from("config.dcnf").exists() {
            let file = OpenOptions::new()
                .append(true)
                .write(true)
                .open(path_name)
                .unwrap();
            let mut file = LineWriter::new(file);
            let line_to = format!("{}{}{}\n", key, &*SPLITOR, value);
            let proc = thread::spawn(move || {
                file.write(line_to.as_bytes()).unwrap();
            });
            proc.join().unwrap_or_else(|x| {panic!("")});
        }
    }

    pub fn get_property(key: &str, value: &String, path_name: &str) -> String {
        if !PathBuf::from(path_name).exists() {
            return value.to_string();
        }

        let binding = fs::read_to_string(path_name).unwrap_or_else(|_m| {
            panic!("{}", "wtf".on_red());
        });

        let lines = binding.lines();

        for line in lines {
            let splited: Vec<&str> = line.split(&*SPLITOR).collect();
            if splited[0].eq(key) {
                if splited.len() == 2 {
                    return splited[1].to_string();
                } else {
                    return value.to_string();
                }
            }
        }
        return value.to_string();
    }
}
