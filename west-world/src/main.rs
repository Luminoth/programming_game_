mod entity;
mod location;
mod miner;

use miner::Miner;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_logging()?;

    let mut miner = Miner::new("Miner Bob");
    loop {
        miner.update();
    }
}
