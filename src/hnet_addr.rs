#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HNetAddr {
    pub continent: String,
    pub country: String,
    pub provider: String,
    pub subnet: String,
    pub host: String,
}

impl HNetAddr {
    pub fn parse(addr: &str) -> Option<Self> {
        let parts: Vec<&str> = addr.split(':').collect();
        if parts.len() != 5 {
            return None;
        }
        Some(HNetAddr {
            continent: parts[0].to_string(),
            country: parts[1].to_string(),
            provider: parts[2].to_string(),
            subnet: parts[3].to_string(),
            host: parts[4].to_string(),
        })
    }

    pub fn generate_prefixes(&self) -> Vec<String> {
        vec![
            format!(
                "{}:{}:{}:{}:{}",
                self.continent, self.country, self.provider, self.subnet, self.host
            ),
            format!(
                "{}:{}:{}:{}:*",
                self.continent, self.country, self.provider, self.subnet
            ),
            format!("{}:{}:{}:*:*", self.continent, self.country, self.provider),
            format!("{}:{}:*:*:*", self.continent, self.country),
            format!("{}:*:*:*:*", self.continent),
        ]
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.continent, self.country, self.provider, self.subnet, self.host
        )
    }
}
