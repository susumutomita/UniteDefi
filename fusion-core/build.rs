use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../contracts/ethereum/src/");

    // プロジェクトのルートディレクトリを取得
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = PathBuf::from(&manifest_dir).parent().unwrap().to_path_buf();
    let contracts_dir = project_root.join("contracts").join("ethereum");

    // Foundryでコントラクトをコンパイル
    let output = Command::new("forge")
        .arg("build")
        .current_dir(&contracts_dir)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "Forge build failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                // ビルドが失敗してもRustのビルドは継続（開発中のため）
            }
        }
        Err(e) => {
            eprintln!(
                "Failed to run forge: {}. Make sure Foundry is installed.",
                e
            );
            // Foundryがインストールされていない場合もRustのビルドは継続
        }
    }

    // ABIファイルのパスを設定
    let out_dir = contracts_dir.join("out");
    let escrow_abi = out_dir.join("IEscrow.sol").join("IEscrow.json");
    let factory_abi = out_dir
        .join("IEscrowFactory.sol")
        .join("IEscrowFactory.json");

    // ABIファイルが存在する場合のみバインディングを生成
    if escrow_abi.exists() && factory_abi.exists() {
        // ethers-rsのabigenを使用してバインディングを生成
        ethers::contract::Abigen::new("IEscrow", escrow_abi.to_str().unwrap())
            .unwrap()
            .generate()
            .unwrap()
            .write_to_file(PathBuf::from(&manifest_dir).join("src/chains/ethereum/abi/escrow.rs"))
            .unwrap();

        ethers::contract::Abigen::new("IEscrowFactory", factory_abi.to_str().unwrap())
            .unwrap()
            .generate()
            .unwrap()
            .write_to_file(PathBuf::from(&manifest_dir).join("src/chains/ethereum/abi/factory.rs"))
            .unwrap();
    }
}
