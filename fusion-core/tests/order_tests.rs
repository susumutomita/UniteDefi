#[cfg(test)]
mod order_tests {
    use fusion_core::order::OrderBuilder;

    #[test]
    fn test_create_order() {
        let order = OrderBuilder::new()
            .maker_asset("0x4200000000000000000000000000000000000006")
            .taker_asset("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")
            .maker("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950")
            .making_amount(1000000000000000000u128)
            .taking_amount(3000000000u128)
            .build()
            .expect("Failed to build order");

        assert_eq!(
            order.maker_asset(),
            "0x4200000000000000000000000000000000000006"
        );
        assert_eq!(
            order.taker_asset(),
            "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"
        );
        assert_eq!(order.maker(), "0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950");
        assert_eq!(order.making_amount(), 1000000000000000000u128);
        assert_eq!(order.taking_amount(), 3000000000u128);
    }
}
