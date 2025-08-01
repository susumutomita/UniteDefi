//! 自動実行エンジン
//! 
//! クロスチェーントランザクションを自動的に実行し、監視します。

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::interval;

use crate::cross_chain_executor::{CrossChainExecutor, ExecutionParams, ExecutionState};
use crate::execution_path_optimizer::{ExecutionPath, ExecutionStep, StepType};
use crate::order_matching_engine::OrderMatch;

/// 自動実行タスク
#[derive(Debug, Clone)]
pub struct ExecutionTask {
    /// タスクID
    pub id: String,
    /// オーダーマッチ
    pub order_match: OrderMatch,
    /// 実行パス
    pub execution_path: ExecutionPath,
    /// ステータス
    pub status: TaskStatus,
    /// 作成時刻
    pub created_at: u64,
    /// 更新時刻
    pub updated_at: u64,
    /// エラーメッセージ
    pub error_message: Option<String>,
}

/// タスクステータス
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    /// 待機中
    Pending,
    /// 実行中
    Executing {
        current_step: usize,
        step_status: StepStatus,
    },
    /// 完了
    Completed {
        tx_hashes: Vec<String>,
    },
    /// 失敗
    Failed {
        reason: String,
        retry_count: u8,
    },
    /// キャンセル済み
    Cancelled,
}

/// ステップステータス
#[derive(Debug, Clone, PartialEq)]
pub enum StepStatus {
    /// 開始前
    NotStarted,
    /// 実行中
    InProgress,
    /// 完了
    Completed,
    /// 失敗
    Failed,
}

/// 実行エンジンのトレイト
#[async_trait]
pub trait ExecutionEngine: Send + Sync {
    /// タスクを実行
    async fn execute_task(&mut self, task: &ExecutionTask) -> Result<TaskStatus>;
    
    /// タスクの進捗を取得
    async fn get_task_progress(&self, task_id: &str) -> Result<TaskStatus>;
    
    /// タスクをキャンセル
    async fn cancel_task(&mut self, task_id: &str) -> Result<()>;
}

/// 自動実行マネージャー
pub struct AutomatedExecutor {
    /// 実行エンジン
    engine: Box<dyn ExecutionEngine>,
    /// タスクキュー
    task_queue: Arc<Mutex<Vec<ExecutionTask>>>,
    /// アクティブタスク
    active_tasks: Arc<Mutex<HashMap<String, ExecutionTask>>>,
    /// 最大同時実行数
    max_concurrent_tasks: usize,
    /// リトライ設定
    retry_config: RetryConfig,
}

/// リトライ設定
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// 最大リトライ回数
    pub max_retries: u8,
    /// リトライ間隔（秒）
    pub retry_delay: u64,
    /// 指数バックオフ
    pub exponential_backoff: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: 60,
            exponential_backoff: true,
        }
    }
}

/// 標準実行エンジン
pub struct StandardExecutionEngine {
    /// クロスチェーン実行器
    cross_chain_executor: CrossChainExecutor,
    /// 実行ログ
    execution_log: Vec<ExecutionLog>,
}

/// 実行ログ
#[derive(Debug, Clone)]
struct ExecutionLog {
    /// タイムスタンプ
    timestamp: u64,
    /// タスクID
    task_id: String,
    /// メッセージ
    message: String,
    /// ログレベル
    level: LogLevel,
}

/// ログレベル
#[derive(Debug, Clone)]
enum LogLevel {
    Info,
    Warning,
    Error,
}

impl StandardExecutionEngine {
    pub fn new(cross_chain_executor: CrossChainExecutor) -> Self {
        Self {
            cross_chain_executor,
            execution_log: Vec::new(),
        }
    }

    /// ログを追加
    fn add_log(&mut self, task_id: String, message: String, level: LogLevel) {
        self.execution_log.push(ExecutionLog {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            task_id,
            message,
            level,
        });
    }
}

