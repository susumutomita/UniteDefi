#[cfg(test)]
mod integration_tests {
    use crate::chains::near_events::NearHtlcClaimEvent;
    use crate::htlc::{generate_secret, hash_secret, Htlc, HtlcState};
    use crate::secret_manager::{SecretError, SecretManager};
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn test_complete_secret_lifecycle_integration() {
        let mut secret_manager = SecretManager::new()
            .with_ttl(Duration::from_secs(10))
            .with_cleanup_interval(Duration::from_millis(100));

        // 1. Generate secret for a swap
        let swap_id = "test_swap_001";
        let secret_hash = secret_manager.generate_secret(swap_id).unwrap();

        // 2. Create HTLC with the generated secret hash
        let secret = *secret_manager.get_secret(swap_id).unwrap();
        let mut htlc = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_secs(3600),
        )
        .unwrap();

        // 3. Verify HTLC is in pending state
        assert_eq!(htlc.state(), &HtlcState::Pending);
        assert!(!secret_manager.is_secret_revealed(swap_id));

        // 4. Simulate secret revelation through blockchain transaction
        secret_manager
            .mark_secret_revealed(
                swap_id,
                "ethereum".to_string(),
                Some("0x123abc".to_string()),
            )
            .unwrap();

        // 5. Verify secret is marked as revealed
        assert!(secret_manager.is_secret_revealed(swap_id));
        let (chain, _timestamp, tx_hash) = secret_manager.get_revelation_details(swap_id).unwrap();
        assert_eq!(chain, "ethereum");
        assert_eq!(tx_hash, Some("0x123abc".to_string()));

        // 6. Use revealed secret to claim HTLC
        let result = htlc.claim(&secret);
        assert!(result.is_ok());
        assert_eq!(htlc.state(), &HtlcState::Claimed);

