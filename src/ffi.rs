// Cryo FFI Module - Foreign Function Interface
// Load and call functions from dynamic libraries (.dll/.so)

#![allow(dead_code)]

use libloading::{Library, Symbol};
use std::collections::HashMap;

/// Loaded dynamic libraries
pub struct FfiManager {
    libraries: HashMap<String, Library>,
}

impl FfiManager {
    pub fn new() -> Self {
        FfiManager {
            libraries: HashMap::new(),
        }
    }
    
    /// Load a dynamic library (.dll on Windows, .so on Linux)
    pub fn load_library(&mut self, name: &str) -> Result<(), String> {
        if self.libraries.contains_key(name) {
            return Ok(()); // Already loaded
        }
        
        // Try different naming conventions
        let lib_names = if cfg!(windows) {
            vec![
                format!("{}.dll", name),
                format!("lib{}.dll", name),
                name.to_string(),
            ]
        } else {
            vec![
                format!("lib{}.so", name),
                format!("{}.so", name),
                name.to_string(),
            ]
        };
        
        for lib_name in &lib_names {
            match unsafe { Library::new(lib_name) } {
                Ok(lib) => {
                    self.libraries.insert(name.to_string(), lib);
                    return Ok(());
                }
                Err(_) => continue,
            }
        }
        
        Err(format!("Failed to load library: {}", name))
    }
    
    /// Call a function with i64 arguments and i64 return
    pub fn call_i64(&self, lib_name: &str, func_name: &str, args: &[i64]) -> Result<i64, String> {
        let lib = self.libraries.get(lib_name)
            .ok_or_else(|| format!("Library not loaded: {}", lib_name))?;
        
        unsafe {
            match args.len() {
                0 => {
                    let func: Symbol<extern "C" fn() -> i64> = lib.get(func_name.as_bytes())
                        .map_err(|e| format!("Function not found: {} ({})", func_name, e))?;
                    Ok(func())
                }
                1 => {
                    let func: Symbol<extern "C" fn(i64) -> i64> = lib.get(func_name.as_bytes())
                        .map_err(|e| format!("Function not found: {} ({})", func_name, e))?;
                    Ok(func(args[0]))
                }
                2 => {
                    let func: Symbol<extern "C" fn(i64, i64) -> i64> = lib.get(func_name.as_bytes())
                        .map_err(|e| format!("Function not found: {} ({})", func_name, e))?;
                    Ok(func(args[0], args[1]))
                }
                3 => {
                    let func: Symbol<extern "C" fn(i64, i64, i64) -> i64> = lib.get(func_name.as_bytes())
                        .map_err(|e| format!("Function not found: {} ({})", func_name, e))?;
                    Ok(func(args[0], args[1], args[2]))
                }
                _ => Err("FFI: Too many arguments (max 3)".to_string()),
            }
        }
    }
    
    /// Call a function with f64 arguments and f64 return (for math libs)
    pub fn call_f64(&self, lib_name: &str, func_name: &str, args: &[f64]) -> Result<f64, String> {
        let lib = self.libraries.get(lib_name)
            .ok_or_else(|| format!("Library not loaded: {}", lib_name))?;
        
        unsafe {
            match args.len() {
                1 => {
                    let func: Symbol<extern "C" fn(f64) -> f64> = lib.get(func_name.as_bytes())
                        .map_err(|e| format!("Function not found: {} ({})", func_name, e))?;
                    Ok(func(args[0]))
                }
                2 => {
                    let func: Symbol<extern "C" fn(f64, f64) -> f64> = lib.get(func_name.as_bytes())
                        .map_err(|e| format!("Function not found: {} ({})", func_name, e))?;
                    Ok(func(args[0], args[1]))
                }
                _ => Err("FFI f64: Expected 1 or 2 arguments".to_string()),
            }
        }
    }
    
    /// Call a void function (no return)
    pub fn call_void(&self, lib_name: &str, func_name: &str, args: &[i64]) -> Result<(), String> {
        let lib = self.libraries.get(lib_name)
            .ok_or_else(|| format!("Library not loaded: {}", lib_name))?;
        
        unsafe {
            match args.len() {
                0 => {
                    let func: Symbol<extern "C" fn()> = lib.get(func_name.as_bytes())
                        .map_err(|e| format!("Function not found: {} ({})", func_name, e))?;
                    func();
                    Ok(())
                }
                1 => {
                    let func: Symbol<extern "C" fn(i64)> = lib.get(func_name.as_bytes())
                        .map_err(|e| format!("Function not found: {} ({})", func_name, e))?;
                    func(args[0]);
                    Ok(())
                }
                _ => Err("FFI void: Too many arguments".to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ffi_manager_new() {
        let ffi = FfiManager::new();
        assert!(ffi.libraries.is_empty());
    }
}
