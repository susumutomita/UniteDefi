use fusion_core::htlc::{generate_secret, hash_secret, Htlc};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_concurrent_claim_attempts() {
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    let htlc = Arc::new(Mutex::new(
        Htlc::new(
            "Alice".to_string(),
            "Bob".to_string(),
            1000,
            secret_hash,
            Duration::from_secs(3600),
        )
        .expect("Failed to create HTLC"),
    ));

    // 複数のスレッドから同時にクレームを試みる
    let mut handles = vec![];

    for i in 0..10 {
        let htlc_clone = Arc::clone(&htlc);
        let secret_clone = secret;
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10)); // 少し待機して同時実行を促す
            let mut htlc_guard = htlc_clone.lock().unwrap();
            let result = htlc_guard.claim(&secret_clone);
            println!("Thread {i}: claim result = {result:?}");
            result
        });
        handles.push(handle);
    }

    // 結果を収集
    let mut success_count = 0;
    let mut failure_count = 0;

    for handle in handles {
        match handle.join().unwrap() {
            Ok(_) => success_count += 1,
            Err(_) => failure_count += 1,
        }
    }

    // 1つのスレッドだけが成功し、残りは失敗するはず
    assert_eq!(
        success_count, 1,
        "Only one thread should successfully claim"
    );
    assert_eq!(failure_count, 9, "Nine threads should fail to claim");
}

#[test]
fn test_concurrent_refund_after_timeout() {
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    let htlc = Arc::new(Mutex::new(
        Htlc::new(
            "Alice".to_string(),
            "Bob".to_string(),
            1000,
            secret_hash,
            Duration::from_secs(1), // 1秒でタイムアウト
        )
        .expect("Failed to create HTLC"),
    ));

    // タイムアウトを待つ
    thread::sleep(Duration::from_secs(2));

    // 複数のスレッドから同時にリファンドを試みる
    let mut handles = vec![];

    for i in 0..5 {
        let htlc_clone = Arc::clone(&htlc);
        let handle = thread::spawn(move || {
            let mut htlc_guard = htlc_clone.lock().unwrap();
            let result = htlc_guard.refund();
            println!("Thread {i}: refund result = {result:?}");
            result
        });
        handles.push(handle);
    }

    // 結果を収集
    let mut success_count = 0;
    let mut failure_count = 0;

    for handle in handles {
        match handle.join().unwrap() {
            Ok(_) => success_count += 1,
            Err(_) => failure_count += 1,
        }
    }

    // 1つのスレッドだけが成功し、残りは失敗するはず
    assert_eq!(
        success_count, 1,
        "Only one thread should successfully refund"
    );
    assert_eq!(failure_count, 4, "Four threads should fail to refund");
}

#[test]
fn test_claim_refund_race_condition() {
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    let htlc = Arc::new(Mutex::new(
        Htlc::new(
            "Alice".to_string(),
            "Bob".to_string(),
            1000,
            secret_hash,
            Duration::from_secs(1), // 1秒でタイムアウト
        )
        .expect("Failed to create HTLC"),
    ));

    // タイムアウトぎりぎりで競合を起こす
    thread::sleep(Duration::from_millis(990));

    let htlc_clone1 = Arc::clone(&htlc);
    let secret_clone = secret;

    // クレームを試みるスレッド
    let claim_thread = thread::spawn(move || {
        let mut htlc_guard = htlc_clone1.lock().unwrap();
        htlc_guard.claim(&secret_clone)
    });

    let htlc_clone2 = Arc::clone(&htlc);

    // 少し待ってからリファンドを試みるスレッド
    let refund_thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50)); // タイムアウト後
        let mut htlc_guard = htlc_clone2.lock().unwrap();
        htlc_guard.refund()
    });

    let claim_result = claim_thread.join().unwrap();
    let refund_result = refund_thread.join().unwrap();

    // どちらか一方だけが成功するはず
    match (claim_result, refund_result) {
        (Ok(_), Err(_)) => {
            println!("Claim succeeded, refund failed (expected)");
        }
        (Err(_), Ok(_)) => {
            println!("Claim failed, refund succeeded (also valid if timeout occurred)");
        }
        _ => panic!("Unexpected result: both succeeded or both failed"),
    }
}
