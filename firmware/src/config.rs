#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use spin::Once;

/* ===========================
 * Encoder
 * =========================== */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncoderConfig {
    pub pin_a: u8,
    pub pin_b: u8,
    pub reduction_factor: u16,
    pub pulses_per_rev: u16,
}

/* ===========================
 * Driver
 * =========================== */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DriverConfig {
    pub pin_a: u8,
    pub pin_b: u8,
    pub pwm_wrap: u16,
}

/* ===========================
 * PID Polinomial
 * =========================== */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolynomialGain {
    /// [a0, a1, a2, a3]
    pub coeffs: [f32; 4],
}

impl PolynomialGain {
    pub fn eval(&self, x: f32) -> f32 {
        // Horner
        (((self.coeffs[3] * x
            + self.coeffs[2]) * x
            + self.coeffs[1]) * x
            + self.coeffs[0])
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PidConfig {
    pub kp: PolynomialGain,
    pub ki: PolynomialGain,
    pub kd: PolynomialGain,

    pub period_ms: u16,

    pub out_min: i16,
    pub out_max: i16,
}

/* ===========================
 * Motor
 * =========================== */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MotorConfig {
    pub encoder: EncoderConfig,
    pub driver: DriverConfig,
    pub pid: PidConfig,
}

/* ===========================
 * IMU
 * =========================== */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImuConfig {
    pub miso: u8,
    pub mosi: u8,
    pub cs: u8,
    pub sck: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GpsConfig {
    pub tx: u8,
    pub rx: u8,
    pub baud_rate: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigJson {
    pub motors: BTreeMap<u8, MotorConfig>,
    pub imu: ImuConfig,
    pub gps: GpsConfig,
}

pub type Config = ConfigJson;

impl Default for Config {
    fn default() -> Self {
        let mut motors = BTreeMap::new();

        motors.insert(
            1,
            MotorConfig {
                encoder: EncoderConfig {
                    pin_a: 18,
                    pin_b: 19,
                    reduction_factor: 496,
                    pulses_per_rev: 11,
                },

                driver: DriverConfig {
                    pin_a: 20,
                    pin_b: 21,
                    pwm_wrap: 1000,
                },

                pid: PidConfig {
                    kp: PolynomialGain {
                        coeffs: [131.6, 0.0, 0.0, 0.0],
                    },

                    ki: PolynomialGain {
                        coeffs: [10.01, 0.0, 0.0, 0.0],
                    },

                    kd: PolynomialGain {
                        coeffs: [0.01, 0.0, 0.0, 0.0],
                    },

                    period_ms: 10,

                    out_min: -100,
                    out_max: 100,
                },
            },
        );

        /*motors.insert(
            2,
            MotorConfig {
                encoder: EncoderConfig {
                    pin_a: 16,
                    pin_b: 17,
                    reduction_factor: 496,
                    pulses_per_rev: 11,
                },

                driver: DriverConfig {
                    pin_a: 22,
                    pin_b: 23,
                    pwm_wrap: 1000,
                },

                pid: PidConfig {
                    kp: PolynomialGain {
                        coeffs: [131.6, 0.0, 0.0, 0.0],
                    },

                    ki: PolynomialGain {
                        coeffs: [10.01, 0.0, 0.0, 0.0],
                    },

                    kd: PolynomialGain {
                        coeffs: [0.01, 0.0, 0.0, 0.0],
                    },

                    period_ms: 10,

                    out_min: -100,
                    out_max: 100,
                },
            },
        );*/

        Self {
            motors,

            imu: ImuConfig {
                miso: 4,
                mosi: 7,
                cs: 5,
                sck: 6,
            },

            gps: GpsConfig {
                tx: 11,
                rx: 12,
                baud_rate: 115200,
            },
        }
    }
}

pub static GLOBAL_CONFIG: Once<Config> = Once::new();

pub fn init_global_config() {
    GLOBAL_CONFIG.call_once(Config::default);
}

pub fn get_config() -> Option<&'static Config> {
    GLOBAL_CONFIG.poll()
}