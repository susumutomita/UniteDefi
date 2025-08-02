use fusion_core::chains::ethereum::order_extractor::OrderExtractor;

#[tokio::test]
async fn should_extract_order_from_limit_order_protocol() {
    // Given: A valid order hash and EVM connection
    let order_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";
    let rpc_url = "https://base-sepolia.infura.io/v3/test-key";
    let limit_order_address = "0x171C87724E720F2806fc29a010a62897B30fdb62"; // Base Sepolia deployment

    let extractor =
        OrderExtractor::new(rpc_url, limit_order_address).expect("Should create extractor");

    // When: We extract the order
    let result = extractor.extract_order_by_hash(order_hash).await;

    // Then: We should get a valid order
    assert!(result.is_ok());
    let order = result.unwrap();
    assert!(!order.maker().is_empty());
    assert!(!order.maker_asset().is_empty());
    assert!(order.making_amount() > 0);
}

#[tokio::test]
async fn should_handle_invalid_order_hash() {
    // Given: An invalid order hash
    let order_hash = "invalid_hash";
    let rpc_url = "https://base-sepolia.infura.io/v3/test-key";
    let limit_order_address = "0x171C87724E720F2806fc29a010a62897B30fdb62";

    let extractor =
        OrderExtractor::new(rpc_url, limit_order_address).expect("Should create extractor");

    // When: We try to extract the order
    let result = extractor.extract_order_by_hash(order_hash).await;

    // Then: We should get an error
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid order hash"));
}

#[tokio::test]
async fn should_parse_order_struct_from_contract() {
    // Given: A mock order structure from the contract
    let order_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";
    let rpc_url = "https://base-sepolia.infura.io/v3/test-key";
    let limit_order_address = "0x171C87724E720F2806fc29a010a62897B30fdb62";

    let extractor =
        OrderExtractor::new(rpc_url, limit_order_address).expect("Should create extractor");

    // When: We extract order details
    let result = extractor.get_order_details(order_hash).await;

    // Then: We should get properly parsed order details
    assert!(result.is_ok());
    let details = result.unwrap();
    assert_eq!(details.status, "active");
    assert!(details.remaining_amount > 0);
}
