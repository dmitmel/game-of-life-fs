use clap;

use std::path::PathBuf;

const DIR_ARG: &str = "DIR";
const WIDTH_ARG: &str = "WIDTH";
const HEIGHT_ARG: &str = "HEIGHT";
const PROBABILITY_ARG: &str = "PROBABILITY";

/// This struct holds options passed via [CLI](parse_options).
///
/// _See also:_ [`run`](super::run)
pub struct Options {
  /// Path to a directory for the [rendered](super::rename_row_files)
  /// [world](super::world::World).
  pub game_dir: PathBuf,
  /// [World](super::world::World) width.
  pub world_width: usize,
  /// [World](super::world::World) height.
  pub world_height: usize,
  /// See [`fill_world`](super::fill_world).
  pub fill_probability: f32,
}

/// Parses command-line arguments into an [`Options`] struct.
pub fn parse_options() -> Result<Options, clap::Error> {
  let matches = create_app().get_matches_safe()?;
  Ok(Options {
    game_dir: PathBuf::from(matches.value_of_os(DIR_ARG).unwrap()),
    world_width: value_t!(matches, WIDTH_ARG, usize)?,
    world_height: value_t!(matches, HEIGHT_ARG, usize)?,
    fill_probability: value_t!(matches, PROBABILITY_ARG, f32)?,
  })
}

fn create_app<'a, 'b>() -> clap::App<'a, 'b> {
  app_from_crate!()
    .setting(clap::AppSettings::ColoredHelp)
    .arg(
      clap::Arg::with_name(DIR_ARG)
        .help("a directory for the rendered world")
        .required(true),
    )
    .arg(
      clap::Arg::with_name(WIDTH_ARG)
        .help("world width")
        .required(true),
    )
    .arg(
      clap::Arg::with_name(HEIGHT_ARG)
        .help("world height")
        .required(true),
    )
    .arg(
      clap::Arg::with_name(PROBABILITY_ARG)
        .long("random")
        .short("r")
        .value_name("PROBABILITY")
        .help(
          "Before the simulation begins, the world is filled randomly with \
           alive cells. This option sets the chance of a cell being filled.",
        )
        .default_value("0.2")
        .takes_value(true),
    )
}
