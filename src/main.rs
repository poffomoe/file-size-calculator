use std::fs::read_dir;
use std::path::Path;
use std::env::args;

fn main(){
    let args: Vec<String> = args().collect();

    let path_to_file: &Path = Path::new(
        match args.len() {
            0 => {println!("0 args? how?.."); return;},
            1 => ".",
            2 => args[1].as_str(),

            _ => {println!("too many arguments!!!! exiting..."); return;}
        }
    );
    if !path_to_file.exists() {
        println!("the specified directory or file does not exist");
        return;
    }

    let file_size: u64 = calculate_size(path_to_file);
    println!("{} Bytes", file_size);

    let (converted_file_size, unit): (f64, String) = to_highest_prefix(file_size);
    println!("{:.2} {}B", converted_file_size, unit);
}

fn calculate_size(path: &Path) -> u64 {
    if !path.exists() { return 0; }

    let mut sum: u64 = 0;
    if path.is_dir() {
        match read_dir(path) {
            Ok(unwrapped_dir) => {
                for entry in unwrapped_dir {
                    if let Ok(dir_entry) = entry {
                        let buf = dir_entry.path();
                        let entry_path: &Path = buf.as_path();
                        sum += calculate_size(entry_path);
                    }
                }
            }, _ => ()
        }
        sum += path.metadata().unwrap().len();
    } else {
        match path.metadata() {
            Ok(file_metadata) => if !file_metadata.is_symlink() {
                sum += file_metadata.len()
            }, _ => ()
        };
    }

    sum
}

fn to_highest_prefix(bytes: u64) -> (f64, String) {
    let units: [(u64, &str); 5] = [
        (1024_u64.pow(4), "Ti"),
        (1024_u64.pow(3), "Gi"),
        (1024_u64.pow(2), "Mi"),
        (1024, "Ki"),
        (1, "")
    ];

    let mut return_tuple: (f64,String) = (0_f64, "".to_string());
    for (power, unit) in units {
        if bytes >= power {
            return_tuple = (bytes as f64 / power as f64, unit.to_string());
            break;
        }
    }

    return_tuple
}