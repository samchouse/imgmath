use clap::{Parser, Subcommand};
use image;
use shaders::{blue_light_filter::BlueLightFilter, Shader};

mod shaders;

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

    let mut img = image::open(args.input).unwrap().to_rgba8();
    match args.command {
        Command::Reverse { command } => match command {
            ReverseCommand::BlueLightFilter { temperature } => {
                let shader = BlueLightFilter::new(temperature);
                img.pixels_mut().for_each(|pixel| {
                    let reversed_pixel = shader.reverse([pixel[0], pixel[1], pixel[2]]);
                    *pixel = image::Rgba([
                        reversed_pixel[0],
                        reversed_pixel[1],
                        reversed_pixel[2],
                        pixel[3],
                    ]);
                });
            }
        },
    }
    img.save(args.output).unwrap();
}
