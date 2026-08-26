#![allow(unused)]

// config.rs - Configuration (alternative to Singleton)

use crate::token::NumberFormat;
use std::sync::OnceLock;

#[derive(Default, Debug, Clone)]
pub enum AngleMode {
    Degrees,
    #[default]
    Radians,
}

#[derive(Debug, Clone)]
pub struct CalculatorConfig {
    pub precision: u32,
    pub angle_mode: AngleMode,
    pub notation: NumberFormat,
}

impl Default for CalculatorConfig {
    fn default() -> Self {
        Self {
            precision: 10,
            angle_mode: Default::default(),
            notation: Default::default(),
        }
    }
}

impl CalculatorConfig {
    pub fn standard() -> Self {
        Default::default()
    }

    // Factory methods for common configurations
    pub fn scientific() -> Self {
        Self {
            precision: 15,
            notation: NumberFormat::Scientific,
            ..Default::default()
        }
    }

    pub fn engineering() -> Self {
        Self {
            notation: NumberFormat::Engineering,
            ..Default::default()
        }
    }
}

// Constants
pub const DEFAULT_PRECISION: u32 = 10;
pub const MAX_PRECISION: u32 = 100;

// If we need a global configuration (alternative to Singleton)
static CONFIG: OnceLock<CalculatorConfig> = OnceLock::new();

pub fn get_global_config() -> &'static CalculatorConfig {
    CONFIG.get_or_init(|| {
        // In a real application, this might load from a file or environment
        Default::default()
    })
}

// Thread-safe calculator with shared config
use std::sync::Arc;

#[derive(Default)]
pub struct CalculatorPool {
    shared_config: Arc<CalculatorConfig>,
    // In a real application, this would store calculator instances
    _calculators: Vec<()>,
}

impl CalculatorPool {
    pub fn new(config: CalculatorConfig) -> Self {
        Self {
            shared_config: Arc::new(config),
            _calculators: Default::default(),
        }
    }

    pub fn get_config(&self) -> Arc<CalculatorConfig> {
        Arc::clone(&self.shared_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let c = CalculatorConfig::default();
        assert_eq!(c.precision, 10);
        assert!(matches!(c.angle_mode, AngleMode::Radians));
        assert!(matches!(c.notation, NumberFormat::Decimal));
    }

    #[test]
    fn test_scientific_config() {
        let c = CalculatorConfig::scientific();
        assert_eq!(c.precision, 15);
        assert!(matches!(c.angle_mode, AngleMode::Radians));
        assert!(matches!(c.notation, NumberFormat::Scientific));
    }

    #[test]
    fn test_engineering_config() {
        let c = CalculatorConfig::engineering();
        assert_eq!(c.precision, 10);
        assert!(matches!(c.angle_mode, AngleMode::Radians));
        assert!(matches!(c.notation, NumberFormat::Engineering));
    }

    #[test]
    fn test_global_config() {
        let cfg = get_global_config();
        assert_eq!(cfg.precision, 10);
        assert!(matches!(cfg.angle_mode, AngleMode::Radians));
        assert!(matches!(cfg.notation, NumberFormat::Decimal));
    }

    #[test]
    fn test_pool_get_config() {
        let pool = CalculatorPool::new(CalculatorConfig::default());
        let cfg = pool.get_config();
        assert_eq!(cfg.precision, 10);
        assert!(matches!(cfg.angle_mode, AngleMode::Radians));
        assert!(matches!(cfg.notation, NumberFormat::Decimal));
    }
}
