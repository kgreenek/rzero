extern crate clap;
extern crate hound;
extern crate rzero;
extern crate sample;

use sample::Signal;

fn main() {
    let args = clap::App::new("Draw wav")
            .version("v0.1.0")
            .about("Graphs the diff_squared out for a mono wav file")
            .arg(clap::Arg::with_name("WAV_FILE")
                 .help("Input wav file")
                 .required(true)
                 .index(1))
            .get_matches();
    let wav_file = args.value_of("WAV_FILE").unwrap();
    println!("Opening wav file ({})...", wav_file);
    let wav_reader = hound::WavReader::open(wav_file).unwrap();
    let wav_spec = wav_reader.spec();
    let num_samples = wav_reader.len() as u32;
    let sample_rate = wav_spec.sample_rate as u32;
    let samples = wav_reader.into_samples::<i16>().filter_map(Result::ok);
    let frames = sample::signal::from_interleaved_samples_iter::<_, [i16; 1]>(samples)
            .until_exhausted();
    println!("Sample rate {}", sample_rate);
    println!("Num wav_reader samples {}", num_samples);
    println!("Num frames {}", frames.count());
}
