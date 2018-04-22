extern crate dbus;

use std::{env, rc::Rc, cell::Cell};
use dbus::{Connection, BusType, Message};

fn main() {
    let conn = Connection::get_private(BusType::Session).unwrap();

    let mut args = env::args();
    args.next(); // skip program name
    while let Some(arg) = args.next() {
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
}

// this method is gross but I'm not sure about a better way
fn get_percentage(c: &Connection) -> u32 {
    let msg = Message::new_method_call("org.gnome.SettingsDaemon.Power",
                                        "/org/gnome/SettingsDaemon/Power",
                                        "org.gnome.SettingsDaemon.Power.Screen",
                                        "GetPercentage").unwrap();

    // In order to talk back and forth across this closeure whose liftime we don't know
    // I'm using Rc (reference counted) Cells (which allow for mutable interiors)
    let data = Rc::new(Cell::new(0));     // hold data
    let done = Rc::new(Cell::new(false)); // hold done state
    let data_clone = data.clone();        // clones only clone the Rc pointer, which goes to same data
    let done_clone = done.clone();

    c.add_handler(c.send_with_reply(msg, move |reply| {
        let percentage: u32 = reply.unwrap().read1().unwrap();
        data_clone.set(percentage);
        done_clone.set(true);
    }).unwrap());

    // process incoming message and wait for closure to finish.
    while !done.get() {
        c.incoming(10).next();
    }
    data.get()
}

fn set_percentage(c: &Connection, percent: u32) {
    let msg = Message::new_method_call("org.gnome.SettingsDaemon.Power",
                                        "/org/gnome/SettingsDaemon/Power",
                                        "org.gnome.SettingsDaemon.Power.Screen",
                                        "SetPercentage").unwrap().append(percent);
    c.send(msg).unwrap();
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
