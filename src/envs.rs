use std::env;

pub use env::args;
use dirs;

macro_rules! declare_env {
    ($env:ident: $env_var:ident, $def:expr, $type:ty) => {
        pub fn $env() -> $type {
            match env::var("TDT_$env_var") {
                Ok(res) => res.into(),
                _ => $def.into()
            }
        }
    };
    ($env:ident: $env_var:ident, $def:expr) => {
        pub fn $env() -> String {
            match env::var("TDT_$env_var") {
                Ok(res) => res,
                _ => $def.to_string()
            }
        }
    }
}

declare_env!(server_addr: SERVER_ADDRESS, "[::1]:50051");
declare_env!(std_out: SERVER_STDOUT, "/tmp/tdt.out");
declare_env!(std_err: SERVER_STDERR, "/tmp/tdt.err");
declare_env!(pid_file: PID_FILE, "/tmp/tdt.pid");
declare_env!(working_dir: WORKING_DIR, dirs::home_dir().unwrap().to_str().unwrap());
declare_env!(chown_pid: CHOWN_PID, "false");
declare_env!(user: USER, "nobody");
declare_env!(group: GROUP, "nobody");
declare_env!(umask: UMASK, "777");


