use auto_flask::config::FlasksConfig;
use auto_flask::Flasks;
use auto_flask::key::{KeySequence, KeyResponder};
use rdev::{listen, EventType};
use std::sync::mpsc;
use std::thread;

static mut KEY: EventType = EventType::MouseMove { x: 0.0, y: 0.0 };
static mut TX: Option<mpsc::Sender<()>> = None;

fn main() {
    let config = FlasksConfig::from_yaml_file("config.yml");
    let (mut flasks, key): (Flasks<KeySequence>, KeySequence) = config.into_flasks_and_key();
    let key = KeyResponder::from(key);
    let (tx, rx) = mpsc::channel();
    unsafe {
        KEY = *key;
        TX = Some(tx);
    };
    thread::spawn(move || {
        let _ = listen(|event| {
            if &event.event_type == unsafe { &KEY } {
                println!("{:?}", &event);
                let _ = unsafe { TX.as_ref().unwrap().send(()) };
            }
        });
    });
    loop {
        let _ = rx.recv();
        flasks.send();
    }
}