use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerRollResultTier {
    Tier1, 
    Tier2, 
    Tier3,
    Tier3Critical
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
pub fn rolld3s(d3s: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..d3s).map(|_| rng.gen_range(1..=3) as i32).collect()
}

pub fn rolld10s(d10s: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..d10s).map(|_| rng.gen_range(1..=10) as i32).collect()
}

pub fn power_roll(bonus: i32, edge: EdgeState, bane: BaneState) -> PowerRollResultTier {
    let roll_result = rolld10s(2).iter().sum();
    to_power_roll(roll_result, bonus, edge, bane)
}

fn to_power_roll(natural_roll: i32, bonus:i32, edge: EdgeState, bane: BaneState) -> PowerRollResultTier {

    let did_crit = natural_roll == 19 || natural_roll == 20;
    if did_crit {
        return PowerRollResultTier::Tier3Critical;
    }

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
        1 => natural_roll + bonus+ 2,
        -1 => natural_roll + bonus - 2,
        _ => natural_roll + bonus,
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

fn to_tier(roll:i32) -> PowerRollResultTier {
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
        // to_tier tests the final result after bonus is applied
        for roll in -10..=11 {
            assert_eq!(to_tier(roll), PowerRollResultTier::Tier1);
        }
        for roll in 12..=16 {
            assert_eq!(to_tier(roll), PowerRollResultTier::Tier2);
        }
        for roll in 17..=30 {
            assert_eq!(to_tier(roll), PowerRollResultTier::Tier3);
        }
    }

    #[test]
    fn test_no_edge_no_bane() {
        // Natural roll 2d10 = 2-20, no modifiers
        let e = EdgeState::None;
        let b = BaneState::None;
        
        // Natural rolls 2-11: +0 bonus → 2-11 → Tier1
        for roll in 2..=11 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier1);
        }
        // Natural rolls 12-16: +0 bonus → 12-16 → Tier2
        for roll in 12..=16 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier2);
        }
        // Natural rolls 17-18: +0 bonus → 17-18 → Tier3
        for roll in 17..=18 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3);
        }
        // Natural rolls 19-20: Always Tier3Critical (early return)
        for roll in 19..=20 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3Critical);
        }
    }

    #[test]
    fn test_bonus() {
        // Test how bonus affects the final tier
        let e = EdgeState::None;
        let b = BaneState::None;
        
        // Natural roll 10, different bonuses
        // 10 + 0 = 10 → Tier1
        assert_eq!(to_power_roll(10, 0, e, b), PowerRollResultTier::Tier1);
        // 10 + 1 = 11 → Tier1
        assert_eq!(to_power_roll(10, 1, e, b), PowerRollResultTier::Tier1);
        // 10 + 2 = 12 → Tier2
        assert_eq!(to_power_roll(10, 2, e, b), PowerRollResultTier::Tier2);
        // 10 + 6 = 16 → Tier2
        assert_eq!(to_power_roll(10, 6, e, b), PowerRollResultTier::Tier2);
        // 10 + 7 = 17 → Tier3
        assert_eq!(to_power_roll(10, 7, e, b), PowerRollResultTier::Tier3);
        
        // Natural roll 15, different bonuses
        // 15 + 0 = 15 → Tier2
        assert_eq!(to_power_roll(15, 0, e, b), PowerRollResultTier::Tier2);
        // 15 + 1 = 16 → Tier2
        assert_eq!(to_power_roll(15, 1, e, b), PowerRollResultTier::Tier2);
        // 15 + 2 = 17 → Tier3
        assert_eq!(to_power_roll(15, 2, e, b), PowerRollResultTier::Tier3);
        
        // Natural roll 8, negative bonus
        // 8 + (-2) = 6 → Tier1
        assert_eq!(to_power_roll(8, -2, e, b), PowerRollResultTier::Tier1);
        // 8 + (-5) = 3 → Tier1
        assert_eq!(to_power_roll(8, -5, e, b), PowerRollResultTier::Tier1);
        
        // Critical rolls ignore bonus
        assert_eq!(to_power_roll(19, 5, e, b), PowerRollResultTier::Tier3Critical);
        assert_eq!(to_power_roll(20, -5, e, b), PowerRollResultTier::Tier3Critical);
    }

    #[test]
    fn test_single_edge() {
        // Natural roll 2d10 = 2-20, Single Edge (+2 to roll, no tier shift)
        let e = EdgeState::Single;
        let b = BaneState::None;
        // Natural 2-9: +2 = 4-11 → Tier1
        for roll in 2..=9 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier1);
        }
        // Natural 10-14: +2 = 12-16 → Tier2
        for roll in 10..=14 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier2);
        }   
        // Natural 15-18: +2 = 17-20 → Tier3
        for roll in 15..=18 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3);
        }
        // Natural 19-20: Always Tier3Critical
        for roll in 19..=20 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3Critical);
        }
    }

    #[test]
    fn test_double_edge() {
        // Natural roll 2d10 = 2-20, Double Edge (+2 tier shift)
        let e = EdgeState::Double;
        let b = BaneState::None;
        // Natural 2-11: Tier1 → Tier2 (tier shift)
        for roll in 2..=11 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier2);
        }
        // Natural 12-18: Tier2 → Tier3 (tier shift)
        for roll in 12..=18 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3);
        }
        // Natural 19-20: Always Tier3Critical
        for roll in 19..=20 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3Critical);
        }
    }

    #[test]
    fn test_single_bane() {
        // Natural roll 2d10 = 2-20, Single Bane (-2 to roll, no tier shift)
        let e = EdgeState::None;
        let b = BaneState::Single;
        // Natural 2-13: -2 = 0-11 → Tier1
        for roll in 2..=13 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier1);
        }
        // Natural 14-18: -2 = 12-16 → Tier2
        for roll in 14..=18 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier2);
        }
        // Natural 19-20: Always Tier3Critical
        for roll in 19..=20 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3Critical);
        }
    }

    #[test]
    fn test_double_bane() {
        // Natural roll 2d10 = 2-20, Double Bane (-2 tier shift)
        let e = EdgeState::None;
        let b = BaneState::Double;
        // Natural 2-16: Tier1 or Tier2 → Tier1 (tier shift down)
        for roll in 2..=16 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier1);
        }
        // Natural 17-18: Tier3 → Tier2 (tier shift down)
        for roll in 17..=18 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier2);
        }
        // Natural 19-20: Always Tier3Critical
        for roll in 19..=20 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3Critical);
        }
    }

    #[test]
    fn test_single_edge_single_bane() {
        // Natural roll 2d10 = 2-20, Single Edge + Single Bane (net 0, no modifiers)
        let e = EdgeState::Single;
        let b = BaneState::Single;
        // Natural 2-11: +0 = 2-11 → Tier1
        for roll in 2..=11 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier1);
        }
        // Natural 12-16: +0 = 12-16 → Tier2
        for roll in 12..=16 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier2);
        }
        // Natural 17-18: +0 = 17-18 → Tier3
        for roll in 17..=18 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3);
        }
        // Natural 19-20: Always Tier3Critical
        for roll in 19..=20 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3Critical);
        }
    }

    #[test]
    fn test_single_edge_double_bane() {
        // Natural roll 2d10 = 2-20, Single Edge + Double Bane (net -1, -2 to roll)
        let e = EdgeState::Single;
        let b = BaneState::Double;
        // Natural 2-13: -2 = 0-11 → Tier1
        for roll in 2..=13 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier1);
        }
        // Natural 14-18: -2 = 12-16 → Tier2
        for roll in 14..=18 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier2);
        }
        // Natural 19-20: Always Tier3Critical
        for roll in 19..=20 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3Critical);
        }
    }

    #[test]
    fn test_double_edge_single_bane() {
        // Natural roll 2d10 = 2-20, Double Edge + Single Bane (net +1, +2 to roll)
        let e = EdgeState::Double;
        let b = BaneState::Single;
        // Natural 2-9: +2 = 4-11 → Tier1
        for roll in 2..=9 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier1);
        }
        // Natural 10-14: +2 = 12-16 → Tier2
        for roll in 10..=14 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier2);
        }
        // Natural 15-18: +2 = 17-20 → Tier3
        for roll in 15..=18 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3);
        }
        // Natural 19-20: Always Tier3Critical
        for roll in 19..=20 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3Critical);
        }
    }

    #[test]
    fn test_double_edge_double_bane() {
        // Natural roll 2d10 = 2-20, Double Edge + Double Bane (net 0, no tier shift)
        let e = EdgeState::Double;
        let b = BaneState::Double;
        // Natural 2-11: +0 = 2-11 → Tier1
        for roll in 2..=11 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier1);
        }
        // Natural 12-16: +0 = 12-16 → Tier2
        for roll in 12..=16 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier2);
        }
        // Natural 17-18: +0 = 17-18 → Tier3
        for roll in 17..=18 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3);
        }
        // Natural 19-20: Always Tier3Critical
        for roll in 19..=20 {
            assert_eq!(to_power_roll(roll, 0, e, b), PowerRollResultTier::Tier3Critical);
        }
    }
}