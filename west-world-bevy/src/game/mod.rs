pub mod messaging;
pub mod miner;
pub mod state;
pub mod wife;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Location {
    GoldMine,
    Bank,
    Shack,
    Saloon,
}
