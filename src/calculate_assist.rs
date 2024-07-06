use rastro::{
    constants::{GM_EARTH, R_EARTH},
    orbits::orbital_period_general,
};
use std::f64::{self, consts::PI};

use crate::vector::Vector;

const GM: f64 = GM_EARTH;

/// Given an enter velocity and the planet's velocity, returns an exit velocity
/// for a body (a probe) orbiting said planet, in the star's reference frame.
fn velocity(enter_velocity: Vector, planet_velocity: Vector) -> Vector {
    let v = enter_velocity.magnitude;
    let u = planet_velocity.magnitude;

    let final_speed = (v + 2_f64 * u) // (v+2u)
        * (
            1_f64 - (4_f64 * u * v * (1_f64 - (enter_velocity.angle).cos())) // 4uv(1-cos(t))
                    / (v + 2_f64 * u).powi(2) // (v+2u)^2
        ).sqrt();
    Vector::new(enter_velocity.angle, final_speed)
}

/// Returns the orbital velocity of an astronomical object at a
/// certain point in its orbit around a celestial body.
fn orbital_speed(gm: f64, distance_to_center_of_mass: f64, semi_major_axis_length: f64) -> f64 {
    let r = distance_to_center_of_mass;
    let a = semi_major_axis_length;
    (gm * (2_f64 / r - 1_f64 / a)).sqrt()
}

pub fn calculate_assist() {
    let enter_velocity = Vector::new(PI, 100_f64);
    let planet_velocity = enter_velocity;

    let semi_major_axis = R_EARTH + 100e3;
    let semi_minor_axis = semi_major_axis;

    let exit_velocity = velocity(enter_velocity, planet_velocity);
    dbg!(exit_velocity);

    let speed = orbital_speed(GM, R_EARTH + 100e3, R_EARTH + 100e3);
    dbg!(speed);

    let orbital_period = orbital_period_general(semi_major_axis, GM);
    dbg!(orbital_period);
}
