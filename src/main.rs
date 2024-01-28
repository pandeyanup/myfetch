use std::{thread, time};

use colored::*;
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("󰄛: {}", sysinfo::System::name().unwrap().cyan());
    println!(": {}", sysinfo::System::kernel_version().unwrap().cyan());
    println!(": {}", sysinfo::System::os_version().unwrap().cyan());
    println!(": {}", sysinfo::System::host_name().unwrap().cyan());

    loop {
        // Refresh CPU and memory information
        sys.refresh_all();

        // Move cursor to start of line
        print!("\r");

        // Print CPU usage for each core on the same line
        for (i, processor) in sys.cpus().iter().enumerate() {
            println!(
                "CPU {}: {}%   ",
                i,
                format!("{:.2}", processor.cpu_usage())
                    .to_string()
                    .bold()
                    .cyan()
            );
        }

        // Print RAM information on new line
        println!(
            "RAM: {}/{} GB",
            format!(
                "{:.2}",
                sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0
            )
            .bold()
            .cyan(),
            format!("{:.2}", sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0)
                .bold()
                .cyan()
        );

        // Move cursor up by the number of lines printed
        for _ in 0..(sys.cpus().len() + 1) {
            print!("\x1B[1A");
        }

        // Sleep for a while
        thread::sleep(time::Duration::from_millis(500));
    }
}
