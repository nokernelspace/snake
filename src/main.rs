pub mod instruction;
use toml::Table;
use toml::Value;

pub fn cwd() -> String {
    let base_path = std::env::current_dir().unwrap();
    std::fs::canonicalize(&base_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn now() -> i64 {
    let unix_timestamp: i64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    unix_timestamp
}

pub fn since(timestamp1: u64, timestamp2: u64) -> u64 {
    // Convert UNIX timestamps to SystemTime
    let time1 = UNIX_EPOCH + std::time::Duration::from_secs(timestamp1);
    let time2 = UNIX_EPOCH + std::time::Duration::from_secs(timestamp2);

    // Calculate the difference
    let difference = if time1 > time2 {
        time1
            .duration_since(time2)
            .unwrap_or(std::time::Duration::from_secs(0))
    } else {
        time2
            .duration_since(time1)
            .unwrap_or(std::time::Duration::from_secs(0))
    };

    difference.as_secs()
}

pub fn cd(relative: &str) {
    let base_path = std::env::current_dir().unwrap();
    let base_path = base_path.join(relative);

    println!("CD: {:?}", base_path);
    let absolute = std::fs::canonicalize(&base_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    std::env::set_current_dir(&absolute).unwrap();
}

pub fn exists(absolute: &str) -> bool {
    if !std::fs::exists(absolute).unwrap() {
        panic!("{} does not exist", absolute);
    }
    true
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root = {
        if args.len() > 2 {
            cd(&args[2]);
            &args[2]
        } else {
            "."
        }
    };

    let config = project_toml.parse::<Table>().unwrap();
    let gekko_dir = config.get("gekko").unwrap().to_str().unwrap();
    let powerpc_dir = config.get("powerpc").unwrap().to_str().unwrap();

    let name = config.get("name").unwrap().to_str().unwrap();
    let version = config.get("version").unwrap().as_array().unwrap();
    let desc = config.get("description").unwrap().as_array().unwrap();
}
