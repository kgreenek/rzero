extern crate clap;
extern crate hound;
extern crate io;
extern crate rzero;

fn main() {
    let args = clap::App::new("Draw diff_squared")
            .version("v0.1.0")
            .about("Graphs the diff_squared out for a mono wav file")
            .arg(clap::Arg::with_name("WAV_FILE")
                 .help("Input wav file")
                 .required(true)
                 .index(1))
            .arg(clap::Arg::with_name("window_size")
                 .long("window_size")
                 .help("Window size to use for diff_squared")
                 .takes_value(true))
            .get_matches();
    let wav_file = args.value_of("WAV_FILE").unwrap();
    println!("Opening wav file ({})...", wav_file);
    let mut wav_reader = hound::WavReader::open(wav_file).unwrap();
    let num_samples = wav_reader.len() as u32;
    let sample_rate = wav_reader.spec().sample_rate as u32;
    let mut samples = wav_reader.samples::<i16>();
    println!("Num samples {}", num_samples);
    println!("Sample rate {}", sample_rate);
    let pitch_at_zero = rzero::extract_pitch(samples, sample_rate as f64);
    println!("Pitch at 0 {}", pitch_at_zero);
}
