use assert_cmd::prelude::*; // Add methods on commands
use std::process::{Command, Stdio};  // Run programs
use std::thread::sleep;
use std::time::Duration;
use reqwest::blocking::get;
use std::env;
use std::path::Path;
use std::io::{self, BufRead, Write};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::processthreadsapi::TerminateProcess;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{HANDLE, PROCESS_TERMINATE};

fn wait_for_server(url: &str, timeout: Duration) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if get(url).is_ok() {
            return true;
        }
        sleep(Duration::from_secs(1));
    }
    false
}

#[test]
fn test_tauri_app_launch() {
    // Set the working directory to the location of your Trunk project
    let trunk_project_dir = env::current_dir().unwrap().parent().unwrap().join("dist");

    // Start the Trunk server on port 1420
    let mut trunk_cmd = Command::new("trunk")
        .arg("serve")
        .arg("--port")
        .arg("1420")
        .current_dir(&trunk_project_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start Trunk server");

    // Capture and print the server logs
    let trunk_stdout = trunk_cmd.stdout.take().unwrap();
    let trunk_stderr = trunk_cmd.stderr.take().unwrap();
    std::thread::spawn(move || {
        let reader = std::io::BufReader::new(trunk_stdout);
        for line in reader.lines() {
            println!("Trunk stdout: {}", line.unwrap());
        }
    });
    std::thread::spawn(move || {
        let reader = std::io::BufReader::new(trunk_stderr);
        for line in reader.lines() {
            eprintln!("Trunk stderr: {}", line.unwrap());
        }
    });

    // Wait for the server to be ready
    assert!(wait_for_server("http://localhost:1420", Duration::from_secs(30)), "Server did not start in time");

    // Path to the Tauri application executable
    let mut cmd = Command::cargo_bin("spotify-bb").expect("Failed to find the binary");

    // Capture the output of the Tauri application
    let mut tauri_process = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("Failed to start Tauri application");

    // Sleep for 5 seconds to allow the Tauri application to initialize
    sleep(Duration::from_secs(5));

    // Verify the server response
    let response = get("http://localhost:1420").expect("Failed to get response from server");
    let body = response.text().expect("Failed to read response body");
    println!("Server response: {}", body);

    // Attempt to gracefully terminate the Tauri process
    match tauri_process.try_wait() {
        Ok(Some(status)) => {
            println!("Tauri process exited with status: {}", status);
        }
        Ok(None) => {
            println!("Tauri process is still running, attempting to terminate gracefully...");
            unsafe {
                let process_id = tauri_process.id();
                let handle: HANDLE = OpenProcess(PROCESS_TERMINATE, 0, process_id);
                if !handle.is_null() {
                    TerminateProcess(handle, 0);
                    CloseHandle(handle);
                } else {
                    eprintln!("Failed to open process handle");
                }
            }
            sleep(Duration::from_secs(5)); // Give it some time to exit gracefully
            match tauri_process.try_wait() {
                Ok(Some(status)) => {
                    println!("Tauri process exited with status: {}", status);
                }
                Ok(None) => {
                    println!("Tauri process did not exit, attempting to kill...");
                    tauri_process.kill().expect("Failed to kill Tauri process");
                }
                Err(e) => {
                    eprintln!("Error attempting to wait on Tauri process: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error attempting to wait on Tauri process: {}", e);
        }
    }

    // Wait for the Tauri process to exit and capture its output
    let output = tauri_process.wait_with_output().expect("Failed to wait on Tauri process");

    // Print the output of the Tauri application
    println!("Tauri stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("Tauri stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Check if the Tauri application exited successfully
    assert!(output.status.success(), "Tauri application did not exit successfully");

    // Kill the Trunk server after the test
    trunk_cmd.kill().expect("Failed to kill Trunk server");
}