use std::fs;
use std::process::Command;
use std::time::{Duration, Instant};
use chrono::Local;

// Define colors and reset constants
const RESET: &str = "^d^";

#[derive(Debug)]
struct BarSegment {
    bg_color: &'static str,
    fg_color: &'static str,
}

impl BarSegment {
    fn format(&self, content: &str) -> String {
        format!("{}{} {} {}", self.bg_color, self.fg_color, content, RESET)
    }
}

struct StatusBar {
    cpu: BarSegment,
    // battery: BarSegment,
    wifi: BarSegment,
    nowifi: BarSegment,
    clock: BarSegment,
    // network: BarSegment,
    volume: BarSegment,
}

impl StatusBar {
    fn new() -> Self {
        StatusBar {
            cpu: BarSegment { bg_color: "^b#b383f3^", fg_color: "^c#2E3440^" },  // Nord14 and Nord0
            // battery: BarSegment { bg_color: "^b#b383f3^", fg_color: "^c#2E3440^" },  // Nord11 and Nord6
            wifi: BarSegment { bg_color: "^b#3fc1b0^", fg_color: "^c#2E3440^" },  // Nord8 and Nord0
            nowifi: BarSegment { bg_color: "^b#d73f3f^", fg_color: "^c#2E3440^" },  // Nord8 and Nord0
            clock: BarSegment { bg_color: "^b#599dcf^", fg_color: "^c#3E3440^" },  // Nord10 and Nord6
            // network: BarSegment { bg_color: "^b#359ad4^", fg_color: "^c#2E3440^" },  // Nord7 and Nord0
            volume: BarSegment { bg_color: "^b#d95574^", fg_color: "^c#2E3440^" },  // Nord12 and Nord6
        }
    }

    fn loadavg(&self) -> String {
        let contents = fs::read_to_string("/proc/loadavg").unwrap_or_default();
        let cpu_val = contents.split_whitespace().next().unwrap_or("N/A");
        self.cpu.format(&format!("CPU: {}", cpu_val))
    }

    // fn battery(&self) -> String {
    //     let capacity_str = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap_or("N/A".to_string());
    //     let capacity: u8 = capacity_str.trim().parse().unwrap_or(0);

    //     let icon = match capacity {
    //         0..=20 => "",
    //         21..=40 => "",
    //         41..=60 => "",
    //         61..=80 => "",
    //         81..=100 => "",
    //         _ => "",
    //     };

    //     self.battery.format(&format!("{}  {}%", icon, capacity))
    // }
fn battery(&self) -> String {
    let capacity_str = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap_or("N/A".to_string());
    let capacity: u8 = capacity_str.trim().parse().unwrap_or(0);

    // Define the battery icon and color based on the capacity range
    let (icon, bg_color, fg_color) = match capacity {
        0..=20 => ("", "^b#BF616A^", "^c#ECEFF4^"),  // Red background for low battery
        21..=40 => ("", "^b#D08770^", "^c#2E3440^"), // Orange background for 21-40% battery
        41..=60 => ("", "^b#EBCB8B^", "^c#2E3440^"), // Yellow background for 41-60% battery
        61..=80 => ("", "^b#A3BE8C^", "^c#2E3440^"), // Green background for 61-80% battery
        81..=100 => ("", "^b#88C0D0^", "^c#2E3440^"), // Blue background for 81-100% battery
        _ => ("", "^b#4C566A^", "^c#D8DEE9^"),       // Default color for unknown status
    };

    // Format the output with the appropriate color, icon, and capacity percentage
    format!("{}{} {}  {}% {}", bg_color, fg_color, icon, capacity, RESET)
}


    fn wlan(&self) -> String {
        let operstate = fs::read_to_string("/sys/class/net/wlp0s20f3/operstate").unwrap_or("down".to_string());
        match operstate.trim() {
            "up" => self.wifi.format("  Connected"),
            "down" => self.nowifi.format("Disconnected"),
            _ => self.wifi.format("Unknown")
        }
        
    }

    fn clock(&self) -> String {
        let now = Local::now();
        let time_str = now.format("%I:%M %p").to_string();  // Format the time as HH:MM AM/PM
        let date_str = now.format("%m/%d").to_string();  // Format the date as YYYY-MM-DD
        self.clock.format(&format!(" {}  {}", time_str, date_str))
    }

//     fn network_speed(&self) -> String {
//     let net_info = Command::new("ifstat")
//         .arg("-i")
//         .arg("wlp0s20f3")  // Replace with your network interface if different
//         .arg("1")
//         .arg("1")
//         .output()
//         .expect("Failed to fetch network speed");

//     let output = String::from_utf8_lossy(&net_info.stdout);
//     let lines: Vec<&str> = output.lines().collect();

//     if lines.len() >= 3 {
//         let speeds: Vec<&str> = lines[2].split_whitespace().collect();
//         if speeds.len() >= 2 {
//             let download_kb: f64 = speeds[0].parse().unwrap_or(0.0);
//             let upload_kb: f64 = speeds[1].parse().unwrap_or(0.0);

//             // Convert speeds greater than 10.00 KB/s to MB/s
//             let (download, download_unit) = if download_kb > 10.0 {
//                 (download_kb / 1024.0, "MB/s")
//             } else {
//                 (download_kb, "KB/s")
//             };

//             let (upload, upload_unit) = if upload_kb > 10.0 {
//                 (upload_kb / 1024.0, "MB/s")
//             } else {
//                 (upload_kb, "KB/s")
//             };

//             // Ensure the output is always in the format of 00.00
//             self.network.format(&format!(" {:05.2}{}  {:05.2}{} {}",
//                     download, download_unit, 
//                     upload, upload_unit, 
//                     RESET))
//         } else {
//             self.network.format("Network: N/A")
//         }
//     } else {
//         self.network.format("Network: N/A")
//     }
// }

    fn volume(&self) -> String {
        let vol_info = Command::new("amixer")
            .arg("get")
            .arg("Master")
            .output()
            .expect("Failed to fetch volume level");
        let output = String::from_utf8_lossy(&vol_info.stdout);
        let volume = output.lines().find(|line| line.contains('%')).unwrap_or("N/A").split_whitespace().nth(4).unwrap_or("N/A");
        self.volume.format(&format!(" {}", volume))
    }

    fn update_status(&self) {
        let status = format!("{}{}{}{}{}", 
                             self.loadavg(), 
                             self.battery(), 
                             self.clock(),
                             self.volume(),
                             self.wlan());
                             // self.network_speed()); 
        Command::new("xsetroot")
            .arg("-name")
            .arg(status)
            .output()
            .expect("Failed to execute xsetroot");
    }
}

fn main() {
    let status_bar = StatusBar::new();
    let interval = Duration::from_secs(1); // 1-second interval

    loop {
        let start = Instant::now();
        status_bar.update_status();
        let elapsed = start.elapsed();

        if elapsed < interval {
            std::thread::sleep(interval - elapsed);
        }
    }
}

