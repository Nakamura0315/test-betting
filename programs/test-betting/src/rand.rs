/// generate random value for loss_chance
pub fn generate(timestamp: u32, amount: u64) -> u8 {
    ((timestamp as u64).wrapping_add(amount) % 2) as u8
}
