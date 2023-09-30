

pub struct Pa {
    port : String,
    address : String,
}

impl Pa {

    pub fn new(address: String, port: String) -> Self {
        Pa {
            address,
            port
        }
    }

    pub fn get_string(&self) -> String {
        let mut out = String::new().to_owned();
        out.push_str(self.address.as_str());
        out.push_str(":");
        out.push_str(self.port.as_str());
        return out;
    }
}

