use enostr::RelayPool;
use enostr::Result;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut pool = RelayPool::new();

    // Example: Add a multicast relay
    pool.add_multicast_relay(|| {})?;

    // Loop to receive events (this is a basic example, a real relay would be more complex)
    loop {
        if let Some(event) = pool.try_recv() {
            println!("Received event from {}: {:?}", event.relay, event.event);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
