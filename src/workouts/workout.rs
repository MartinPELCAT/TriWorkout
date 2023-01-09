use std::io::Error;

pub trait Workout {
    fn new() -> Self;

    fn start_planning(&self) -> Result<(), Error>;

    fn ask_for_notes(&self) -> String {
        todo!()
    }

    fn ask_for_intensity(&self) -> WorkoutIntensity {
        todo!()
    }
}

pub struct WorkoutSteps {}

pub enum WorkoutIntensity {
    Hard,
    Treshold,
    Endurance,
    ActiveRecovery,
}
