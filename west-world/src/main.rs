mod entity;
mod location;
mod miner;
mod wife;

use std::thread;
use std::time::Duration;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use miner::Miner;
use wife::Wife;

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_logging()?;

    let mut bob = Miner::new("Miner Bob");
    let mut elsa = Wife::new("Elsa");
    loop {
        bob.update();
        elsa.update();

        thread::sleep(Duration::from_millis(800));
    }
}
