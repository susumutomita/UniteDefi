# UniteSwap FAQ (Frequently Asked Questions)

## General Questions

### Q: What is UniteSwap?
**A:** UniteSwap is a Rust CLI implementation that extends the 1inch Fusion+ protocol to enable trustless atomic swaps between EVM and non-EVM chains (NEAR, Cosmos, Stellar).

### Q: How does it differ from regular 1inch Fusion+?
**A:** While 1inch Fusion+ focuses on EVM chains, UniteSwap adds support for non-EVM chains using Hash Time Lock Contracts (HTLCs) while preserving all security guarantees.

### Q: Which chains are supported?
**A:** Currently supported:
- EVM: Ethereum, Polygon, Arbitrum, Optimism, Base
- Non-EVM: NEAR (implemented), Cosmos and Stellar (architecture ready)

## Technical Questions

### Q: How do HTLCs work in UniteSwap?
**A:** HTLCs use cryptographic hashes and time locks to ensure atomic swaps:
1. Alice locks funds with a secret hash
2. Bob locks corresponding funds with the same hash
3. Alice reveals the secret to claim Bob's funds
4. Bob uses the revealed secret to claim Alice's funds

### Q: What happens if a swap fails?
**A:** If the secret isn't revealed before timeout:
- Both parties can refund their locked funds
- No funds are lost
- The atomic nature ensures all-or-nothing execution

### Q: How are orders matched?
**A:** Orders are matched through:
1. On-chain orderbook for each supported chain
2. Cross-chain relayers monitor and match compatible orders
3. HTLC creation ensures atomic execution

## Usage Questions

### Q: Do I need to run my own node?
**A:** No, the CLI can connect to public RPC endpoints for all supported chains.

### Q: What are the fees?
**A:** Fees include:
- Network gas fees on each chain
- No additional protocol fees during hackathon phase
- Future versions may include small relayer fees

### Q: How long do swaps take?
**A:** Typical swap times:
- Same-chain: 1-2 minutes
- Cross-chain: 5-10 minutes
- Depends on block times of involved chains

## Security Questions

### Q: Is it safe to use?
**A:** Yes, security features include:
- Cryptographic hash locks prevent theft
- Time locks ensure refundability
- No custody of funds by third parties
- Open source and auditable code

### Q: What if I lose my secret?
**A:** If you lose the secret:
- You cannot claim the counterparty's funds
- After timeout, both parties can refund
- Always save the secret when creating HTLCs

### Q: Can someone steal my funds?
**A:** No, funds are protected by:
- Cryptographic secrets (practically unguessable)
- Time locks for refund protection
- Smart contract enforcement

## Development Questions

### Q: How can I add a new chain?
**A:** To add a new chain:
1. Implement the `HTLCContract` trait
2. Add chain-specific configuration
3. Update the CLI to support the new chain
4. Submit a PR with tests

### Q: Can I use this in production?
**A:** Currently in beta/hackathon phase:
- Testnet deployments are active
- Mainnet deployments planned post-audit
- Use at your own risk on mainnet

### Q: How can I contribute?
**A:** Contributions welcome:
1. Check open issues on GitHub
2. Review contribution guidelines
3. Submit PRs with tests
4. Join our Discord for discussions

## Troubleshooting Questions

### Q: Why does my order show as "not found"?
**A:** Possible reasons:
- Order ID is incorrect
- Order was cancelled or filled
- Looking on wrong chain

### Q: Why can't I claim my HTLC?
**A:** Common issues:
- Wrong secret provided
- HTLC already claimed
- Not the designated recipient

### Q: Build fails with Rust errors?
**A:** Solutions:
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build`
- Check minimum Rust version (1.75+)

## Hackathon Specific

### Q: Which track does this project target?
**A:** Track 1: Cross-chain Swap Extension - extending 1inch Fusion+ to non-EVM chains.

### Q: What are the main achievements?
**A:** 
- ✅ Bidirectional swaps (EVM ↔ non-EVM)
- ✅ Preserved security guarantees
- ✅ Working CLI implementation
- ✅ NEAR integration complete
- ✅ Modular architecture for new chains

### Q: Where can I see a demo?
**A:** 
- Run `./demo_verification.sh` for automated demo
- Check `DEMO_SCENARIOS.md` for manual walkthroughs
- Video demo available in submission

## Contact & Support

### Q: Where can I get help?
**A:** 
- GitHub Issues: [github.com/susumutomita/UniteDefi/issues](https://github.com/susumutomita/UniteDefi/issues)
- Documentation: See README.md and docs/
- Code examples: Check examples/ directory

### Q: Is there a UI?
**A:** Currently CLI-only for the hackathon. Web UI is planned as a future enhancement.