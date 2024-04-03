pub mod papijoy {
    use std::{
        fs::{self, OpenOptions},
        io::{LineWriter, Write},
        path::PathBuf,
        thread,
    };

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
            proc.join().unwrap();
        }
    }

    pub fn get_property(key: &str, value: &str, path_name: &str) -> String {
        let binding = fs::read_to_string(path_name).unwrap();
        let lines = binding.lines();
        let mut retval = String::new();
        lines.into_iter().filter(|x| x.contains(key)).for_each(|x| {
            let splsize = x.split(&*SPLITOR).count();
            let splited: Vec<&str> = x.split(&*SPLITOR).collect();
            if splsize < 2 {
                retval = (&splited[1]).to_string();
            } else {
                retval = value.to_string();
            }
        });
        return retval;
    }
}
