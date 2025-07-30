// ABIバインディングモジュール
// これらのファイルはbuild.rsによって自動生成される

#[allow(dead_code)]
#[allow(clippy::all)]
pub mod escrow {
    // build.rsによって生成されるまでダミー実装
    use ethers::prelude::*;
    
    abigen!(
        IEscrow,
        r#"[
            {
                "inputs": [{"internalType": "bytes32", "name": "secret", "type": "bytes32"}],
                "name": "claim",
                "outputs": [],
                "stateMutability": "nonpayable",
                "type": "function"
            },
            {
                "inputs": [],
                "name": "refund",
                "outputs": [],
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]"#
    );
}

#[allow(dead_code)]
#[allow(clippy::all)]
pub mod factory {
    // build.rsによって生成されるまでダミー実装
    use ethers::prelude::*;
    
    abigen!(
        IEscrowFactory,
        r#"[
            {
                "inputs": [
                    {"internalType": "address", "name": "token", "type": "address"},
                    {"internalType": "uint256", "name": "amount", "type": "uint256"},
                    {"internalType": "bytes32", "name": "secretHash", "type": "bytes32"},
                    {"internalType": "uint256", "name": "timeout", "type": "uint256"},
                    {"internalType": "address", "name": "recipient", "type": "address"}
                ],
                "name": "createEscrow",
                "outputs": [{"internalType": "address", "name": "escrow", "type": "address"}],
                "stateMutability": "payable",
                "type": "function"
            }
        ]"#
    );
}