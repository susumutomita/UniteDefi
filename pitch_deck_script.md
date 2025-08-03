# UniteSwap Pitch Deck Script (4 minutes)

## Time Allocation:
- Slide 1: Title (10 seconds)
- Slide 2: Problem (30 seconds) 
- Slide 3: How It Works (40 seconds)
- Slide 4: Live Demo (2 minutes 20 seconds)
- Slide 5: Technical Implementation & Thank You (20 seconds)

---

## Slide 1: Title (10 seconds)

**"Hi everyone! I'm here to show you UniteSwap - the first trustless atomic swap solution between EVM and NEAR chains, powered by HTLC technology and 1inch Fusion protocol."**

---

## Slide 2: The Problem (30 seconds)

**"Here's the real problem - there's NO trustless bridge between EVM chains and NEAR Protocol.**

**Think about it - NEAR has amazing technology and a growing ecosystem, but it's completely isolated from Ethereum DeFi.**

**If you want to move assets between Ethereum and NEAR today, you need to use a centralized exchange or chain multiple bridges together. It's slow, expensive, and risky.**

**We're talking about billions in liquidity that can't freely flow between these ecosystems.**

**Our solution? We built the FIRST atomic swap bridge between EVM and NEAR using 1inch's Limit Order Protocol and HTLCs. Direct, trustless, atomic."**

---

## Slide 3: How It Works (30 seconds)

**"Here's our architecture. On the left, we have Ethereum with 1inch Fusion integration. On the right, NEAR with our HTLC contract.**

**In the middle, our Rust-based Fusion Core monitors both chains in real-time.**

**Our key innovation is being the first to integrate 1inch's Fusion protocol with NEAR HTLCs, enabling truly atomic cross-chain swaps.**

**When a user initiates a swap, matching HTLCs are created on both chains with the same secret hash. Both succeed or both fail - guaranteed atomically."**

---

## Slide 4: Live Demo (2 minutes)

**"Let me show you a real atomic swap in action."**

[Start terminal/demo]

**"Here's our scenario: Alice wants to swap her ETH for NEAR tokens."**

[Execute first command]
```bash
fusion-cli order create --from-chain ethereum --to-chain near --amount 1 --token ETH
```

**"Watch as the order is created and the HTLC deploys on Ethereum..."**

[Show output]

**"Now Bob creates a matching order on NEAR..."**

[Execute second command]
```bash
fusion-cli order create --from-chain near --to-chain ethereum --amount 1000 --token NEAR
```

**"See how our event monitors detect both HTLCs? They're cryptographically linked by the same secret hash."**

[Show monitoring output]

**"Now watch the magic - when Bob reveals the secret to claim his ETH, our system automatically uses that same secret to claim NEAR for Alice."**

[Show claim execution]

**"There it is! A complete atomic swap in under 30 seconds. No trust, no intermediary, just pure cryptographic guarantees."**

**"Notice how both transactions either succeed together or fail together. That's the beauty of atomic swaps."**

---

## Slide 5: Technical Implementation & Thank You (20 seconds)

**"Here's what powers UniteSwap:**

**Official 1inch integration on Ethereum, custom HTLC on NEAR, and our Rust CLI orchestrating everything.**

**That's UniteSwap - the first production-ready NEAR to EVM atomic swap solution.**

**Clone our repo on GitHub and try it yourself. Thank you!"**

---

## Q&A Preparation (Extra time)

### Quick answers to likely questions:

**Q: What about different finality times?**
A: "We use conservative timeouts that accommodate both chains' finality."

**Q: What if a chain goes down?**
A: "Time locks ensure automatic refunds - funds are never stuck."

**Q: Gas costs?**
A: "Actually cheaper than bridges - no bridge fees, just HTLC transactions."

**Q: Other chains?**
A: "Any chain supporting HTLCs can be added - we started with NEAR and EVM."

---

## Demo Troubleshooting

If demo fails:
1. **Network issues**: "Let me show you the recorded demo instead..."
2. **Contract issues**: "Here's what would happen..." [explain flow]
3. **Always have backup**: Screenshots or recorded video ready