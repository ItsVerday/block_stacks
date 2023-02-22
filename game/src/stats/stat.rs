use std::vec;

type StatValue = f64;

pub struct Stat {
    pub base_value: StatValue,
    pub final_value: Option<StatValue>,
    pub base: StatLayer,
    pub early: StatLayer,
    pub middle: StatLayer,
    pub late: StatLayer,
    pub total: StatLayer
}

impl Stat {
    pub fn new(base_value: StatValue) -> Stat {
        Stat {
            base_value,
            final_value: None,
            base: StatLayer::new(),
            early: StatLayer::new(),
            middle: StatLayer::new(),
            late: StatLayer::new(),
            total: StatLayer::new()
        }
    }

    pub fn clear(&mut self) {
        self.base.clear();
        self.early.clear();
        self.middle.clear();
        self.late.clear();
        self.total.clear();
        self.final_value = None;
    }

    pub fn get_value(&mut self) -> StatValue {
        if let Some(final_value) = self.final_value {
            return final_value;
        }

        let mut current_value = self.base_value;
        current_value = self.base.apply(current_value);
        current_value = self.early.apply(current_value);
        current_value = self.middle.apply(current_value);
        current_value = self.late.apply(current_value);
        current_value = self.total.apply(current_value);

        self.final_value = Some(current_value);
        current_value
    }
}

pub struct StatLayer {
    pub multipliers: Vec<Box<dyn StatModifier>>,
    pub adders: Vec<Box<dyn StatModifier>>,
    pub min: Option<Box<dyn StatModifier>>,
    pub max: Option<Box<dyn StatModifier>>
}

impl StatLayer {
    pub fn new() -> StatLayer {
        StatLayer {
            multipliers: vec![],
            adders: vec![],
            min: None,
            max: None
        }
    }

    pub fn clear(&mut self) {
        self.multipliers = vec![];
        self.adders = vec![];
        self.min = None;
        self.max = None;
    }

    pub fn multiply(&mut self, value: StatValue) {
        self.multipliers.push(Box::new(MultiplicativeStatModifier {
            coefficient: value
        }));
    }

    pub fn add(&mut self, value: StatValue) {
        self.adders.push(Box::new(AdditiveStatModifier {
            increment: value
        }));
    }

    pub fn hard_min(&mut self, value: StatValue) {
        self.min = Some(Box::new(HardMinimumStatModifier {
            min: value
        }));
    }

    pub fn clear_min(&mut self) {
        self.min = None;
    }

    pub fn hard_max(&mut self, value: StatValue) {
        self.max = Some(Box::new(HardMaximumStatModifier {
            max: value
        }));
    }

    pub fn clear_max(&mut self) {
        self.max = None;
    }

    pub fn apply(&self, value: StatValue) -> StatValue {
        let mut current_value = value;
        for multiplier in self.multipliers.iter() {
            current_value = multiplier.modify_stat(current_value);
        }

        for adder in self.adders.iter() {
            current_value = adder.modify_stat(current_value);
        }

        if let Some(min) = &self.min {
            current_value = min.modify_stat(current_value);
        }

        if let Some(max) = &self.max {
            current_value = max.modify_stat(current_value);
        }

        current_value
    }
}

pub trait StatModifier {
    fn modify_stat(&self, stat: StatValue) -> StatValue;
}

pub struct AdditiveStatModifier {
    pub increment: StatValue
}

impl StatModifier for AdditiveStatModifier {
    fn modify_stat(&self, stat: StatValue) -> StatValue {
        stat + self.increment
    }
}

pub struct MultiplicativeStatModifier {
    pub coefficient: StatValue
}

impl StatModifier for MultiplicativeStatModifier {
    fn modify_stat(&self, stat: StatValue) -> StatValue {
        stat * self.coefficient
    }
}

pub struct HardMinimumStatModifier {
    pub min: StatValue
}

impl StatModifier for HardMinimumStatModifier {
    fn modify_stat(&self, stat: StatValue) -> StatValue {
        if stat < self.min {
            self.min
        } else {
            stat
        }
    }
}

pub struct HardMaximumStatModifier {
    pub max: StatValue
}

impl StatModifier for HardMaximumStatModifier {
    fn modify_stat(&self, stat: StatValue) -> StatValue {
        if stat > self.max {
            self.max
        } else {
            stat
        }
    }
}