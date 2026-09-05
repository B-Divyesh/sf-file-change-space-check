use std::fs::OpenOptions;
use std::io::Write;

#[unsafe(no_mangle)]
pub extern "C" fn socket(_domain: i32, _kind: i32, _protocol: i32) -> i32 {
    if let Ok(path) = std::env::var("FCSC_NETWORK_ATTEMPT_LOG")
        && let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path)
    {
        let _ = writeln!(file, "socket called");
    }
    -1
}
