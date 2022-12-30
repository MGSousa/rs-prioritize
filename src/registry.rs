use std::{io::{Result, ErrorKind}, path::{Path, PathBuf}, process::exit};
use winapi::um::winreg::HKEY_CURRENT_USER;
use winreg::RegKey;

const PROGRAM_PATH: &str = "Software";
const PROGRAM_FOLDER: &str = "RsTop";
const PROGRAM_KEY: &str = "LastProgram";

pub struct Registry {
    reg: RegKey,
    path: PathBuf,
}

impl Registry {
    pub fn new() -> Self {
        let reg = RegKey::predef(HKEY_CURRENT_USER);
        let path = Path::new(PROGRAM_PATH).join(PROGRAM_FOLDER);

        Self {
            reg,
            path
        }
    }

    pub fn create(&self) -> RegKey {
        let (reg, _) = self.reg.create_subkey(&self.path).unwrap_or_else(
            | e | match e.kind() {
                ErrorKind::PermissionDenied => {
                    println!("Access denied - {}", e.to_string());
                    exit(1)
                },
                ErrorKind::OutOfMemory => {
                    println!("OOM - {}", e.to_string());
                    exit(135)
                },
                _ => panic!("Other write error: {:?}", e)
            });
        reg
    }

    pub fn get(&self) -> Result<String> {
        let key = self.reg.open_subkey(&self.path).unwrap_or_else(
            | e | match e.kind() {
                ErrorKind::NotFound => {
                    println!("Key not found - {}", e.to_string());
                    exit(1)
                },
                ErrorKind::PermissionDenied => {
                    println!("Access denied - {}", e.to_string());
                    exit(1)
                },
                _ => panic!("Other error: {:?}", e),
            }
        );
        let res: String = key.get_value(PROGRAM_KEY)?;
        Ok(res)
    }

    pub fn set(&self, val: String) -> Result<()> {
        let res = self.create().set_value(PROGRAM_KEY, &val)?;
        Ok(res)
    }

    pub fn delete(&self) -> Result<()> {
        let res = self.reg.delete_subkey(&self.path).unwrap_or_else(
            | e | match e.kind() {
                ErrorKind::PermissionDenied => {
                    println!("Access denied when trying remove RegKey - {}", e.to_string());
                },
                _ => panic!("Other error: {:?}", e),
            }
        );
        Ok(res)
    }
}
