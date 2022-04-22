pub mod miner;
pub mod wife;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Location {
    GoldMine,
    Bank,
    Shack,
    Saloon,
}
