use std::process::Command;
use std::process;
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use std::path::Display;

struct GlOps {
  threads: i32,
  min_delta_from: f32,
  min_delta_to: f32,
  min_delta_step: f32,
  min_size_from: i32,
  min_size_to: i32,
  min_size_step: i32,
  max_size_from: i32,
  max_size_to: i32,
  max_size_step: i32,
  min_tail_from: i32,
  min_tail_to: i32,
  min_tail_step: i32,
  max_tail_from: i32,
  max_tail_to: i32,
  max_tail_step: i32,

  c_piks_dists_deltas: i32,
}

#[derive(Debug)]
struct Datas {
  piks: f32,
  dists_am: f32,
  dists: f32,
  deltas_am: f32,
  deltas: f32,
  min_delta: f32,
  min_size: f32,
  max_size: f32,
  min_tail: f32,
  max_tail: f32,
  log_str: String,
}

fn main() {

  static OPS: GlOps = GlOps {
    threads: 4,

    min_delta_from: 2.0,
    min_delta_to: 3.0,
    min_delta_step: 0.5,

    max_size_from: 200,
    max_size_to: 301,
    max_size_step: 50,

    min_size_from: 20,
    min_size_to: 21,
    min_size_step: 20,
    min_tail_from: 6,
    min_tail_to: 7,
    min_tail_step: 6,
    max_tail_from: 15,
    max_tail_to: 16,
    max_tail_step: 15,

    c_piks_dists_deltas: 1,
  };

  static MODE: i32 = 1;
  static MIN_PIKS_SEL: f32 = 15.0;
  static MIN_DISTS_SEL: f32 = 80.0;
  static MIN_DELTAS_SEL: f32 = 90.0;
  static MIN_DISTS_AMOUNT_SEL: f32 = 7.0;
  static MIN_DELTAS_AMOUNT_SEL: f32 = 5.0;
  static MIN_ONE_DELTA_SEL: f32 = 2.0;
  static DATA_DIR: &'static str = "./data1/";
  static RESULTS_DIR: &'static str = "./res1/";
  static SEL_DIR: &'static str = "./sel5/";

  static COM_NAME: &'static str = "./hello_world1.exe";

  if MODE == 0 {
    let file_names: Vec<String> = get_file_names(DATA_DIR);
    for f_name in file_names {
      let data_file_str_path = DATA_DIR.to_string() + &f_name;
      let log_file_str_path = RESULTS_DIR.to_string() + &f_name;
      let path = Path::new(&log_file_str_path);
      let display = path.display();
      let log_file = open_log_file(path, &display);

      let args: Vec<Vec<String>> =
        make_args(&OPS, &data_file_str_path);

      println!("Args len: {}", args.len());

      for arg in args {
        let mut command_result: String = run_command(COM_NAME,
                                                     &arg);
        for a_str in arg {
          command_result = command_result + ", " + &a_str;
        }
        command_result = command_result + "\n";

        write_to_log(&log_file, &display, &command_result);
      }
    }
  }

  if MODE == 1 {
    let file_names: Vec<String> = get_file_names(RESULTS_DIR);
    for f_name in file_names {
      let log_file_str_path = RESULTS_DIR.to_string() + &f_name;
      let file_data: Vec<f32> = read_file(&log_file_str_path);
      let datas: Vec<Datas> = get_datas(&file_data,
                                        MIN_PIKS_SEL,
                                        MIN_DISTS_SEL,
                                        MIN_DISTS_AMOUNT_SEL,
                                        MIN_DELTAS_SEL,
                                        MIN_DELTAS_AMOUNT_SEL,
                                        MIN_ONE_DELTA_SEL);
      if datas.len() > 0 {
        let sel_file_str_path = SEL_DIR.to_string() + &f_name;
        let path = Path::new(&sel_file_str_path);
        let display = path.display();
        let log_file = open_log_file(path, &display);
        for one_data in datas {
          write_to_log(&log_file, &display, &one_data.log_str);
        }
      }
    }
  }
}


// ********************* METHODS **************************

fn get_file_names(data_dir: &str) -> Vec<String> {
  let paths = fs::read_dir(data_dir).unwrap();
  let mut names: Vec<String> = vec![];
  for path in paths {
    let one_path: String = path.unwrap()
                               .file_name()
                               .to_str()
                               .unwrap()
                               .to_owned();
    names.push(one_path.to_owned());
    // println!("File name: {}", one_path);
  }
  names
}

fn run_command(com_name: &str, args: &Vec<String>) -> String {

  // println!("Com name: {}", com_name);
  let output = Command::new(com_name)
                 .args(args)
                 .output()
                 .unwrap_or_else(|e| {
                   panic!("failed to execute process: {}", e)
                 });
  // let stat = output.status;
  let out = String::from_utf8(output.stdout).unwrap();
  // let err = String::from_utf8_lossy(&output.stderr);
  // println!("status: {}", stat);
  // println!("stdout: {}", out);
  // println!("stderr: {}", err);
  out
}

