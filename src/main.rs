use anyhow::Context;
use clap::Parser;
use image::{EncodableLayout, ImageFormat, ImageReader};
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
struct Args {
    #[arg(long, short)]
    input: PathBuf,

    #[arg(long, short = 'f', value_enum)]
    to_format: Format,

    #[arg(long, short)]
    output_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum Format {
    Jpg,
    Png,
    Tiff,
    Webp,
    Avif,
}

impl Format {
    pub fn extension(&self) -> &'static str {
        match self {
            Format::Jpg => "jpg",
            Format::Png => "png",
            Format::Tiff => "tiff",
            Format::Webp => "webp",
            Format::Avif => "avif",
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let from_image = ImageReader::open(&args.input)
        .with_context(|| format!("Failed to open image file {:?}", args.input))?
        .decode()
        .with_context(|| format!("Failed to decode image file {:?}", args.input))?;

    let to_image_path = match args.output_dir {
        Some(ref output_dir) => output_dir.join(args.input.file_name().unwrap()),
        None => args.input.clone(),
    }
    .with_extension(args.to_format.extension());

    if args.input == to_image_path {
        anyhow::bail!("Input and output are the same file");
    }

    let to_image_format = match to_image_path.extension() {
        Some(ext) => ImageFormat::from_extension(ext)
            .with_context(|| format!("Unsupported output file extension: {:?}", ext)),
        None => anyhow::bail!("Missing output file extension: {:?}", to_image_path),
    }?;

    match to_image_format {
        ImageFormat::Jpeg => {
            image::save_buffer_with_format(
                &to_image_path,
                from_image.to_rgb8().as_bytes(),
                from_image.width(),
                from_image.height(),
                image::ColorType::Rgb8,
                to_image_format,
            )
            .with_context(|| format!("Failed to save image file {:?}", to_image_path))?;
        }
        _ => {
            image::save_buffer_with_format(
                &to_image_path,
                from_image.to_rgba8().as_bytes(),
                from_image.width(),
                from_image.height(),
                image::ColorType::Rgba8,
                to_image_format,
            )
            .with_context(|| format!("Failed to save image file {:?}", to_image_path))?;
        }
    };

    Ok(())
}
