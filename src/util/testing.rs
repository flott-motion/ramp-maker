//! Reusable testing code for motion profiles
//!
//! This module contains tests that need to hold for all [`MotionProfile`]
//! implementations. It is called from the test suites of each implementation in
//! this crate, and may be reused by other implementations from outside this
//! crate.
//!
//! Each test is added as a function here, but all of them are called from the
//! [`test`] function.

#![cfg(test)]

/// Alias for [`crate::MotionProfile`] with some extras, used by the tests here
pub trait MotionProfile:
    crate::MotionProfile<Velocity = f32, Delay = f32> + Default
{
}

impl<T> MotionProfile for T where
    T: crate::MotionProfile<Velocity = f32, Delay = f32> + Default
{
}

/// Run full test suite on the provided [`MotionProfile`] implementation
pub fn test<Profile>()
where
    Profile: MotionProfile,
{
    position_mode_must_produce_correct_number_of_steps(Profile::default());
    position_mode_must_respect_maximum_velocity(Profile::default());
    position_mode_must_not_panic_because_of_zero_velocity(Profile::default());
    position_mode_must_not_panic_because_of_zero_steps(Profile::default());
}

/// A motion in position mode must produce the correct number of steps
pub fn position_mode_must_produce_correct_number_of_steps(
    mut profile: impl MotionProfile,
) {
    let num_steps = 200;
    profile.enter_position_mode(1000.0, num_steps);

    assert_eq!(profile.delays().count() as u32, num_steps);
}

/// A motion in position mode must respect the maximum velocity
pub fn position_mode_must_respect_maximum_velocity(
    mut profile: impl MotionProfile,
) {
    let max_velocity = 1000.0;
    profile.enter_position_mode(max_velocity, 200);

    for velocity in profile.velocities() {
        println!("velocity: {}, max velocity: {}", velocity, max_velocity);
        assert!(velocity <= max_velocity);
    }
}

/// Entering position mode with a max velocity of zero must not cause a panic
pub fn position_mode_must_not_panic_because_of_zero_velocity(
    mut profile: impl MotionProfile,
) {
    profile.enter_position_mode(0.0, 200);
    assert_eq!(profile.next_delay(), None);
}

/// Entering position mode with a target of zero steps must not cause a panic
pub fn position_mode_must_not_panic_because_of_zero_steps(
    mut profile: impl MotionProfile,
) {
    profile.enter_position_mode(1000.0, 0);
    assert_eq!(profile.next_delay(), None);
}
