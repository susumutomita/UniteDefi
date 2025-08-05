---
marp: true
theme: default
paginate: true
backgroundColor: #f5f5f5
color: #333
style: |
  section {
    font-family: 'Arial', sans-serif;
    padding: 60px;
  }
  h1 {
    color: #2c3e50;
    font-size: 2em;
    margin-bottom: 20px;
  }
  h2 {
    color: #34495e;
    font-size: 1.5em;
    margin-bottom: 15px;
  }
  h3 {
    color: #7f8c8d;
    font-size: 1.2em;
  }
  code {
    background-color: #ecf0f1;
    color: #2c3e50;
    font-size: 0.9em;
  }
  ul {
    font-size: 1.1em;
    line-height: 1.8;
  }
  strong {
    color: #e74c3c;
  }
---

# UniteSwap

## First EVM ↔️ NEAR Atomic Swaps

1inch Fusion+ Extension

---

# The Problem

**No bridge between EVM and NEAR**

- NEAR isolated from Ethereum DeFi
- Need CEX or multiple hops
- Billions in liquidity disconnected

**Solution: Direct atomic swaps**

---

# How It Works

**Ethereum (Base Sepolia)**
→ 1inch Limit Order Protocol

**↔️ Atomic Swap ↔️**

**NEAR Protocol**
→ Custom HTLC Contract

**Orchestration: `fusion-cli` (Rust)**

---

# Live Demo

1. **Create HTLC** → Get secret
2. **Create Order** → 1inch protocol

```bash
fusion-cli create-htlc --sender 0x7aD8...
fusion-cli order create --htlc-secret-hash...
```

---

# Technical Implementation

**✅ Official 1inch Integration**
- Limit Order Protocol: `0x171C87...`

**✅ NEAR HTLC Contract**
- Deployed: `htlc-v2.testnet`

**✅ Rust CLI Tool**
- Complete atomic swap flow

---

# Thank You!

## UniteSwap - EVM ↔️ NEAR Bridge

**GitHub**: github.com/susumutomita/UniteDefi
