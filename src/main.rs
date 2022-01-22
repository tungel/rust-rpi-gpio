use std::error::Error;
use std::process::Command;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 14 is tied to physical pin 8.
const GPIO_FAN: u8 = 14;

const ON_THRESHOLD: f32 = 52.0; // (degrees Celsius) Fan turns on at this temperature.
const OFF_THRESHOLD: f32 = 48.0; // (degress Celsius) Fan turns off at this temperature.
const SLEEP_INTERVAL: u64 = 30; // (seconds) How often we check the core temperature.

fn get_temp() -> String {
    let output = Command::new("sh")
        .args(["-c", "vcgencmd measure_temp"])
        .output()
        .expect("failed to execute process");
    String::from_utf8(output.stdout).expect("Failed to convert output to String")
}

fn write_log(temp_float: f32) {
    let msg = format!("{}{}", "Current temp is: ", temp_float);
    println!("{}", msg);
    Command::new("logger")
        .args(["-p", "notice", "-t", "tungle", &msg])
        .output();
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_FAN)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    // pin.set_low();
    // thread::sleep(Duration::from_millis(3000));
    // pin.set_high();

    loop {
        let temp_string = get_temp(); // the result is something like this: temp=37.9'C

        // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
        // This gives an iterator, which you can loop over, or collect() into a vector.
        let mut split = temp_string.split("=");
        let mut vec = split.collect::<Vec<&str>>();
        split = vec[1].split("'");
        vec = split.collect::<Vec<&str>>();

        let temp_float: f32 = vec[0].parse().unwrap();
        write_log(temp_float);

        if temp_float > ON_THRESHOLD {
            pin.set_high();
        } else if temp_float <= OFF_THRESHOLD {
            pin.set_low();
        }

        thread::sleep(Duration::from_millis(SLEEP_INTERVAL * 1000));
    }

    Ok(())
}