#[async_trait]
impl ExecutionEngine for StandardExecutionEngine {
    async fn execute_task(&mut self, task: &ExecutionTask) -> Result<TaskStatus> {
        self.add_log(
            task.id.clone(),
            format!("Starting execution of task {}", task.id),
            LogLevel::Info,
        );

        let mut tx_hashes = Vec::new();

        // 各ステップを順番に実行
        for (i, step) in task.execution_path.steps.iter().enumerate() {
            self.add_log(
                task.id.clone(),
                format!("Executing step {}: {:?}", i, step.step_type),
                LogLevel::Info,
            );

            match step.step_type {
                StepType::Bridge => {
                    // ブリッジ実行（仮実装）
                    let tx_hash = format!("bridge_tx_{}", i);
                    tx_hashes.push(tx_hash);
                }
                StepType::Swap => {
                    // スワップ実行（仮実装）
                    let tx_hash = format!("swap_tx_{}", i);
                    tx_hashes.push(tx_hash);
                }
                StepType::HTLCCreate => {
                    // HTLC作成（仮実装）
                    let tx_hash = format!("htlc_create_tx_{}", i);
                    tx_hashes.push(tx_hash);
                }
                StepType::HTLCClaim => {
                    // HTLCクレーム（仮実装）
                    let tx_hash = format!("htlc_claim_tx_{}", i);
                    tx_hashes.push(tx_hash);
                }
                StepType::LimitOrderExecution => {
                    // リミットオーダー実行（仮実装）
                    let tx_hash = format!("limit_order_tx_{}", i);
                    tx_hashes.push(tx_hash);
                }
            }

            // ステップ間の待機
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        self.add_log(
            task.id.clone(),
            format!("Task {} completed successfully", task.id),
            LogLevel::Info,
        );

        Ok(TaskStatus::Completed { tx_hashes })
    }

    async fn get_task_progress(&self, task_id: &str) -> Result<TaskStatus> {
        // 仮実装：実際にはトランザクションの状態を確認
        Ok(TaskStatus::Pending)
    }

    async fn cancel_task(&mut self, task_id: &str) -> Result<()> {
        self.add_log(
            task_id.to_string(),
            format!("Cancelling task {}", task_id),
            LogLevel::Warning,
        );
        Ok(())
    }
}

use std::collections::HashMap;

impl AutomatedExecutor {
    /// 新しい自動実行マネージャーを作成
    pub fn new(
        engine: Box<dyn ExecutionEngine>,
        max_concurrent_tasks: usize,
        retry_config: RetryConfig,
    ) -> Self {
        Self {
            engine,
            task_queue: Arc::new(Mutex::new(Vec::new())),
            active_tasks: Arc::new(Mutex::new(HashMap::new())),
            max_concurrent_tasks,
            retry_config,
        }
    }

    /// タスクを追加
    pub fn add_task(&mut self, task: ExecutionTask) -> Result<()> {
        let mut queue = self.task_queue.lock().unwrap();
        queue.push(task);
        Ok(())
    }

    /// 実行ループを開始
    pub async fn start_execution_loop(&mut self) {
        let mut interval = interval(Duration::from_secs(5));

        loop {
            interval.tick().await;

            // アクティブタスク数をチェック
            let active_count = self.active_tasks.lock().unwrap().len();
            if active_count >= self.max_concurrent_tasks {
                continue;
            }

            // キューから次のタスクを取得
            let next_task = {
                let mut queue = self.task_queue.lock().unwrap();
                queue.pop()
            };

            if let Some(mut task) = next_task {
                // タスクを実行
                match self.engine.execute_task(&task).await {
                    Ok(status) => {
                        task.status = status;
                        task.updated_at = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                    }
                    Err(e) => {
                        task.status = TaskStatus::Failed {
                            reason: e.to_string(),
                            retry_count: 0,
                        };
                        task.error_message = Some(e.to_string());
                    }
                }

                // アクティブタスクに追加
                self.active_tasks.lock().unwrap().insert(task.id.clone(), task);
            }

            // 完了/失敗タスクの処理
            self.process_completed_tasks().await;
        }
    }

    /// 完了/失敗タスクを処理
    async fn process_completed_tasks(&mut self) {
        let mut tasks_to_retry = Vec::new();
        let mut completed_task_ids = Vec::new();

        {
            let active_tasks = self.active_tasks.lock().unwrap();
            for (id, task) in active_tasks.iter() {
                match &task.status {
                    TaskStatus::Completed { .. } => {
                        completed_task_ids.push(id.clone());
                    }
                    TaskStatus::Failed { retry_count, .. } => {
                        if *retry_count < self.retry_config.max_retries {
                            tasks_to_retry.push(task.clone());
                        } else {
                            completed_task_ids.push(id.clone());
                        }
                    }
                    _ => {}
                }
            }
        }

        // 完了タスクを削除
        {
            let mut active_tasks = self.active_tasks.lock().unwrap();
            for id in completed_task_ids {
                active_tasks.remove(&id);
            }
        }

        // リトライタスクを再キュー
        for mut task in tasks_to_retry {
            if let TaskStatus::Failed { retry_count, .. } = &mut task.status {
                *retry_count += 1;
                task.status = TaskStatus::Pending;
                
                // リトライ遅延を適用
                let delay = if self.retry_config.exponential_backoff {
                    self.retry_config.retry_delay * 2u64.pow(*retry_count as u32)
                } else {
                    self.retry_config.retry_delay
                };
                
                tokio::time::sleep(Duration::from_secs(delay)).await;
                
                self.add_task(task).unwrap();
            }
        }
    }

    /// タスクのステータスを取得
    pub fn get_task_status(&self, task_id: &str) -> Option<TaskStatus> {
        self.active_tasks
            .lock()
            .unwrap()
            .get(task_id)
            .map(|t| t.status.clone())
    }

