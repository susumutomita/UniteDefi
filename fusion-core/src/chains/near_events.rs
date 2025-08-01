use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid log format: {0}")]
    InvalidFormat(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid value: {0}")]
    InvalidValue(String),
}

/// NEAR HTLCイベント構造体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NearHtlcCreateEvent {
    pub escrow_id: String,
    pub resolver: String,
    pub beneficiary: String,
    pub amount: u128,
    pub secret_hash: String,
    pub finality_time: u64,
    pub cancel_time: u64,
    pub public_cancel_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NearHtlcClaimEvent {
    pub escrow_id: String,
    pub claimer: String,
    pub secret: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NearHtlcCancelEvent {
    pub escrow_id: String,
    pub canceller: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NearHtlcEvent {
    Create(NearHtlcCreateEvent),
    Claim(NearHtlcClaimEvent),
    Cancel(NearHtlcCancelEvent),
}

/// NEARイベントパーサー
pub struct NearEventParser;

impl NearEventParser {
    /// Createイベントのパース
    pub fn parse_create_event(log: &str) -> Result<NearHtlcCreateEvent, ParseError> {
        // Example log format:
        // "Fusion escrow created: fusion_0 by alice.near for bob.near, amount: 1000000, safety: 100000"
        
        if !log.starts_with("Fusion escrow created:") {
            return Err(ParseError::InvalidFormat("Not a create event".to_string()));
        }
        
        // Simple parsing implementation for initial test
        // TODO: Implement robust regex-based parsing
        
        let parts: Vec<&str> = log.split_whitespace().collect();
        if parts.len() < 11 {
            return Err(ParseError::InvalidFormat("Insufficient parts in log".to_string()));
        }
        
        let escrow_id = parts[3].to_string();
        let resolver = parts[5].to_string();
        let beneficiary = parts[7].trim_end_matches(',').to_string();
        
        // Extract amount
        let amount_str = parts[9].trim_end_matches(',');
        let amount = amount_str.parse::<u128>()
            .map_err(|_| ParseError::InvalidValue(format!("Invalid amount: {}", amount_str)))?;
        
        // For now, return a simple event with default values for missing fields
        Ok(NearHtlcCreateEvent {
            escrow_id,
            resolver,
            beneficiary,
            amount,
            secret_hash: String::new(), // TODO: Extract from actual contract event
            finality_time: 0,
            cancel_time: 0,
            public_cancel_time: 0,
        })
    }
    
    /// Claimイベントのパース
    pub fn parse_claim_event(log: &str) -> Result<NearHtlcClaimEvent, ParseError> {
        // Example log format:
        // "Secret revealed: deadbeef1234567890abcdef"
        
        if !log.starts_with("Secret revealed:") {
            return Err(ParseError::InvalidFormat("Not a claim event".to_string()));
        }
        
        let parts: Vec<&str> = log.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(ParseError::InvalidFormat("Missing secret in log".to_string()));
        }
        
        let secret = parts[2].to_string();
        
        // TODO: Extract actual escrow_id and claimer from full event context
        Ok(NearHtlcClaimEvent {
            escrow_id: String::new(),
            claimer: String::new(),
            secret,
            timestamp: 0,
        })
    }
    
    /// Cancelイベントのパース
    pub fn parse_cancel_event(log: &str) -> Result<NearHtlcCancelEvent, ParseError> {
        // TODO: Implement cancel event parsing
        Err(ParseError::InvalidFormat("Cancel event parsing not implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn should_define_near_htlc_event_structures() {
        // NEAR HTLCイベント構造体が存在し、適切なフィールドを持つことを確認
        let create_event = NearHtlcCreateEvent {
            escrow_id: "fusion_0".to_string(),
            resolver: "alice.near".to_string(),
            beneficiary: "bob.near".to_string(),
            amount: 1000000000000000000000000,
            secret_hash: "test_hash".to_string(),
            finality_time: 3600,
            cancel_time: 7200,
            public_cancel_time: 10800,
        };
        
        assert_eq!(create_event.escrow_id, "fusion_0");
        assert_eq!(create_event.resolver, "alice.near");
        assert_eq!(create_event.beneficiary, "bob.near");
        assert_eq!(create_event.amount, 1000000000000000000000000);
        assert_eq!(create_event.secret_hash, "test_hash");
    }
    
    #[test]
    fn should_parse_near_log_into_event() {
        let log_message = "Fusion escrow created: fusion_0 by alice.near for bob.near, amount: 1000000000000000000000000, safety: 0";
        
        let event = NearEventParser::parse_create_event(log_message).unwrap();
        
        assert_eq!(event.escrow_id, "fusion_0");
        assert_eq!(event.resolver, "alice.near");
        assert_eq!(event.beneficiary, "bob.near");
        assert_eq!(event.amount, 1000000000000000000000000);
    }
    
    #[test]
    fn should_fail_on_invalid_log_format() {
        let invalid_log = "Some random log message";
        
        let result = NearEventParser::parse_create_event(invalid_log);
        assert!(result.is_err());
        
        match result {
            Err(ParseError::InvalidFormat(msg)) => {
                assert_eq!(msg, "Not a create event");
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }
    
    #[test]
    fn should_parse_claim_event_with_secret() {
        let log_message = "Secret revealed: deadbeef1234567890abcdef";
        
        let event = NearEventParser::parse_claim_event(log_message).unwrap();
        
        assert_eq!(event.secret, "deadbeef1234567890abcdef");
    }
}