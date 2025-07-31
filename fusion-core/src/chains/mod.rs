pub mod ethereum;
pub mod near;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Chain {
    Ethereum,
    NEAR,
    Polygon,
    BaseSepolia,
    // Add more chains as needed
}

impl Chain {
    pub fn name(&self) -> &'static str {
        match self {
            Chain::Ethereum => "Ethereum",
            Chain::NEAR => "NEAR",
            Chain::Polygon => "Polygon",
            Chain::BaseSepolia => "Base Sepolia",
        }
    }
}
