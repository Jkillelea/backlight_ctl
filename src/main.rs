extern crate dbus;

use std::env;
use std::process;
use dbus::{Connection, BusType, Message};

mod powerdaemon_sys; 

fn main() {
    let conn = Connection::get_private(BusType::Session).unwrap();

    let mut args = env::args();
    args.next(); // skip program name
    while let Some(arg) = args.next() { // arg parsing
        match arg.as_str() {
            "set"  => if let Some(val) = args.next() {
                            if let Ok(percent) = val.parse() {
                                set_percentage(&conn, percent)
                            } else { print_usage() }
                      } else { print_usage() },
            "up"   => step_up(&conn),
            "down" => step_down(&conn),
            "get"  => println!("{}", get_percentage(&conn)),
            _      => print_usage(),
        }
    }
}

fn print_usage() {
    println!("Usage: backlight_ctl {{up, down, get, set <val>}}");
    process::exit(1);
}

fn get_percentage(c: &Connection) -> u32 {
    let timeout = 200; // milliseconds?
    let msg = Message::new_method_call("org.gnome.SettingsDaemon.Power",        // destination
                                       "/org/gnome/SettingsDaemon/Power",       // path
                                       "org.gnome.SettingsDaemon.Power.Screen", // interface
                                       "GetPercentage").unwrap();               // method
    let result = c.send_with_reply_and_block(msg, timeout);
    if result.is_ok() {
        result.unwrap().read1().unwrap()
    } else {
        use powerdaemon_sys::*;

        let connpath = dbus::ConnPath {
            conn: c,
            dest: dbus::BusName::new("org.gnome.SettingsDaemon.Power").unwrap(),
            path: dbus::Path::new("/org/gnome/SettingsDaemon/Power").unwrap(),
            timeout: 1000i32 // (ms)
        };
        connpath.get_brightness().unwrap() as u32
    }
}

fn set_percentage(c: &Connection, percent: u32) {
    use powerdaemon_sys::*;

    let connpath = dbus::ConnPath {
        conn: c,
        dest: dbus::BusName::new("org.gnome.SettingsDaemon.Power").unwrap(),
        path: dbus::Path::new("/org/gnome/SettingsDaemon/Power").unwrap(),
        timeout: 1000i32 // (ms)
    };
    connpath.set_brightness(percent as i32).unwrap()
}

fn step_down(c: &Connection) {
    let msg = Message::new_method_call("org.gnome.SettingsDaemon.Power",
                                       "/org/gnome/SettingsDaemon/Power",
                                       "org.gnome.SettingsDaemon.Power.Screen",
                                       "StepDown").unwrap();
    c.send(msg).unwrap();
}

fn step_up(c: &Connection) {
    let msg = Message::new_method_call("org.gnome.SettingsDaemon.Power",
                                       "/org/gnome/SettingsDaemon/Power",
                                       "org.gnome.SettingsDaemon.Power.Screen",
                                       "StepUp").unwrap();
    c.send(msg).unwrap();
}
