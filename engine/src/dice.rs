use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerRollResultTier {
    Tier1, 
    Tier2, 
    Tier3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeState {
    None,
    Single,
    Double
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaneState {
    None,
    Single,
    Double
}

/// Roll a pool of d3s and d10s and add a flat bonus.
/// Returns the total of all dice plus bonus.
pub fn roll(d3s: u32, d10s: u32, bonus: i32) -> i32 {
    let mut rng = rand::thread_rng();

    let d3_total: i32 = (0..d3s).map(|_| rng.gen_range(1..=3) as i32).sum();
    let d10_total: i32 = (0..d10s).map(|_| rng.gen_range(1..=10) as i32).sum();

    d3_total + d10_total + bonus
}

pub fn power_roll(bonus: i32, edge: EdgeState, bane: BaneState) -> PowerRollResultTier {
    let roll_result = roll(0, 2, bonus);
    to_power_roll(roll_result, edge, bane)
}

pub fn to_power_roll(roll_result: i32, edge: EdgeState, bane: BaneState) -> PowerRollResultTier {

    let mut total_edge = 0;    
    match edge {
        EdgeState::Single => total_edge = total_edge + 1,
        EdgeState::Double => total_edge = total_edge + 2,
        _ => (),
    }
    match bane {
        BaneState::Single => total_edge = total_edge - 1,
        BaneState::Double => total_edge = total_edge - 2,
        _ => (),
    }

    let final_roll_result = match total_edge {
        1 => roll_result + 2,
        -1 => roll_result - 2,
        _ => roll_result,
    };

    let tier = to_tier(final_roll_result);

    let final_tier = match (total_edge, tier) {
        (2, PowerRollResultTier::Tier1) => PowerRollResultTier::Tier2,
        (2, PowerRollResultTier::Tier2) => PowerRollResultTier::Tier3,
        (-2, PowerRollResultTier::Tier2) => PowerRollResultTier::Tier1,
        (-2, PowerRollResultTier::Tier3) => PowerRollResultTier::Tier2,
        _ => tier,
    };

    final_tier
}

pub fn to_tier(roll:i32) -> PowerRollResultTier {
    match roll {
        ..=11 => PowerRollResultTier::Tier1,
        12..=16 => PowerRollResultTier::Tier2,
        17.. => PowerRollResultTier::Tier3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_tier() {
        for roll in -1..=11 {
            assert_eq!(to_tier(roll), PowerRollResultTier::Tier1);
        }
        for roll in 12..=16 {
            assert_eq!(to_tier(roll), PowerRollResultTier::Tier2);
        }
        for roll in 17..=21 {
            assert_eq!(to_tier(roll), PowerRollResultTier::Tier3);
        }
    }

    #[test]
    fn test_single_edge() {
        let e = EdgeState::Single;
        let b = BaneState::None;
        for roll in -1..=9 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier1);
        }
        for roll in 10..=14 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier2);
        }
        for roll in 15..=21 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier3);
        }
    }

    #[test]
    fn test_double_edge() {
        let e = EdgeState::Double;
        let b = BaneState::None;
        for roll in -1..=11 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier2);
        }
        for roll in 12..=21 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier3);
        }
    }

    #[test]
    fn test_single_bane() {
        let e = EdgeState::None;
        let b = BaneState::Single;
        for roll in -1..=13 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier1);
        }
        for roll in 14..=18 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier2);
        }
        for roll in 19..=21 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier3);
        }
    }

    #[test]
    fn test_double_bane() {
        let e = EdgeState::None;
        let b = BaneState::Double;
        for roll in -1..=16 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier1);
        }
        
        for roll in 17..=21 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier2);
        }
    }

    #[test]
    fn test_single_edge_single_bane() {
        let e = EdgeState::Single;
        let b = BaneState::Single;
        for roll in -1..=11{
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier1);
        }
        for roll in 12..=16 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier2);
        }
        for roll in 17..=21 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier3);
        }
    }

    #[test]
    fn test_single_edge_double_bane() {
        let e = EdgeState::Single;
        let b = BaneState::Double;
        for roll in -1..=13 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier1);
        }
        for roll in 14..=18 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier2);
        }
        for roll in 19..=21 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier3);
        }
    }

    #[test]
    fn test_double_edge_single_bane() {
        let e = EdgeState::Double;
        let b = BaneState::Single;
        for roll in -1..=9 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier1);
        }
        for roll in 10..=14 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier2);
        }
        for roll in 15..=21 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier3);
        }
    }

    #[test]
    fn test_double_edge_double_bane() {
        let e = EdgeState::Double;
        let b = BaneState::Double;
        for roll in -1..=11{
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier1);
        }
        for roll in 12..=16 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier2);
        }
        for roll in 17..=21 {
            assert_eq!(to_power_roll(roll, e, b), PowerRollResultTier::Tier3);
        }
    }
}