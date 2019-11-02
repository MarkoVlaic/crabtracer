extern crate clap;
use clap::{ App, Arg };

pub struct CmdArgs {
    pub width: u32,
    pub height: u32,
    pub sample_rate: u32,
    pub filename: String
}

impl CmdArgs {
    pub fn new() -> Result<Self, &'static str> {
        let matches = App::new("Crabtraycer")
            .version("1.0")
            .author("Marko Vlaić <vlaic.marko@gmail.com>")
            .about("Outputs a png image with a 3d scene rendered from the input world")
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .help("Path of the output image file")
                    .takes_value(true)
                    .default_value("out.png")   
            )
            .arg(
                Arg::with_name("width")
                    .short("w")
                    .long("width")
                    .help("Width of the output image")
                    .takes_value(true)
                    .default_value("600")
            )
            .arg(
                Arg::with_name("height")
                    .short("h")
                    .long("height")
                    .help("Height of the output image")
                    .takes_value(true)
                    .default_value("400")
            )
            .arg(
                Arg::with_name("sample_rate")
                    .short("s")
                    .long("sample_rate")
                    .help("Sample rate for antialiasing")
                    .takes_value(true)
                    .default_value("100")
            )
            .get_matches();

        let width = matches.value_of("width").unwrap().parse().map_err(|e|return e).unwrap();
        let height = matches.value_of("height").unwrap().parse().map_err(|e|return e).unwrap();
        let sample_rate = matches.value_of("sample_rate").unwrap().parse().map_err(|e|return e).unwrap();
        let filename = matches.value_of("output").unwrap().to_owned();


        Ok(CmdArgs{ width, height, sample_rate, filename })
    }
}
