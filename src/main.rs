mod workouts;

use clap::{command, ArgAction, Parser};

use crate::workouts::bike::start_bike_workout;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Want to generate a run
    #[arg(short, long, action)]
    run: bool,

    // Want to generate a bike
    #[arg(short, long, action)]
    bike: bool,

    // Want to generate a swim
    #[arg(short, long, action)]
    swim: bool,
}

fn main() {
    let args = Args::parse();

    let bike = args.bike;
    let run = args.run;
    let swim = args.swim;

    if bike == true {
        start_bike_workout();
    } else if run == true {
        todo!()
    } else if swim == true {
        todo!()
    } else {
        todo!()
    }

    println!("args {:?}", args);
}
