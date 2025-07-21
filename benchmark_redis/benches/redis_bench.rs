use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use deadpool_redis::redis::AsyncTypedCommands;
use deadpool_redis::{Config as DeadpoolConfig, Pool, Runtime as DeadpoolRuntime};
use futures::future::join_all;
use rand::{Rng, thread_rng};
use redis::{AsyncCommands, Client, aio::ConnectionManager};
use std::time::Duration;
use tokio::runtime::Runtime;

const REDIS_URL: &str = "redis://127.0.0.1:6380";
const NUM_CONCURRENT_TASKS: usize = 200;
const OPS_PER_TASK: usize = 500;
const TOTAL_OPS: usize = NUM_CONCURRENT_TASKS * OPS_PER_TASK;

fn setup_redis_client() -> Client {
    Client::open(REDIS_URL).expect("Failed to create Redis client")
}

async fn setup_connection_manager() -> ConnectionManager {
    let client = setup_redis_client();
    ConnectionManager::new(client)
        .await
        .expect("Failed to create ConnectionManager")
}

async fn setup_deadpool_pool() -> Pool {
    let cfg = DeadpoolConfig::from_url(REDIS_URL);
    cfg.create_pool(Some(DeadpoolRuntime::Tokio1))
        .expect("Failed to create deadpool-redis pool")
}

fn generate_unique_key(prefix: &str, task_idx: usize, op_idx: usize) -> String {
    format!(
        "{}_{}_{}_{}",
        prefix,
        task_idx,
        op_idx,
        thread_rng().r#gen::<u64>()
    )
}

fn bench_connection_manager_non_blocking(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let conn_manager = rt.block_on(setup_connection_manager());

    let mut group = c.benchmark_group("ConnectionManager_Async_NonBlocking");
    group.throughput(Throughput::Elements(TOTAL_OPS as u64));

    group.bench_function("SET_GET_Concurrent", |b| {
        b.to_async(&rt).iter(|| async {
            let mut tasks = Vec::with_capacity(NUM_CONCURRENT_TASKS);
            for i in 0..NUM_CONCURRENT_TASKS {
                let mut conn = conn_manager.clone();
                let key_prefix = format!("cm_nb_key_{}", i);
                tasks.push(tokio::spawn(async move {
                    for j in 0..OPS_PER_TASK {
                        let key = generate_unique_key(&key_prefix, i, j);
                        let value = "test_value_long_enough_to_be_representative_of_data";
                        let _: () = black_box(conn.set(&key, value).await.unwrap());
                        let _: String = black_box(conn.get(&key).await.unwrap());
                    }
                }));
            }
            join_all(tasks).await;
        });
    });
    group.finish();
}

fn bench_deadpool_non_blocking(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let pool = rt.block_on(setup_deadpool_pool());

    let mut group = c.benchmark_group("Deadpool_Async_NonBlocking");
    group.throughput(Throughput::Elements(TOTAL_OPS as u64));

    group.bench_function("SET_GET_Concurrent", |b| {
        b.to_async(&rt).iter(|| async {
            let mut tasks = Vec::with_capacity(NUM_CONCURRENT_TASKS);
            for i in 0..NUM_CONCURRENT_TASKS {
                let pool_clone = pool.clone();
                let key_prefix = format!("dp_nb_key_{}", i);
                tasks.push(tokio::spawn(async move {
                    for j in 0..OPS_PER_TASK {
                        let mut conn = pool_clone.get().await.unwrap();
                        let key = generate_unique_key(&key_prefix, i, j);
                        let value = "test_value_long_enough_to_be_representative_of_data";

                        let _: () = black_box(conn.set(&key, value).await.unwrap());
                        let _: String = black_box(conn.get(&key).await.unwrap().unwrap());
                    }
                }));
            }
            join_all(tasks).await;
        });
    });
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(10));
    targets =
        bench_connection_manager_non_blocking,
        bench_deadpool_non_blocking
}
criterion_main!(benches);
