use xsd::bus::XsdBusError;
use xsd::client::XsdClient;

fn list_recursive(client: &mut XsdClient, level: usize, path: &str) -> Result<(), XsdBusError> {
    let children = match client.list(path) {
        Ok(children) => children,
        Err(error) => {
            return if error.to_string() == "EINVAL" {
                Ok(())
            } else {
                Err(error)
            }
        }
    };

    for child in children {
        let full = format!("{}/{}", if path == "/" { "" } else { path }, child);
        let value = client.read(full.as_str())?;
        println!(
            "{}{} = {:?}",
            " ".repeat(level),
            child,
            String::from_utf8(value)?
        );
        list_recursive(client, level + 1, full.as_str())?;
    }
    Ok(())
}

fn main() -> Result<(), XsdBusError> {
    let mut client = XsdClient::new()?;
    list_recursive(&mut client, 0, "/")?;
    Ok(())
}
