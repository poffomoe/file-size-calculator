use std::fs::read_dir;
use std::path::Path;

fn main() {
    let path_to_file: &Path = Path::new("test_folder");

    let file_size: u64 = calculate_size(path_to_file);
    println!("{} Bytes", file_size);

    let (converted_file_size, unit): (f64, String) = to_highest_prefix(file_size);
    println!("{:.2} {}B", converted_file_size, unit);
}

fn calculate_size(path: &Path) -> u64 {
    if path.to_str().unwrap() == "" {
        panic!("the path is empty")
    };
    
    let mut sum: u64 = 0;
    if path.is_dir() {
        match read_dir(path) {
            Ok(unwrapped_dir) => {
                for entry in unwrapped_dir {
                    if let Ok(dir_entry) = entry {
                        let buf = dir_entry.path();
                        let entry_path: &Path = buf.as_path();
                        sum += calculate_size(entry_path);
                    } else {
                        sum += 0;
                    }
                }
            },
            Err(_) => sum += 0
        }
        sum += path.metadata().unwrap().len();
    } else {
        match path.metadata() {
            Ok(file_metadata) => sum += file_metadata.len(),
            Err(the_error) => {panic!("[Path: {}] {}", path.to_str().unwrap(), the_error)}
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