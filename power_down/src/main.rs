use std::{env, error::Error};
use rppal::gpio::Gpio;

const GPIO_POWER_EN: u8 = 4;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() >= 2 && args[1].eq_ignore_ascii_case("poweroff") {
        let gpio = Gpio::new()?;

        let mut power_en_pin = gpio.get(GPIO_POWER_EN)?.into_output();
        power_en_pin.set_reset_on_drop(false);
        power_en_pin.set_low();
    }

    Ok(())
}
