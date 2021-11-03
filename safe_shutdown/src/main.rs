use std::{error::Error, thread, time::{Duration, Instant}};
use system_shutdown::{shutdown, reboot};
use crossbeam_channel::{Sender, Receiver, select, unbounded};
use rppal::gpio::{self, Gpio, InputPin, OutputPin, Trigger};

const GPIO_POWER: u8 = 3;
const GPIO_POWER_EN: u8 = 4;
const GPIO_RESET: u8 = 2;
const GPIO_LED: u8 = 14;

fn pressed(pin: &mut InputPin) -> Result<Receiver<()>, gpio::Error> {
    let (sender, receiver) = unbounded();
    pin.set_async_interrupt(Trigger::FallingEdge, move |_| { let _ = sender.send(()); })?;

    Ok(receiver)
}

fn startable_tick(duration: Duration, terminator: Receiver<()>) -> (Sender<bool>, Receiver<Instant>) {
    let (sender, receiver) = unbounded::<bool>();
    let (tick_sender, tick_receiver) = unbounded::<Instant>();

    thread::spawn(move || {
        let mut terminated = false;
        loop {
            select! {
                recv(receiver) -> msg => if msg.unwrap_or(false) { break },
                recv(terminator) -> _ => { terminated = true; break }
            }
        }

        let _ = tick_sender.send(Instant::now());
        if !terminated {
            loop {
                select! {
                    recv(terminator) -> _ => break, 
                    default => {
                        thread::sleep(duration);
                        let _ = tick_sender.send(Instant::now());
                    }
                }
            }
        }
    });

    (sender, tick_receiver)
}

fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;

    let mut led_pin = gpio.get(GPIO_LED)?.into_output();
    let mut power_pin = gpio.get(GPIO_POWER)?.into_input_pullup();
    let mut power_en_pin = gpio.get(GPIO_POWER_EN)?.into_output();
    let mut reset_pin = gpio.get(GPIO_RESET)?.into_input_pullup();

    led_pin.set_high();
    power_en_pin.set_high();

    let terminated = interruptor::interruption_or_termination();
    let power_off = pressed(&mut power_pin)?;
    let reset = pressed(&mut reset_pin)?;
    let (tick_enabler, tick) = 
        startable_tick(Duration::from_millis(500), terminated.clone());
    
    let mut shutting_down = false;
    let mut shutdown_now = |led_pin: &mut OutputPin, reset| -> Result<(), Box<dyn Error>> {
        if !shutting_down {
            shutting_down = true;
            led_pin.set_reset_on_drop(false);

            led_pin.set_low();
            thread::sleep(Duration::from_millis(300));
            led_pin.set_high();
        
            if reset {
                reboot()?;
            } else {
                shutdown()?;
            }

            tick_enabler.send(true)?;
        }

        Ok(())
    };
    loop {
        select! {
            recv(power_off) -> _ => shutdown_now(&mut led_pin, false)?,
            recv(reset) -> _ => shutdown_now(&mut led_pin, true)?,
            recv(tick) -> _ => led_pin.toggle(),
            recv(terminated) -> _ => break,
        }
    };

    Ok(())
}
