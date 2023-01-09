use std::io::Error;

use super::workout::{Workout, WorkoutSteps};

pub struct BikeWorkout {
    steps: Option<WorkoutSteps>,
}

impl Workout for BikeWorkout {
    fn new() -> Self {
        BikeWorkout { steps: None }
    }

    fn start_planning(&self) -> Result<(), Error> {
        println!("Welcome to the bike workout planner");

        Ok(())
    }
}

pub fn start_bike_workout() -> BikeWorkout {
    let bike_workout = BikeWorkout::new();
    bike_workout.start_planning().unwrap();

    return bike_workout;
}
