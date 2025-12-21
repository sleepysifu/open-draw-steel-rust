use rand::Rng;

/// Roll a pool of d3s and d10s and add a flat bonus.
/// Returns the total of all dice plus bonus.
pub fn roll(d3s: u32, d10s: u32, bonus: i32) -> i32 {
    let mut rng = rand::thread_rng();

    let d3_total: i32 = (0..d3s).map(|_| rng.gen_range(1..=3) as i32).sum();
    let d10_total: i32 = (0..d10s).map(|_| rng.gen_range(1..=10) as i32).sum();

    d3_total + d10_total + bonus
}