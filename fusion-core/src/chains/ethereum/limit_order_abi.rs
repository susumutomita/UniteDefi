use ethers::prelude::*;

// Order struct following 1inch Limit Order Protocol V4
#[derive(Debug, Clone, Default, EthAbiType, EthAbiCodec)]
pub struct Order {
    pub salt: U256,
    pub maker: Address,
    pub receiver: Address,
    pub maker_asset: Address,
    pub taker_asset: Address,
    pub making_amount: U256,
    pub taking_amount: U256,
    pub maker_traits: U256,
}

// Generate bindings for the Limit Order Protocol
abigen!(
    LimitOrderProtocol,
    r#"[
        function remainingInvalidatorForOrder(address maker, bytes32 orderHash) external view returns (uint256)
        function hashOrder((uint256,address,address,address,address,uint256,uint256,uint256) order) external view returns (bytes32)
        function fillOrder((uint256,address,address,address,address,uint256,uint256,uint256) order, bytes signature, bytes interaction, uint256 makingAmount, uint256 takingAmount, uint256 skipPermitAndThresholdAmount) external payable returns (uint256 actualMakingAmount, uint256 actualTakingAmount, bytes32 orderHash)
        function cancelOrder(uint256 makerTraits, bytes32 orderHash) external
        function checkPredicate((uint256,address,address,address,address,uint256,uint256,uint256) order) external view returns (bool)
        
        event OrderFilled(
            bytes32 indexed orderHash,
            uint256 remainingAmount
        )
        
        event OrderCancelled(
            bytes32 indexed orderHash
        )
    ]"#
);

impl Order {
    pub fn hash(&self) -> H256 {
        // Using the same type hash as 1inch Limit Order Protocol V4
        let type_hash = keccak256(
            "Order(uint256 salt,address maker,address receiver,address makerAsset,address takerAsset,uint256 makingAmount,uint256 takingAmount,uint256 makerTraits)"
        );
        
        let encoded = ethers::abi::encode(&[
            Token::FixedBytes(type_hash.to_vec()),
            Token::Uint(self.salt),
            Token::Address(self.maker),
            Token::Address(self.receiver),
            Token::Address(self.maker_asset),
            Token::Address(self.taker_asset),
            Token::Uint(self.making_amount),
            Token::Uint(self.taking_amount),
            Token::Uint(self.maker_traits),
        ]);
        
        keccak256(&encoded).into()
    }
}