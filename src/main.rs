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

struct CachedData<T> {
    value: T,
    last_updated: Instant,
}

impl<T> CachedData<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            last_updated: Instant::now(),
        }
    }

    fn is_expired(&self, duration: Duration) -> bool {
        self.last_updated.elapsed() >= duration
    }
}

struct StatusBar {
    cpu: BarSegment,
    wifi: BarSegment,
    nowifi: BarSegment,
    clock: BarSegment,
    volume: BarSegment,
    battery_cache: CachedData<String>,
    clock_cache: CachedData<String>,
    previous_status: String,
}

impl StatusBar {
    fn new() -> Self {
        StatusBar {
            cpu: BarSegment { bg_color: "^b#b383f3^", fg_color: "^c#2E3440^" },  // Nord14 and Nord0
            wifi: BarSegment { bg_color: "^b#3fc1b0^", fg_color: "^c#2E3440^" },  // Nord8 and Nord0
            nowifi: BarSegment { bg_color: "^b#d73f3f^", fg_color: "^c#2E3440^" },  // Nord8 and Nord0
            clock: BarSegment { bg_color: "^b#599dcf^", fg_color: "^c#3E3440^" },  // Nord10 and Nord6
            volume: BarSegment { bg_color: "^b#818991^", fg_color: "^c#2E3440^" },  // Nord12 and Nord6
            battery_cache: CachedData::new(String::new()),  // Initialize with empty string
            clock_cache: CachedData::new(String::new()),    // Initialize with empty string
            previous_status: String::new(),
        }
    }

    fn loadavg(&self) -> String {
        let contents = fs::read_to_string("/proc/loadavg").unwrap_or_default();
        let cpu_val = contents.split_whitespace().next().unwrap_or("N/A");
        self.cpu.format(&format!("CPU: {}", cpu_val))
    }

    // fn battery(&mut self) -> String {
    //     let cache_duration = Duration::from_secs(30);  // Cache battery status for 30 seconds
    //     if self.battery_cache.is_expired(cache_duration) {
    //         let capacity_str = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap_or("N/A".to_string());
    //         let capacity: u8 = capacity_str.trim().parse().unwrap_or(0);
    //         let (icon, bg_color, fg_color) = match capacity {
    //             0..=20 => ("", "^b#BF616A^", "^c#ECEFF4^"),  // Red background for low battery
    //             21..=40 => ("", "^b#D08770^", "^c#2E3440^"), // Orange background for 21-40% battery
    //             41..=60 => ("", "^b#EBCB8B^", "^c#2E3440^"), // Yellow background for 41-60% battery
    //             61..=80 => ("", "^b#A3BE8C^", "^c#2E3440^"), // Green background for 61-80% battery
    //             81..=100 => ("", "^b#88C0D0^", "^c#2E3440^"), // Blue background for full battery
    //             _ => ("", "^b#4C566A^", "^c#D8DEE9^"),       // Default color for unknown status
    //         };
    //         // Update the cache with the new value
    //         self.battery_cache = CachedData::new(format!("{}{} {}  {}% {}", bg_color, fg_color, icon, capacity, RESET));
    //     }
    //     self.battery_cache.value.clone()
    // }
        fn battery(&mut self) -> String {
        let cache_duration = Duration::from_secs(30); // Cache battery for 30 seconds

        // Read the current charging status
        let status = fs::read_to_string("/sys/class/power_supply/BAT0/status").unwrap_or("Unknown".to_string()).trim().to_string();

        // If the status changes or the cache is expired, update the cache
        if status != self.previous_status || self.battery_cache.is_expired(cache_duration) {
            // Update the previous status
            self.previous_status = status.clone();

            // Read the battery capacity
            let capacity_str = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap_or("N/A".to_string());
            let capacity: u8 = capacity_str.trim().parse().unwrap_or(0);

            // Determine the icon and color based on capacity and charging status
            let (icon, bg_color, fg_color) = match status.as_str() {
                "Charging" => ("", "^b#A3BE8C^", "^c#2E3440^"),  // Green background when charging
                _ => match capacity {
                    0..=20 => (" ", "^b#BF616A^", "^c#ECEFF4^"),  // Red background for low battery
                    21..=40 => (" ", "^b#D08770^", "^c#2E3440^"), // Orange background for 21-40% battery
                    41..=60 => (" ", "^b#EBCB8B^", "^c#2E3440^"), // Yellow background for 41-60% battery
                    61..=80 => (" ", "^b#A3BE8C^", "^c#2E3440^"), // Green background for 61-80% battery
                    81..=100 => (" ", "^b#88C0D0^", "^c#2E3440^"), // Blue background for full battery
                    _ => ("", "^b#4C566A^", "^c#D8DEE9^"),       // Default color for unknown status
                },
            };

            // Update the cache with the formatted string
            self.battery_cache = CachedData::new(format!("{}{} {} {}% {}", bg_color, fg_color, icon, capacity, RESET));
        }

        // Return the cached value
        self.battery_cache.value.clone()
    }

    fn wlan(&self) -> String {
        let operstate = fs::read_to_string("/sys/class/net/wlp0s20f3/operstate").unwrap_or("down".to_string());
        match operstate.trim() {
            "up" => self.wifi.format("  Connected"),
            "down" => self.nowifi.format("Disconnected"),
            _ => self.wifi.format("Unknown")
        }
    }

    fn clock(&mut self) -> String {
        let cache_duration = Duration::from_secs(60);  // Cache clock for 60 seconds
        if self.clock_cache.is_expired(cache_duration) {
            let now = Local::now();
            let time_str = now.format("%I:%M %p").to_string();  // Format the time as HH:MM AM/PM
            let date_str = now.format("%m/%d").to_string();  // Format the date as MM/DD

            // Update the cache with the new value
            self.clock_cache = CachedData::new(self.clock.format(&format!(" {}  {}", time_str, date_str)));
        }

        self.clock_cache.value.clone()
    }

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

    fn update_status(&mut self) {
        let status = format!("{}{}{}{}{}", 
                             self.clock(),
                             self.loadavg(), 
                             self.battery(), 
                             self.volume(),
                             self.wlan());
        Command::new("xsetroot")
            .arg("-name")
            .arg(status)
            .output()
            .expect("Failed to execute xsetroot");
    }
}

fn main() {
    let mut status_bar = StatusBar::new();
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

