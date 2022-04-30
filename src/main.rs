extern crate bit_vec;
#[macro_use]
extern crate clap;
extern crate rand;

use std::io;
use std::io::prelude::*;

use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

mod cli;
mod utils;
mod world;

use utils::ResultContextExt;
use world::World;

fn main() {
  run().unwrap_or_else(|e| e.exit());
}

fn run() -> clap::Result<()> {
  let options = cli::parse_options()?;

  let mut world = World::new(options.world_width, options.world_height);
  // fill_world(&mut world, options.fill_probability);

  let pattern = &[
    "........................O...........",
    "......................O.O...........",
    "............OO......OO............OO",
    "...........O...O....OO............OO",
    "OO........O.....O...OO..............",
    "OO........O...O.OO....O.O...........",
    "..........O.....O.......O...........",
    "...........O...O....................",
    "............OO......................",
    ".......................O............",
    "........................OO..........",
    ".......................OO...........",
  ];

  for (y, line) in pattern.iter().enumerate() {
    for (x, chr) in line.chars().enumerate() {
      world.set(2 + x, 1 + y, chr == 'O');
    }
  }

  create_game_directory(&options.game_dir)?;
  let mut row_files = create_row_files(&options.game_dir, &world)?;

  print_welcome();

  use std::thread::sleep;
  use std::time::Duration;
  sleep(Duration::from_millis(2000));

  for generation in 0.. {
    println!("generation #{}:", generation);
    println!("{}", world.render());

    row_files = rename_row_files(&options.game_dir, &row_files, &world)?;

    // print prompt
    print!("> ");
    io::stdout().flush().context("couldn't flush stdout")?;

    let mut line = String::new();
    // let stdin = io::stdin();
    // let bytes = stdin
    //   .lock()
    //   .read_line(&mut line)
    //   .context("couldn't get input")?;

    use std::thread::sleep;
    use std::time::Duration;
    sleep(Duration::from_millis(1500));
    let bytes = if generation < 60 {
      println!();
      1
    } else {
      println!("exit");
      0
    };

    println!();

    if bytes == 0 || line == "exit\n" {
      println!("bye");
      break;
    } else {
      world = world.next_generation();
    }
  }

  remove_row_files(&options.game_dir, &row_files)?;

  Ok(())
}

#[allow(dead_code)]
fn fill_world(world: &mut World, probability: f32) {
  for y in 0..world.height {
    for x in 0..world.width {
      world.set(x, y, rand::random::<f32>() < probability);
    }
  }
}

fn print_welcome() {
  println!(
    "Welcome to {} v{}!",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION")
  );
  println!("press [enter] to show the next generation");
  println!("press [Ctrl-D] or type [exit] to exit");
  println!();
}

fn create_game_directory(dir: &Path) -> io::Result<()> {
  fs::create_dir_all(dir)
    .context(&format!("couldn't create directory '{}'", dir.display()))
}

fn create_row_files(dir: &Path, world: &World) -> io::Result<Vec<PathBuf>> {
  (0..world.height)
    .map(|row| -> io::Result<PathBuf> {
      let file_path = dir.join(format!("{:02}", row));

      File::create(&file_path)
        .map(|_| file_path)
        .context(&format!("couldn't create file for row {}", row))
    })
    .collect::<io::Result<Vec<PathBuf>>>()
}

pub fn rename_row_files(
  dir: &Path,
  current_files: &[PathBuf],
  world: &World,
) -> io::Result<Vec<PathBuf>> {
  current_files
    .iter()
    .enumerate()
    .map(|(row, current_file)| -> io::Result<PathBuf> {
      let row_data = world.render_row(row);
      let new_file = dir.join(format!("{:02} {}", row, row_data));

      fs::rename(current_file, &new_file)
        .context(&format!("couldn't rename file for row {}", row))
        .map(|_| new_file)
    })
    .collect()
}

pub fn remove_row_files(dir: &Path, files: &[PathBuf]) -> io::Result<()> {
  for (row, file) in files.iter().enumerate() {
    fs::remove_file(file)
      .context(&format!("couldn't remove file for row {}", row))?;
  }
  fs::remove_dir(dir).context("couldn't clean up the game directory")?;
  Ok(())
}
