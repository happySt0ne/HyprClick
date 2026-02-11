use evdev::{AttributeSet, EventType, InputEvent, Key, uinput::VirtualDeviceBuilder};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::BTN_LEFT);

    let mut device = VirtualDeviceBuilder::new()?
        .name("hyper-fast-clicker")
        .with_keys(&keys)?
        .build()?;

    println!("super-fast clicker is running");

    let target_cps = 1000;
    let interval = Duration::from_nanos(1_000_000_000 / target_cps as u64);

    let press_event = InputEvent::new(EventType::KEY, Key::BTN_LEFT.0, 1);
    let release_event = InputEvent::new(EventType::KEY, Key::BTN_LEFT.0, 0);

    loop {
        let start = Instant::now();

        device.emit(&[press_event])?;
        device.emit(&[release_event])?;

        while start.elapsed() < interval {}
    }
}
