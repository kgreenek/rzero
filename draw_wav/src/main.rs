extern crate clap;
extern crate hound;
extern crate rustplotlib;
extern crate rzero;
extern crate sample;

use rustplotlib::Figure;
use sample::{Sample, Signal};

fn make_figure<'a>(
    x: &'a [f64],
    wav_data: &'a [f64],
    pitch_data: &'a [f64],
    rms_data: &'a [f64],
    duration_secs: f64,
) -> Figure<'a> {
    use rustplotlib::{Axes2D, Line2D};
    let ax1 = Axes2D::new()
        .add(
            Line2D::new("WAV file")
                .data(x, wav_data)
                .color("green")
                .marker("")
                .linestyle("-")
                .linewidth(1.0),
        )
        .grid(true)
        .xlabel("Time (s)")
        .ylabel("Value")
        .xlim(0.0, duration_secs)
        .ylim(-1.5, 1.5);

    let ax2 = Axes2D::new()
        .add(
            Line2D::new("Pitch Hz")
                .data(x, pitch_data)
                .color("red")
                .marker("")
                .linestyle("-")
                .linewidth(1.0),
        )
        .grid(true)
        .xlabel("Time (s)")
        .ylabel("Pitch (hz)")
        .xlim(0.0, duration_secs)
        .ylim(0.0, 1000.0);

    let ax3 = Axes2D::new()
        .add(
            Line2D::new("RMS")
                .data(x, rms_data)
                .color("blue")
                .marker("")
                .linestyle("-")
                .linewidth(1.0),
        )
        .grid(true)
        .xlabel("Time (s)")
        .ylabel("RMS")
        .xlim(0.0, duration_secs)
        .ylim(0.0, 1.0);

    Figure::new().subplots(3, 1, vec![Some(ax1), Some(ax2), Some(ax3)])
}

fn main() {
    let args = clap::App::new("Draw WAV")
        .version("v0.1.0")
        .about(concat!(
            "Renders WAV_FILE using matplotlib\n",
            "WAV_FILE format must be 16-bit int with 1 channel"
        ))
        .arg(
            clap::Arg::with_name("WAV_FILE")
                .help("Input wav file")
                .required(true)
                .index(1),
        )
        .get_matches();
    let wav_file = args.value_of("WAV_FILE").unwrap();
    println!("Opening wav file ({})...", wav_file);
    let wav_reader = hound::WavReader::open(wav_file).unwrap();
    let wav_spec = wav_reader.spec();
    let num_frames = wav_reader.len() / wav_spec.channels as u32;
    let sample_rate = wav_spec.sample_rate as u32;
    let samples = wav_reader.into_samples::<i16>().filter_map(Result::ok);
    let duration_secs = (num_frames as f64) / (sample_rate as f64);
    let frames = sample::signal::from_interleaved_samples_iter::<_, [i16; 1]>(samples)
        .until_exhausted();
    println!("Sample rate: {} hz", sample_rate);
    println!("Channels: {}", wav_spec.channels);
    println!(
        "Duration: {:.2} seconds ({} samples)",
        duration_secs,
        num_frames
    );

    let frames_vec: Vec<_> = frames.map(|frame| frame[0].to_sample::<f64>()).collect();
    let mut x = vec![0.0 as f64; frames_vec.len()];
    for i in 0..x.len() {
        x[i] = (i as f64) / (sample_rate as f64);
    }

    let mut pitch_extractor = rzero::PitchExtractorContainer::new();
    let mut rms_vec = Vec::with_capacity(frames_vec.len());
    let pitches_vec: Vec<_> = frames_vec
        .iter()
        .map(|frame: &f64| {
            pitch_extractor.add_frames(&[[frame.to_sample::<f32>()]]);
            rms_vec.push(pitch_extractor.rms() as f64);
            pitch_extractor.pitch(sample_rate as f64) as f64
        })
        .collect();

    let figure = make_figure(&x, &frames_vec, &pitches_vec, &rms_vec, duration_secs);

    use rustplotlib::Backend;
    use rustplotlib::backend::Matplotlib;
    let mut mpl = Matplotlib::new().unwrap();
    mpl.set_style("ggplot").unwrap();
    figure.apply(&mut mpl).unwrap();
    mpl.show().unwrap();
    mpl.wait().unwrap();
}
