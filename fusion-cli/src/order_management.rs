use anyhow::{anyhow, Result};
use clap::Args;
use serde_json::json;
use crate::storage::{OrderStorage, OrderStatus, StoredOrder};
use once_cell::sync::Lazy;
use chrono::{DateTime, Utc};

pub static ORDER_STORAGE: Lazy<OrderStorage> = Lazy::new(OrderStorage::new);

#[derive(Args)]
pub struct StatusArgs {
    /// Order ID to check status
    #[arg(long)]
    pub order_id: String,
}

#[derive(Args)]
pub struct CancelArgs {
    /// Order ID to cancel
    #[arg(long)]
    pub order_id: String,
}

#[derive(Args)]
pub struct OrderbookArgs {
    /// Chain to get orderbook for (e.g., "ethereum", "near", "polygon")
    #[arg(long)]
    pub chain: String,
}

pub async fn handle_order_status(args: StatusArgs) -> Result<()> {
    // Get order from storage
    match ORDER_STORAGE.get(&args.order_id) {
        Ok(order) => {
            let created_at: DateTime<Utc> = order.created_at.into();
            let output = json!({
                "order_id": order.id,
                "status": format!("{:?}", order.status),
                "maker": order.maker,
                "maker_asset": order.maker_asset,
                "taker_asset": order.taker_asset,
                "making_amount": order.making_amount.to_string(),
                "taking_amount": order.taking_amount.to_string(),
                "chain": order.chain,
                "order_hash": order.order_hash,
                "created_at": created_at.to_rfc3339(),
            });

            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        Err(_) => {
            let output = json!({
                "error": "Order not found",
                "order_id": args.order_id
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}

pub async fn handle_order_cancel(args: CancelArgs) -> Result<()> {
    // Get order from storage
    let order = match ORDER_STORAGE.get(&args.order_id) {
        Ok(order) => order,
        Err(_) => {
            let output = json!({
                "error": "Order not found",
                "order_id": args.order_id
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            return Ok(());
        }
    };

    // Check if order can be cancelled
    match order.status {
        OrderStatus::Active => {
            // Update status to cancelled
            ORDER_STORAGE.update_status(&args.order_id, OrderStatus::Cancelled)?;
            
            let output = json!({
                "order_id": args.order_id,
                "status": "Cancelled",
                "message": "Order has been successfully cancelled",
                "cancelled_at": chrono::Utc::now().to_rfc3339()
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        OrderStatus::Filled => {
            let output = json!({
                "error": "Cannot cancel filled order",
                "order_id": args.order_id,
                "status": "Filled"
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        OrderStatus::Cancelled => {
            let output = json!({
                "error": "Order already cancelled",
                "order_id": args.order_id,
                "status": "Cancelled"
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        OrderStatus::Expired => {
            let output = json!({
                "error": "Cannot cancel expired order",
                "order_id": args.order_id,
                "status": "Expired"
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}

pub async fn handle_orderbook(args: OrderbookArgs) -> Result<()> {
    // Get all orders for the specified chain
    let orders = ORDER_STORAGE.get_orders_by_chain(&args.chain)?;
    
    if orders.is_empty() {
        let output = json!({
            "chain": args.chain,
            "orderbook": [],
            "message": format!("No orders found for chain: {}", args.chain)
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    // Convert orders to JSON format
    let orders_json: Vec<serde_json::Value> = orders
        .iter()
        .filter(|order| order.status == OrderStatus::Active) // Only show active orders
        .map(|order| {
            let created_at: DateTime<Utc> = order.created_at.into();
            json!({
                "order_id": order.id,
                "maker": order.maker,
                "maker_asset": order.maker_asset,
                "taker_asset": order.taker_asset,
                "making_amount": order.making_amount.to_string(),
                "taking_amount": order.taking_amount.to_string(),
                "price": calculate_price(order.making_amount, order.taking_amount),
                "order_hash": order.order_hash,
                "created_at": created_at.to_rfc3339(),
            })
        })
        .collect();

    let output = json!({
        "chain": args.chain,
        "orderbook": orders_json,
        "total_orders": orders_json.len(),
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn calculate_price(making_amount: u128, taking_amount: u128) -> String {
    if taking_amount == 0 {
        return "0".to_string();
    }
    
    // Simple price calculation - in production, we'd need to consider decimals
    let price = making_amount as f64 / taking_amount as f64;
    format!("{:.6}", price)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[tokio::test]
    async fn test_order_status_existing_order() {
        // Setup
        let order_id = "test_order_status_1";
        let order = StoredOrder {
            id: order_id.to_string(),
            maker: "0x1234567890123456789012345678901234567890".to_string(),
            maker_asset: "0xA000000000000000000000000000000000000001".to_string(),
            taker_asset: "0xB000000000000000000000000000000000000002".to_string(),
            making_amount: 1000000000000000000u128,
            taking_amount: 3000000000u128,
            status: OrderStatus::Active,
            created_at: SystemTime::now(),
            chain: "ethereum".to_string(),
            order_hash: "0xdeadbeef".to_string(),
        };
        ORDER_STORAGE.store(order_id.to_string(), order).unwrap();

        // Execute
        let args = StatusArgs {
            order_id: order_id.to_string(),
        };
        let result = handle_order_status(args).await;

        // Verify
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_order_cancel_active_order() {
        // Setup
        let order_id = "test_order_cancel_1";
        let order = StoredOrder {
            id: order_id.to_string(),
            maker: "0x1234567890123456789012345678901234567890".to_string(),
            maker_asset: "0xA000000000000000000000000000000000000001".to_string(),
            taker_asset: "0xB000000000000000000000000000000000000002".to_string(),
            making_amount: 1000000000000000000u128,
            taking_amount: 3000000000u128,
            status: OrderStatus::Active,
            created_at: SystemTime::now(),
            chain: "ethereum".to_string(),
            order_hash: "0xdeadbeef".to_string(),
        };
        ORDER_STORAGE.store(order_id.to_string(), order).unwrap();

        // Execute
        let args = CancelArgs {
            order_id: order_id.to_string(),
        };
        let result = handle_order_cancel(args).await;

        // Verify
        assert!(result.is_ok());
        let updated_order = ORDER_STORAGE.get(order_id).unwrap();
        assert_eq!(updated_order.status, OrderStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_orderbook_by_chain() {
        // Setup - Add some ethereum orders
        for i in 0..3 {
            let order = StoredOrder {
                id: format!("test_orderbook_eth_{}", i),
                maker: "0x1234567890123456789012345678901234567890".to_string(),
                maker_asset: "0xA000000000000000000000000000000000000001".to_string(),
                taker_asset: "0xB000000000000000000000000000000000000002".to_string(),
                making_amount: 1000000000000000000u128,
                taking_amount: 3000000000u128,
                status: OrderStatus::Active,
                created_at: SystemTime::now(),
                chain: "ethereum".to_string(),
                order_hash: format!("0xdeadbeef{}", i),
            };
            ORDER_STORAGE.store(order.id.clone(), order).unwrap();
        }

        // Execute
        let args = OrderbookArgs {
            chain: "ethereum".to_string(),
        };
        let result = handle_orderbook(args).await;

        // Verify
        assert!(result.is_ok());
    }

    #[test]
    fn test_calculate_price() {
        assert_eq!(calculate_price(1000000000000000000, 3000000000), "333333.333333");
        assert_eq!(calculate_price(0, 1000), "0.000000");
        assert_eq!(calculate_price(1000, 0), "0");
    }
}