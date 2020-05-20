#![allow(dead_code)]
use dbus;
use std::ops::Deref;

pub trait OrgGnomeSettingsDaemonPowerScreen {
    type Err;
    fn step_up(&self)                    -> Result<(i32, i32), Self::Err>;
    fn step_down(&self)                  -> Result<(i32, i32), Self::Err>;
    fn get_brightness(&self)             -> Result<i32, Self::Err>;
    fn set_brightness(&self, value: i32) -> Result<(), Self::Err>;
}

// impl<'a, C: Deref<Target = dbus::Connection>> OrgGnomeSettingsDaemonPowerScreen for dbus::ConnPath<'a, C> {
impl<'a, C> OrgGnomeSettingsDaemonPowerScreen for dbus::ConnPath<'a, C>
    where C: Deref<Target = dbus::Connection>
{
    type Err = dbus::Error;

    fn step_up(&self) -> Result<(i32, i32), Self::Err> {
        let mut m = self.method_call_with_args(
                &"org.gnome.SettingsDaemon.Power.Screen".into(), 
                &"StepUp".into(), 
                |_| {}
            )?;

        m.as_result()?;
        let mut i = m.iter_init();

        let new_percentage: i32 = i.read()?;
        let output_id: i32 = i.read()?;

        Ok((new_percentage, output_id))
    }

    fn step_down(&self) -> Result<(i32, i32), Self::Err> {
        let mut m = self.method_call_with_args(
                &"org.gnome.SettingsDaemon.Power.Screen".into(), 
                &"StepDown".into(), 
                |_| {}
            )?;
        m.as_result()?;

        let mut i = m.iter_init();
        let new_percentage: i32 = i.read()?;
        let output_id: i32 = i.read()?;

        Ok((new_percentage, output_id))
    }

    fn get_brightness(&self) -> Result<i32, Self::Err> {
        use dbus::stdintf::org_freedesktop_dbus::Properties;
        <Self as Properties>::get(&self, 
                                  "org.gnome.SettingsDaemon.Power.Screen", 
                                  "Brightness"
                                  )
    }

    fn set_brightness(&self, value: i32) -> Result<(), Self::Err> {
        use dbus::stdintf::org_freedesktop_dbus::Properties;
        <Self as Properties>::set(&self, 
                                  "org.gnome.SettingsDaemon.Power.Screen", 
                                  "Brightness", 
                                  value
                                  )
    }
}

