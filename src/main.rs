use fuck_you_john::{wlan::Wlan, AnyResult};

fn main() -> AnyResult<()> {
    let wlan = Wlan::new()?;
    for interface in wlan.get_interfaces()? {
        for profile in wlan.get_profiles(interface)? {
            println!(
                "{:?}: {:?}",
                profile.to_ascii_uppercase(),
                wlan.get_authentication(&interface, &profile)
            );
        }
    }
    Ok(())
}
