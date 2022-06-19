use clap::Parser;
use midify_chords::*;

fn main() {
    let args: Args = Args::parse();

    let config = Config::new(args.bpm, args.out_file);
    run(config);
}

/// Convert plaintext chords to MIDI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Song tempo
    #[clap(short, long, value_parser, default_value_t = 120)]
    bpm: u32,
    /// File to output MIDI to
    #[clap(short, long, value_parser)]
    out_file: String,
}