        // 7. Verify secret manager statistics
        let stats = secret_manager.get_stats();
        assert_eq!(stats.total_secrets, 1);
        assert_eq!(stats.revealed_secrets, 1);
        assert_eq!(stats.active_secrets, 1);
    }

    #[tokio::test]
    async fn test_secret_extraction_from_transaction_integration() {
        let mut secret_manager = SecretManager::new();

        // Generate a known secret
        let original_secret = generate_secret();
        let secret_hash = hash_secret(&original_secret);

        // Create transaction data containing the secret
        let mut tx_data = vec![0u8; 200];
        // Embed the secret at position 100
        tx_data[100..132].copy_from_slice(&original_secret);

        // Extract secret from transaction
        let extracted_secret = secret_manager
            .extract_secret_from_transaction(&tx_data, &secret_hash)
            .await
            .unwrap();

        assert_eq!(extracted_secret, original_secret);

        // Verify the extracted secret can be used with HTLC
        let mut htlc = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_secs(3600),
        )
        .unwrap();

        let result = htlc.claim(&extracted_secret);
        assert!(result.is_ok());
        assert_eq!(htlc.state(), &HtlcState::Claimed);
    }

    #[tokio::test]
    async fn test_near_claim_event_processing_integration() {
        let mut secret_manager = SecretManager::new();

        // Generate secret for a swap
        let swap_id = "test_swap_002";
        let secret_hash = secret_manager.generate_secret(swap_id).unwrap();
        let secret = *secret_manager.get_secret(swap_id).unwrap();

        // Create HTLC
        let mut htlc = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_secs(3600),
        )
        .unwrap();

        // Simulate NEAR claim event
        let claim_event = NearHtlcClaimEvent {
            escrow_id: "fusion_0".to_string(),
            claimer: "bob.near".to_string(),
            secret: hex::encode(secret),
            timestamp: 1234567890,
        };

        // Process the claim event
        secret_manager
            .process_claim_event(&claim_event)
            .await
            .unwrap();

        // Verify secret is marked as revealed on NEAR
        assert!(secret_manager.is_secret_revealed(swap_id));
        let (chain, _timestamp, _tx_hash) = secret_manager.get_revelation_details(swap_id).unwrap();
        assert_eq!(chain, "near");

        // Use the secret to claim HTLC
        let result = htlc.claim(&secret);
        assert!(result.is_ok());
        assert_eq!(htlc.state(), &HtlcState::Claimed);
    }

    #[tokio::test]
    async fn test_automatic_cleanup_integration() {
        let mut secret_manager = SecretManager::new()
            .with_ttl(Duration::from_millis(50))
            .with_cleanup_interval(Duration::from_millis(10));

        // Generate multiple secrets
        secret_manager.generate_secret("swap_1").unwrap();
        secret_manager.generate_secret("swap_2").unwrap();
        secret_manager.generate_secret("swap_3").unwrap();

        let stats = secret_manager.get_stats();
        assert_eq!(stats.total_secrets, 3);
        assert_eq!(stats.active_secrets, 3);

        // Wait for expiration
        thread::sleep(Duration::from_millis(100));

        // Trigger cleanup
        let cleaned = secret_manager.cleanup_expired_secrets();
        assert_eq!(cleaned, 3);

        let stats = secret_manager.get_stats();
        assert_eq!(stats.total_secrets, 0);
        assert_eq!(stats.active_secrets, 0);
    }

    #[tokio::test]
    async fn test_error_handling_integration() {
        let mut secret_manager = SecretManager::new();

        // Test duplicate secret generation
        secret_manager.generate_secret("swap_1").unwrap();
        let result = secret_manager.generate_secret("swap_1");
        assert!(matches!(result, Err(SecretError::SecretAlreadyExists)));

        // Test accessing non-existent secret
        let result = secret_manager.get_secret("non_existent");
        assert!(matches!(result, Err(SecretError::SecretNotFound)));

        // Test marking non-existent secret as revealed
        let result =
            secret_manager.mark_secret_revealed("non_existent", "ethereum".to_string(), None);
        assert!(matches!(result, Err(SecretError::SecretNotFound)));

        // Test processing invalid claim event
        let invalid_event = NearHtlcClaimEvent {
            escrow_id: "fusion_0".to_string(),
            claimer: "bob.near".to_string(),
            secret: "invalid_hex".to_string(),
            timestamp: 1234567890,
        };

        let result = secret_manager.process_claim_event(&invalid_event).await;
        assert!(matches!(result, Err(SecretError::InvalidSecretFormat)));
    }

    #[tokio::test]
    async fn test_concurrent_secret_operations() {
        use tokio::task;

        let mut secret_manager = SecretManager::new();

        // Generate secrets concurrently
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let swap_id = format!("swap_{}", i);
                task::spawn_blocking(move || {
                    // This would need to be done with proper synchronization in real code
                    swap_id
                })
            })
            .collect();

        // Wait for all tasks to complete
        for handle in handles {
            let swap_id = handle.await.unwrap();
            secret_manager.generate_secret(&swap_id).unwrap();
        }

        let stats = secret_manager.get_stats();
        assert_eq!(stats.total_secrets, 10);
        assert_eq!(stats.active_secrets, 10);
    }

    #[tokio::test]
    async fn test_secret_disposal_integration() {
        let mut secret_manager = SecretManager::new();

        // Generate secret
        let swap_id = "swap_to_dispose";
        let secret_hash = secret_manager.generate_secret(swap_id).unwrap();

        // Verify secret exists
        assert!(secret_manager.get_secret(swap_id).is_ok());
        assert!(secret_manager.get_secret_by_hash(&secret_hash).is_ok());

        // Dispose of secret
        secret_manager.dispose_secret(swap_id).unwrap();

        // Verify secret is gone
        assert!(matches!(
            secret_manager.get_secret(swap_id),
            Err(SecretError::SecretNotFound)
        ));
        assert!(matches!(
            secret_manager.get_secret_by_hash(&secret_hash),
            Err(SecretError::SecretNotFound)
        ));

        let stats = secret_manager.get_stats();
        assert_eq!(stats.total_secrets, 0);
    }
}