fn make_args(ops: &GlOps, f_name: &str) -> Vec<Vec<String>> {

  let mut args: Vec<Vec<String>> = vec![];
  let mut min_delta: f32 = ops.min_delta_from;
  let threads_str = ops.threads.to_string();
  let c_piks_dists_deltas_str = ops.c_piks_dists_deltas
                                   .to_string();

  while min_delta <= ops.min_delta_to {
    let min_delta_str = min_delta.to_string();

    for min_size in ops.min_size_from..ops.min_size_to {
      if min_size % ops.min_size_step != 0 {
        continue;
      }
      let min_size_str = min_size.to_string();

      for max_size in ops.max_size_from..ops.max_size_to {
        if max_size % ops.max_size_step != 0 {
          continue;
        }
        let max_size_str = max_size.to_string();

        for min_tail in ops.min_tail_from..ops.min_tail_to {
          if min_tail % ops.min_tail_step != 0 {
            continue;
          }
          let min_tail_str = min_tail.to_string();

          for max_tail in ops.max_tail_from..ops.max_tail_to {
            if max_tail % ops.max_tail_step != 0 {
              continue;
            }
            let max_tail_str = max_tail.to_string();

            let one_arg: Vec<String> = vec!["file".to_owned(),
                   f_name.to_owned(),
                   "threads".to_owned(),
                   threads_str.to_owned(),
                   "c_piks_dists_deltas".to_owned(),
                   c_piks_dists_deltas_str.to_owned(),

                   "min_delta".to_owned(),
                   min_delta_str.to_owned(),

                   "min_size".to_owned(),
                   min_size_str.to_owned(),
                   "max_size".to_owned(),
                   max_size_str.to_owned(),

                   "min_tail".to_owned(),
                   min_tail_str.to_owned(),
                   "max_tail".to_owned(),
                   max_tail_str.to_owned()];
            args.push(one_arg);
          }
        }
      }
    }
    min_delta += ops.min_delta_step;
  }
  args
}

fn open_log_file(path: &Path, display: &Display) -> File {
  // Open a file in write-only mode, returns `io::Result<File>`
  let file = match File::create(path) {
    Err(why) => {
      panic!("couldn't create {}: {}",
             display,
             Error::description(&why))
    }
    Ok(file) => file,
  };
  file
}

fn write_to_log(mut log_file: &File,
                display: &Display,
                command_result: &str) {
  match log_file.write_all(command_result.as_bytes()) {
    Err(why) => {
      panic!("couldn't write to {}: {}",
             display,
             Error::description(&why))
    }
    Ok(_) => println!("successfully wrote to {}", display),
  }
}

fn read_file(f_name: &str) -> Vec<f32> {
  // Create a path to the desired file
  let path = Path::new(f_name);
  let display = path.display();
  // Open the path in read-only mode, returns `io::Result<File>`
  let mut file = match File::open(&path) {
    // The `description` method of `io::Error` returns a string that
    // describes the error
    Err(why) => {
      panic!("couldn't open {}: {}",
             display,
             Error::description(&why))
    }
    Ok(file) => file,
  };
  // Read the file contents into a string, returns `io::Result<usize>`
  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => {
      panic!("couldn't read {}: {}",
             display,
             Error::description(&why))
    }
    Ok(_) => print!("{}, ", display),
  }
  s.replace("\n", ", ")
   .split(", ")
   .filter_map(|x| {
     match x.parse::<f32>() {
       Ok(n) => Some(n),
       Err(_) => None,
     }
   })
   .collect::<Vec<f32>>()
}

fn get_datas(f_data: &Vec<f32>,
             mps: f32,
             mdis: f32,
             mdis_am: f32,
             mdel: f32,
             mdel_am: f32,
             min_one_del: f32)
             -> Vec<Datas> {
  let mut datas: Vec<Datas> = vec![];
  for fd_win in f_data.chunks(12) {

    // println!("one line {:?}, ", fd_win);
    // process::exit(1);

    if fd_win[0] >= mps && fd_win[7] >= min_one_del &&
       ((fd_win[2] >= mdis && fd_win[4] >= mdel) ||
        (fd_win[2] >= mdel && fd_win[4] >= mdis)) &&
       ((fd_win[1] <= mdis_am && fd_win[3] <= mdel_am) ||
        (fd_win[1] <= mdel_am && fd_win[3] <= mdis_am)) {

      let dlen = datas.len();
      let dlast_idx = if dlen > 0 {
        dlen - 1
      } else {
        0
      };

      if dlen == 0 ||
         !(datas[dlast_idx].piks == fd_win[0] &&
           datas[dlast_idx].dists == fd_win[1] &&
           datas[dlast_idx].deltas == fd_win[2]) {
        let log_str: String = fd_win[0].to_string() + " " +
                              &(fd_win[1].to_string()) +
                              " " +
                              &(fd_win[2].to_string()) +
                              " " +
                              &(fd_win[3].to_string()) +
                              " " +
                              &(fd_win[4].to_string()) +
                              " " +
                              &(fd_win[7].to_string()) +
                              " " +
                              &(fd_win[8].to_string()) +
                              " " +
                              &(fd_win[9].to_string()) +
                              " " +
                              &(fd_win[10].to_string()) +
                              " " +
                              &(fd_win[11].to_string()) +
                              "\n";
        datas.push(Datas {
          piks: fd_win[0],
          dists_am: fd_win[1],
          dists: fd_win[2],
          deltas_am: fd_win[3],
          deltas: fd_win[4],
          min_delta: fd_win[7],
          min_size: fd_win[8],
          max_size: fd_win[9],
          min_tail: fd_win[10],
          max_tail: fd_win[11],
          log_str: log_str,
        });
      }
    }
  }
  datas
}
