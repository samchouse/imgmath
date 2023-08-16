use clap::{Parser, Subcommand};

use imgmath::shaders::{blue_light_filter::BlueLightFilter, Shader};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Path to input file
    #[arg(short, long)]
    input: String,
    /// Path to output file
    #[arg(short, long)]
    output: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Reverse the effect of a shader
    Reverse {
        #[command(subcommand)]
        command: ReverseCommand,
    },
}

#[derive(Subcommand)]
enum ReverseCommand {
    /// Reverse the effect of a blue light filter
    #[command(alias = "blf")]
    BlueLightFilter {
        /// Color temperature
        #[arg(default_value_t = 4000.0)]
        temperature: f64,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Reverse { command } => match command {
            ReverseCommand::BlueLightFilter { temperature } => {
                BlueLightFilter::new(temperature).reverse_on_file(&args.input, &args.output);
            }
        },
    }
}
