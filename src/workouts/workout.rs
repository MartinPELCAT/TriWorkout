use std::io::{stdin, Error};

pub trait Workout {
    fn new() -> Self;

    fn start_planning(&self) -> Result<(), Error>;

    fn ask_for_notes(&self) -> String {
        let mut notes = String::new();
        println!("Voulez vous ajouter une note ? [default = '']");
        stdin().read_line(&mut notes).unwrap();
        return notes.replace("\n", "");
    }

    fn ask_for_intensity(&self) -> WorkoutIntensity {
        let mut intensity_input = String::new();
        println!("Select intensisty ? [Threshold=t, Endurance=e, Recovery=r]");
        stdin().read_line(&mut intensity_input).unwrap();
        let intensity_input = intensity_input.replace("\n", "");
        let intensity: WorkoutIntensity = match intensity_input.as_str() {
            "t" => WorkoutIntensity::Treshold,
            "e" => WorkoutIntensity::Endurance,
            "r" => WorkoutIntensity::ActiveRecovery,
            _ => {
                println!("Intensity does not exist please select a valid one");
                self.ask_for_intensity()
            }
        };

        return intensity;
    }
}

pub struct WorkoutSteps {}

#[derive(Debug)]
pub enum WorkoutIntensity {
    Treshold,
    Endurance,
    ActiveRecovery,
}
