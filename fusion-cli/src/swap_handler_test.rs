#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_decimals() {
        assert_eq!(get_token_decimals("NEAR"), 24);
        assert_eq!(get_token_decimals("ETH"), 18);
        assert_eq!(get_token_decimals("WETH"), 18);
        assert_eq!(get_token_decimals("USDC"), 6);
        assert_eq!(get_token_decimals("USDT"), 6);
        assert_eq!(get_token_decimals("DAI"), 18);
        assert_eq!(get_token_decimals("0x1234..."), 18); // Unknown token
    }

    #[test]
    fn test_amount_conversion() {
        // Test ETH conversion (18 decimals)
        assert_eq!(convert_amount_to_wei(1.0, "ETH"), 1_000_000_000_000_000_000);
        assert_eq!(convert_amount_to_wei(0.001, "ETH"), 1_000_000_000_000_000);
        
        // Test NEAR conversion (24 decimals)
        assert_eq!(convert_amount_to_wei(1.0, "NEAR"), 1_000_000_000_000_000_000_000_000);
        assert_eq!(convert_amount_to_wei(0.001, "NEAR"), 1_000_000_000_000_000_000_000);
        
        // Test USDC conversion (6 decimals)
        assert_eq!(convert_amount_to_wei(1.0, "USDC"), 1_000_000);
        assert_eq!(convert_amount_to_wei(0.001, "USDC"), 1_000);
        assert_eq!(convert_amount_to_wei(1000.0, "USDC"), 1_000_000_000);
    }

    #[test]
    fn test_wei_to_amount_conversion() {
        // Test ETH conversion
        assert_eq!(convert_wei_to_amount(1_000_000_000_000_000_000, "ETH"), 1.0);
        assert_eq!(convert_wei_to_amount(1_000_000_000_000_000, "ETH"), 0.001);
        
        // Test NEAR conversion
        assert_eq!(convert_wei_to_amount(1_000_000_000_000_000_000_000_000, "NEAR"), 1.0);
        
        // Test USDC conversion
        assert_eq!(convert_wei_to_amount(1_000_000, "USDC"), 1.0);
        assert_eq!(convert_wei_to_amount(1_000, "USDC"), 0.001);
    }

    #[test]
    fn test_precision_handling() {
        // Test that we handle floating point precision correctly
        let amount = 0.123456789;
        let wei = convert_amount_to_wei(amount, "ETH");
        let back = convert_wei_to_amount(wei, "ETH");
        
        // Should be close within floating point precision
        assert!((amount - back).abs() < 0.000000001);
        
        // Test with USDC (fewer decimals)
        let usdc_amount = 1234.56;
        let usdc_wei = convert_amount_to_wei(usdc_amount, "USDC");
        assert_eq!(usdc_wei, 1_234_560_000); // 1234.56 * 10^6
        
        let usdc_back = convert_wei_to_amount(usdc_wei, "USDC");
        assert!((usdc_amount - usdc_back).abs() < 0.01);
    }
}