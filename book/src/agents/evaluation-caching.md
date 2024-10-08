# Evaluation Caching

It's not unusual for your agent to have an evaluation function which is expensive to run. To mitigate this, we recommend adding a `Arc<Mutex<f64>` to store a cached value for each of your agents. For example:

```rs
struct AgentWithCache {
    genome: Vec<u8>,
    evaluation: Arc<Mutex<Option<f64>>
}
```

The wrapping `Arc<Mutex<>` is necessary to ensure that the value is both `Send + Sync` in order to work with the multithreaded simulator. If using a custom simulator, you could just use a `RefCell` which also provides interior mutability
