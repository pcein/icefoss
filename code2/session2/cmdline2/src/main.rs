extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long = "led1", help = "turn led1 ON/OFF")]
    led1: Option<String>,

    #[structopt(long = "led2", help = "turn led2 ON/OFF")]
    led2: Option<String>,

    #[structopt(long="readinput", help = "read status of input pin")]
    readinput: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
