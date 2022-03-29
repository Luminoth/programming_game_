mod entity;
mod location;
mod messaging;
mod miner;
mod state;
mod wife;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use messaging::MessageDispatcher;
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

    let dispatcher = Rc::new(RefCell::new(MessageDispatcher::default()));

    let bob = Rc::new(RefCell::new(Miner::new("Miner Bob", dispatcher.clone())));
    dispatcher
        .borrow_mut()
        .register_message_receiver(bob.borrow().entity(), bob.clone());

    let elsa = Rc::new(RefCell::new(Wife::new("Elsa", dispatcher.clone())));
    dispatcher
        .borrow_mut()
        .register_message_receiver(elsa.borrow().entity(), elsa.clone());

    bob.borrow_mut().set_wife_id(elsa.borrow().entity().id());
    elsa.borrow_mut().set_miner_id(bob.borrow().entity().id());

    loop {
        bob.borrow_mut().update();
        elsa.borrow_mut().update();

        // TODO: this panics due to multiple mutable borrows
        dispatcher.borrow_mut().dispatch_deferred_messages();

        thread::sleep(Duration::from_millis(800));
    }
}