    /// 全タスクのステータスサマリーを取得
    pub fn get_status_summary(&self) -> HashMap<String, usize> {
        let mut summary = HashMap::new();
        
        let queue_count = self.task_queue.lock().unwrap().len();
        summary.insert("pending".to_string(), queue_count);
        
        let active_tasks = self.active_tasks.lock().unwrap();
        let executing_count = active_tasks
            .values()
            .filter(|t| matches!(t.status, TaskStatus::Executing { .. }))
            .count();
        summary.insert("executing".to_string(), executing_count);
        
        let completed_count = active_tasks
            .values()
            .filter(|t| matches!(t.status, TaskStatus::Completed { .. }))
            .count();
        summary.insert("completed".to_string(), completed_count);
        
        let failed_count = active_tasks
            .values()
            .filter(|t| matches!(t.status, TaskStatus::Failed { .. }))
            .count();
        summary.insert("failed".to_string(), failed_count);
        
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cross_chain_executor::CrossChainExecutor;

    #[test]
    fn test_task_creation() {
        let task = ExecutionTask {
            id: "test_task_1".to_string(),
            order_match: OrderMatch {
                buy_order_id: "buy1".to_string(),
                sell_order_id: "sell1".to_string(),
                match_price: 5.0,
                match_amount: 1000,
                profit_bps: 100,
            },
            execution_path: ExecutionPath {
                id: "path1".to_string(),
                steps: vec![],
                total_cost: 10.0,
                total_time: 300,
                risk_score: 20,
                expected_profit: 5.0,
            },
            status: TaskStatus::Pending,
            created_at: 1234567890,
            updated_at: 1234567890,
            error_message: None,
        };

        assert_eq!(task.status, TaskStatus::Pending);
        assert!(task.error_message.is_none());
    }

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay, 60);
        assert!(config.exponential_backoff);
    }

    #[tokio::test]
    async fn test_standard_execution_engine() {
        let cross_chain_executor = CrossChainExecutor::new(
            "https://eth.example.com",
            "0x0000000000000000000000000000000000000000",
            "https://near.example.com",
        ).unwrap();

        let mut engine = StandardExecutionEngine::new(cross_chain_executor);

        let task = ExecutionTask {
            id: "test_task".to_string(),
            order_match: OrderMatch {
                buy_order_id: "buy1".to_string(),
                sell_order_id: "sell1".to_string(),
                match_price: 5.0,
                match_amount: 1000,
                profit_bps: 100,
            },
            execution_path: ExecutionPath {
                id: "path1".to_string(),
                steps: vec![
                    ExecutionStep {
                        step_type: StepType::Bridge,
                        source_chain: "ethereum".to_string(),
                        target_chain: "near".to_string(),
                        token: "USDC".to_string(),
                        amount: 1000,
                        estimated_cost: 10.0,
                        estimated_time: 300,
                    },
                ],
                total_cost: 10.0,
                total_time: 300,
                risk_score: 20,
                expected_profit: 5.0,
            },
            status: TaskStatus::Pending,
            created_at: 1234567890,
            updated_at: 1234567890,
            error_message: None,
        };

        let result = engine.execute_task(&task).await.unwrap();
        
        match result {
            TaskStatus::Completed { tx_hashes } => {
                assert!(!tx_hashes.is_empty());
                assert_eq!(tx_hashes[0], "bridge_tx_0");
            }
            _ => panic!("Expected completed status"),
        }
    }

    #[test]
    fn test_automated_executor_creation() {
        let cross_chain_executor = CrossChainExecutor::new(
            "https://eth.example.com",
            "0x0000000000000000000000000000000000000000",
            "https://near.example.com",
        ).unwrap();

        let engine = Box::new(StandardExecutionEngine::new(cross_chain_executor));
        let retry_config = RetryConfig::default();

        let executor = AutomatedExecutor::new(engine, 5, retry_config);
        
        assert_eq!(executor.max_concurrent_tasks, 5);
    }

    #[test]
    fn test_task_status_transitions() {
        let status = TaskStatus::Pending;
        assert_eq!(status, TaskStatus::Pending);

        let status = TaskStatus::Executing {
            current_step: 1,
            step_status: StepStatus::InProgress,
        };
        
        match status {
            TaskStatus::Executing { current_step, step_status } => {
                assert_eq!(current_step, 1);
                assert_eq!(step_status, StepStatus::InProgress);
            }
            _ => panic!("Expected executing status"),
        }

        let status = TaskStatus::Failed {
            reason: "Test error".to_string(),
            retry_count: 2,
        };
        
        match status {
            TaskStatus::Failed { reason, retry_count } => {
                assert_eq!(reason, "Test error");
                assert_eq!(retry_count, 2);
            }
            _ => panic!("Expected failed status"),
        }
    }

    #[test]
    fn test_status_summary() {
        let cross_chain_executor = CrossChainExecutor::new(
            "https://eth.example.com",
            "0x0000000000000000000000000000000000000000",
            "https://near.example.com",
        ).unwrap();

        let engine = Box::new(StandardExecutionEngine::new(cross_chain_executor));
        let retry_config = RetryConfig::default();

        let executor = AutomatedExecutor::new(engine, 5, retry_config);
        
        let summary = executor.get_status_summary();
        assert_eq!(summary.get("pending").unwrap_or(&0), &0);
        assert_eq!(summary.get("executing").unwrap_or(&0), &0);
        assert_eq!(summary.get("completed").unwrap_or(&0), &0);
        assert_eq!(summary.get("failed").unwrap_or(&0), &0);
    }
}