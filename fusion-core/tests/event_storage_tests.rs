#[cfg(test)]
mod tests {
    use fusion_core::chains::ethereum::event_storage::{
        EventData, EventStorage, InMemoryEventStorage, StoredEvent,
    };

    #[tokio::test]
    async fn test_in_memory_storage_save_and_retrieve() {
        // Given
        let storage = InMemoryEventStorage::new();
        let event = StoredEvent {
            event_type: "OrderFilled".to_string(),
            order_hash: "0x1234567890abcdef".to_string(),
            block_number: 100,
            timestamp: 1234567890,
            data: EventData::OrderFilled {
                remaining_amount: "1000000".to_string(),
            },
        };

        // When
        storage.save_event(event.clone()).await.unwrap();

        // Then
        let retrieved = storage
            .get_events_by_order_hash(&event.order_hash)
            .await
            .unwrap();
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].order_hash, event.order_hash);
        assert_eq!(retrieved[0].event_type, event.event_type);
    }

    #[tokio::test]
    async fn test_in_memory_storage_multiple_events_same_order() {
        // Given
        let storage = InMemoryEventStorage::new();
        let order_hash = "0xabcdef1234567890".to_string();

        let event1 = StoredEvent {
            event_type: "OrderFilled".to_string(),
            order_hash: order_hash.clone(),
            block_number: 100,
            timestamp: 1234567890,
            data: EventData::OrderFilled {
                remaining_amount: "1000000".to_string(),
            },
        };

        let event2 = StoredEvent {
            event_type: "OrderCancelled".to_string(),
            order_hash: order_hash.clone(),
            block_number: 101,
            timestamp: 1234567891,
            data: EventData::OrderCancelled,
        };

        // When
        storage.save_event(event1).await.unwrap();
        storage.save_event(event2).await.unwrap();

        // Then
        let retrieved = storage.get_events_by_order_hash(&order_hash).await.unwrap();
        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved[0].event_type, "OrderFilled");
        assert_eq!(retrieved[1].event_type, "OrderCancelled");
    }

    #[tokio::test]
    async fn test_in_memory_storage_get_all_events() {
        // Given
        let storage = InMemoryEventStorage::new();

        let event1 = StoredEvent {
            event_type: "OrderFilled".to_string(),
            order_hash: "0x1111111111111111".to_string(),
            block_number: 100,
            timestamp: 1234567890,
            data: EventData::OrderFilled {
                remaining_amount: "1000000".to_string(),
            },
        };

        let event2 = StoredEvent {
            event_type: "OrderCancelled".to_string(),
            order_hash: "0x2222222222222222".to_string(),
            block_number: 101,
            timestamp: 1234567891,
            data: EventData::OrderCancelled,
        };

        // When
        storage.save_event(event1).await.unwrap();
        storage.save_event(event2).await.unwrap();

        // Then
        let all_events = storage.get_all_events().await.unwrap();
        assert_eq!(all_events.len(), 2);
        assert_eq!(all_events[0].order_hash, "0x1111111111111111");
        assert_eq!(all_events[1].order_hash, "0x2222222222222222");
    }

    #[tokio::test]
    async fn test_in_memory_storage_empty_order_hash() {
        // Given
        let storage = InMemoryEventStorage::new();

        // When
        let retrieved = storage
            .get_events_by_order_hash("0xnonexistent")
            .await
            .unwrap();

        // Then
        assert_eq!(retrieved.len(), 0);
    }
}
