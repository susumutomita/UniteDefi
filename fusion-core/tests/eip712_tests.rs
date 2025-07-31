#[cfg(test)]
mod eip712_tests {
    use fusion_core::eip712::{EIP712Domain, OrderEIP712};
    use fusion_core::order::Order;

    #[test]
    fn test_eip712_domain_separator() {
        let domain = EIP712Domain {
            name: "1inch Limit Order Protocol".to_string(),
            version: "3".to_string(),
            chain_id: 84532,
            verifying_contract: "0x171C87724E720F2806fc29a010a62897B30fdb62".to_string(),
        };

        let separator = domain.separator();
        assert_eq!(separator.len(), 32);
    }

    #[test]
    fn test_order_eip712_hash() {
        let order = Order {
            salt: [1u8; 32],
            maker_asset: "0x4200000000000000000000000000000000000006".to_string(),
            taker_asset: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".to_string(),
            maker: "0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950".to_string(),
            receiver: "0x0000000000000000000000000000000000000000".to_string(),
            allowed_sender: "0x0000000000000000000000000000000000000000".to_string(),
            making_amount: 1000000000000000000u128,
            taking_amount: 3000000000u128,
            offsets: 0,
            interactions: "0x".to_string(),
        };

        let typed_data = order.to_eip712(84532, "0x171C87724E720F2806fc29a010a62897B30fdb62");
        let hash = typed_data.hash();

        assert_eq!(hash.len(), 32);
    }
}
