use structopt::StructOpt;
use std::str::FromStr;
use std::fmt::{Display, Formatter};

#[derive(Debug, StructOpt)]
#[structopt(name = "Tasks", about = "An example of StructOpt usage.")]
enum Opt {
    Create {
        #[structopt(short, long)]
        is_open: bool,
        #[structopt(short = "p", long = "priority", default_value = "5")]
        priority: i32,
        #[structopt(short = "s", long = "status")]
        status: Option<Status>,
    },
    Edit {
        #[structopt(short, long)]
        is_open: bool,
        #[structopt(short = "p", long = "priority", default_value = "5")]
        priority: i32,
        #[structopt(short = "s", long = "status")]
        status: Option<Status>,
    },
}

#[derive(Debug)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl FromStr for Status {
    type Err = MyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ToDo" => Ok(Status::ToDo),
            "InProgress" => Ok(Status::InProgress),
            "Done" => Ok(Status::Done),
            _ => Err(MyError(String::from("Costomized Error")))
        }
    }
}

#[derive(Debug)]
struct MyError(String);

impl std::error::Error for MyError { }

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Status must be one of the following: ToDo, InProgress or Done.")
    }
}

fn main() {
    let opt = Opt::from_args();
    for _ in 0..3 {
        println!("opt = {:?}", opt);
    }
    let task = opt;
    match task {
        Opt::Create { is_open, priority, status } => println!("Created task with is_open = {}, priority = {}, status = {:?}", is_open, priority, status),
        Opt::Edit { is_open, priority, status } => println!("Updated task with is_open = {}, priority = {}, status = {:?}", is_open, priority, status)
    }
}
