use std::env;

pub use env::args;
use dirs;

macro_rules! declare_env {
    ($env:ident: $env_var:ident, $def:expr, $type:ty) => {
        pub fn $env() -> $type {
            match env::var("$env_var") {
                Ok(res) => res.into(),
                _ => $def.into()
            }
        }
    };
    ($env:ident: $env_var:ident, $def:expr) => {
        pub fn $env() -> String {
            match env::var("$env_var") {
                Ok(res) => res.into(),
                _ => $def.into()
            }
        }
    }
}

declare_env!(server_addr: TDT_SERVER_ADDRESS, "[::1]:50051");
declare_env!(std_out: TDT_SERVER_STDOUT, "/tmp/tdt.out");
declare_env!(std_err: TDT_SERVER_STDERR, "/tmp/tdt.err");
declare_env!(pid_file: TDT_PID_FILE, "/tmp/tdt.pid");
declare_env!(working_dir: TDT_WORKING_DIR, dirs::home_dir().unwrap().to_str().unwrap());
declare_env!(chown_pid: TDT_CHOWN_PID, "false");
declare_env!(user: TDT_USER, "nobody");
declare_env!(group: TDT_GROUP, "nobody");
declare_env!(umask: TDT_UMASK, "777");

