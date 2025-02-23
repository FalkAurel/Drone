use core::{f32::consts::PI, ops::Mul};
use libm::{atan2f, sqrtf};

use crate::gy521::{AccelometerData, GyroscopeData};

const ITERATION_LENGTH: f32 = 0.004; // In seconds
pub struct PID {
}

#[derive(Debug)]
pub struct Angle {
    x: f32,
    y: f32,
    z: f32
}

impl Default for Angle {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Mul<f32> for Angle {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}


// Computes the angle of the sensor based on the gyro sensor and °/s.
//
// Angle Pith = ∫_0_i*t Rate_pitch * dt
// 
// Basierend auf der Stammfunktion F, kann man iterativ den nächsten Wert bestimmen, da °/s die Veränderung beschreibt.
//
// Tₛ = Iterationslänge, die Vergangenezeit zwischen den einzelnen Lesungen
//
// F(t + 1) = F(t) + f(t) * Tₛ 
pub fn compute_angle_integration(gyro: &GyroscopeData, prev_angle: Angle) -> Angle {
    let GyroscopeData { x, y, z } = *gyro;
    let Angle { x: prev_x, y: prev_y, z: prev_z } = prev_angle;

    Angle { 
        x: prev_x + x * ITERATION_LENGTH, 
        y: prev_y + y * ITERATION_LENGTH, 
        z: prev_z + z * ITERATION_LENGTH
     }
}


// https://www.researchgate.net/figure/Drones-pitch-roll-and-yaw_fig2_329521700
pub fn compute_angle_acceleration(accel: &AccelometerData) -> Angle {
    let AccelometerData { x, y, z } = *accel;

    let roll: f32 = 180.0 * atan2f(y, sqrtf(x * x + z * z)) / PI;
    let pitch: f32 = 180.0 * atan2f(x, sqrtf(y * y + z * z)) / PI;

    Angle { x: roll, y: pitch, z: 0.0 } 
}