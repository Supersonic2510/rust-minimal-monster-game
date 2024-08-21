use rand::prelude::*;
use rand_distr::{Binomial, Distribution};

#[derive(Debug)]
pub struct DamageDistribution {
    pub damage_low: i8,
    pub damage_center: i8,
    pub damage_high: i8,
    pub crit_low: i8,
    pub crit_center: i8,
    pub crit_high: i8,
    pub crit_rate: f32,
}

impl DamageDistribution {
    pub fn new(
        damage_low: i8,
        damage_center: i8,
        damage_high: i8,
        crit_low: i8,
        crit_center: i8,
        crit_high: i8,
        crit_rate: f32,
    ) -> Result<Self, String> {
        if !(damage_low <= damage_center && damage_center <= damage_high) {
            return Err(String::from(
                "Invalid damage values: ensure damage_low <= damage_center <= damage_high",
            ));
        }
        if !(damage_high <= crit_low) {
            return Err(String::from(
                "Invalid values: ensure damage_high <= crit_low",
            ));
        }
        if !(crit_low <= crit_center && crit_center <= crit_high) {
            return Err(String::from(
                "Invalid crit values: ensure crit_low <= crit_center <= crit_high",
            ));
        }
        if crit_rate < 0.0 || crit_rate > 1.0 {
            return Err(String::from(
                "Invalid crit_rate: ensure it is between 0.0 and 1.0",
            ));
        }

        Ok(DamageDistribution {
            damage_low,
            damage_center,
            damage_high,
            crit_low,
            crit_center,
            crit_high,
            crit_rate,
        })
    }

    pub fn new_no_crit(damage_low: i8, damage_center: i8, damage_high: i8) -> Result<Self, String> {
        return Self::new(
            damage_low,
            damage_center,
            damage_high,
            damage_high,
            damage_high,
            damage_high,
            0f32,
        );
    }

    pub fn get_damage_value(&self) -> i8 {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < self.crit_rate as f64 {
            // Generate a value from the critical distribution
            let n_crit = (self.crit_high - self.crit_low + 1) as u64;
            let p_crit = (self.crit_center - self.crit_low) as f64 / n_crit as f64;
            let binomial = Binomial::new(n_crit, p_crit).unwrap();
            let value = binomial.sample(&mut rng) as i8 + self.crit_low;
            value.clamp(self.crit_low, self.crit_high)
        } else {
            // Generate a value from the damage distribution
            let n_damage = (self.damage_high - self.damage_low + 1) as u64;
            let p_damage = (self.damage_center - self.damage_low) as f64 / n_damage as f64;
            let binomial = Binomial::new(n_damage, p_damage).unwrap();
            let value = binomial.sample(&mut rng) as i8 + self.damage_low;
            value.clamp(self.damage_low, self.damage_high)
        }
    }
}
