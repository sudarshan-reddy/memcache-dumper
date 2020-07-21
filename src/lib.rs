use memcache::Client;
use std::io::Write;
pub struct Dumper {
    client: Client,
}

impl Dumper {
    pub fn new(addr: &str) -> Result<Self, Error> {
        let cl = memcache::Client::connect(addr)?;
        Ok(Dumper { client: cl })
    }

    pub fn dump_key_to_file<W: Write>(&mut self, key: &str, mut writer: W) -> Result<(), Error> {
        let value: Option<Vec<u8>> = self.client.get(key)?;
        value.and_then(|v| Some(writer.write(&v)));
        Ok(())
    }
}

type Error = Box<dyn std::error::Error + Send + Sync>;
