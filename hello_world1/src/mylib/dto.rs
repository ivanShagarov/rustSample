use ordered_float::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use mylib::types as t;
use mylib::pik as p;


fn get_op_names() -> Vec<&'static str> {
  vec!["threads",
       "min_delta",
       "min_size",
       "max_size",
       "min_tail",
       "max_tail",
       "tail",
       "file",

       "c_piks_with_data",
       "c_file_piks",
       "c_dir_piks",
       "c_deltas_prop",
       "c_dists_prop",
       "c_tails",
       "c_tails_with_vol",
       "c_tails_upward",
       "c_data_ops",
       "c_piks_dists_deltas",
       "c_thread_info",
       ]
}

pub fn get_global_ops(def_ops: &t::GlobalOps,
                      args: &HashMap<String, t::Otype>)
                      -> t::GlobalOps {

  t::GlobalOps {
    threads: match args.get("threads") {
      Some(val) => val.num,
      None => def_ops.threads,
    },
    min_delta: match args.get("min_delta") {
      Some(val) => val.flt,
      None => def_ops.min_delta,
    },
    min_size: match args.get("min_size") {
      Some(val) => val.num,
      None => def_ops.min_size,
    },
    max_size: match args.get("max_size") {
      Some(val) => val.num,
      None => def_ops.max_size,
    },
    min_tail: match args.get("min_tail") {
      Some(val) => val.num,
      None => def_ops.min_tail,
    },
    max_tail: match args.get("max_tail") {
      Some(val) => val.num,
      None => def_ops.max_tail,
    },
    tail: match args.get("tail") {
      Some(val) => val.num,
      None => def_ops.tail,
    },
    file: match args.get("file") {
      Some(val) => val.strn.to_owned(),
      None => def_ops.file.to_owned(),
    },

    c_piks_with_data: match args.get("c_piks_with_data") {
      Some(val) => val.bln,
      None => def_ops.c_piks_with_data,
    },
    c_file_piks: match args.get("c_file_piks") {
      Some(val) => val.bln,
      None => def_ops.c_file_piks,
    },
    c_dir_piks: match args.get("c_dir_piks") {
      Some(val) => val.bln,
      None => def_ops.c_dir_piks,
    },
    c_deltas_prop: match args.get("c_deltas_prop") {
      Some(val) => val.bln,
      None => def_ops.c_deltas_prop,
    },
    c_dists_prop: match args.get("c_dists_prop") {
      Some(val) => val.bln,
      None => def_ops.c_dists_prop,
    },
    c_tails: match args.get("c_tails") {
      Some(val) => val.bln,
      None => def_ops.c_tails,
    },
    c_tails_with_vol: match args.get("c_tails_with_vol") {
      Some(val) => val.bln,
      None => def_ops.c_tails_with_vol,
    },
    c_tails_upward: match args.get("c_tails_upward") {
      Some(val) => val.bln,
      None => def_ops.c_tails_upward,
    },
    c_data_ops: match args.get("c_data_ops") {
      Some(val) => val.bln,
      None => def_ops.c_data_ops,
    },
    c_piks_dists_deltas: match args.get("c_piks_dists_deltas") {
      Some(val) => val.bln,
      None => def_ops.c_piks_dists_deltas,
    },
    c_thread_info: match args.get("c_thread_info") {
      Some(val) => val.bln,
      None => def_ops.c_thread_info,
    },
  }
}

fn check_option_name(arg_name: &str) -> bool {
  let op_names: Vec<&str> = get_op_names().to_owned();
  let mut good_arg = false;
  for op_name in op_names {
    if op_name == arg_name {
      good_arg = true;
      // println!("Good arg! {}, {}", arg[0], arg[1]);
    }
  }
  good_arg
}

pub fn get_named_args(args: &Vec<String>)
                      -> HashMap<String, t::Otype> {
  let mut named_args: HashMap<String, t::Otype> = HashMap::new();
  let mut args = args.to_owned();
  if args.len() > 2 {
    args.remove(0);
    for arg in args.chunks(2) {
      let op_name = arg[0].clone();
      let val = arg[1].clone();

      if !check_option_name(&op_name) {
        println!("Bad arg! {}, {}", op_name, val);
      } else {
        let parsed_val: f32 = match val.parse() {
          Ok(n) => n,
          Err(_) => 0.0,
        };
        let int_val = parsed_val.round() as i32;
        let bool_val = to_bool(int_val);
        let typed_val = t::Otype {
          num: int_val,
          flt: parsed_val.to_owned(),
          strn: val.to_owned(),
          bln: bool_val,
        };
        named_args.insert(op_name, typed_val);
      }
    }
  }
  named_args
}

fn to_bool(val: i32) -> bool {
  if val == 0 {
    false
  } else if val == 1 {
    true
  } else {
    false
  }
}

pub fn read_file(f_name: &str) -> Vec<f32> {
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
  s.split_whitespace()
   .filter_map(|x| {
     match x.parse::<f32>() {
       Ok(n) => Some(n),
       Err(_) => None,
     }
   })
   .collect::<Vec<f32>>()
}

pub fn wrap_file_data(raw_file_data: &Vec<f32>)
                      -> Vec<t::FileData> {
  let mut wrapped_file_data: Vec<t::FileData> = Vec::new();
  for win in raw_file_data.chunks(3) {
    let one_data = t::FileData {
      pos: 0,
      date: win[0],
      avg: win[1],
      vol: win[2],
      dir: 0,
      tail_dir: 0,
      vol_dir: 0,
    };
    wrapped_file_data.push(one_data);
  }
  return wrapped_file_data;
}

pub fn make_avgs(file_data: &Vec<t::FileData>) -> Vec<f32> {
  let mut file_avgs: Vec<f32> = Vec::new();
  for el in file_data.iter() {
    file_avgs.push(el.avg);
  }
  return file_avgs;
}


// Calc avgs amount for each thread, add tail to last one
pub fn get_avgs_per_thread(ops: &t::GlobalOps,
                           file_avgs: &Vec<f32>)
                           -> Vec<i32> {
  let file_avgs_len = file_avgs.len() as i32;
  let rem = file_avgs_len % ops.threads;
  let clean_all = file_avgs_len - rem;
  let clean_part = clean_all / ops.threads;
  (0..ops.threads)
    .map(|thread| {
      if rem != 0 && thread == ops.threads - 1 {
        return rem + clean_part;
      }
      clean_part
    })
    .collect::<Vec<i32>>()
}

// Calc avgs indexes range and other options to iterate
// for each thread
pub fn get_calc_ops_per_thread(ops: &t::GlobalOps,
                               all_thread_avgs: &Vec<i32>)
                               -> Vec<Vec<t::CalcOps>> {
  (0..ops.threads)
    .map(|thread| {
      let thread_pointer = thread as usize;
      let thread_avgs_amount = all_thread_avgs[thread_pointer];
      let mut all_calc_ops: Vec<t::CalcOps> =
        Vec::with_capacity(thread_avgs_amount as usize);
      let avgs_shift = if thread != 0 {
        all_thread_avgs[0] * thread
      } else {
        0
      };
      let thread_avgs_end_pos = avgs_shift + thread_avgs_amount;
      if ops.c_thread_info {
        println!("setup thread {} am {} sh {} end {}",
                 thread,
                 thread_avgs_amount,
                 avgs_shift,
                 thread_avgs_end_pos);
      }
      // Iterate positions for current thread
      for pos in avgs_shift..thread_avgs_end_pos {
        for size_hor in ops.min_size..ops.max_size {
          for tail_size in ops.min_tail..ops.max_tail {
            let one_calc_ops = t::CalcOps {
              amount: thread_avgs_amount,
              shift: avgs_shift,
              end: thread_avgs_end_pos,
              pos: pos,
              hor_size: size_hor,
              tail_size: tail_size,
            };
            all_calc_ops.push(one_calc_ops);
          }
        }
      }
      // println!("got ops {}", all_calc_ops.len());
      all_calc_ops
    })
    .collect::<Vec<Vec<t::CalcOps>>>()
}

pub fn get_data_ops(file_avgs: &Vec<f32>) -> t::DataOps {
  let (max_idx, max_item) =
    file_avgs.iter()
             .enumerate()
             .max_by_key(|&(_, item)| {
               // It's possible to pass some inner
               // property of struct here, like *item.value
               // And iterate through array of structs!
               OrderedFloat(*item)
             })
             .unwrap();
  let (min_idx, min_item) = file_avgs.iter()
                                     .enumerate()
                                     .min_by_key(|&(_, item)| {
                                       OrderedFloat(*item)
                                     })
                                     .unwrap();

  let max_distance = file_avgs.len() as f32;
  let max_idx_int = max_idx as i32;
  let min_idx_int = min_idx as i32;
  let max_item_f: f32 = *max_item;
  let min_item_f: f32 = *min_item;
  let delta: f32 = max_item_f - min_item_f;
  let distance: i32;
  let dir: bool;
  // Check if it's an downward pik
  if max_idx_int < min_idx_int {
    distance = min_idx_int - max_idx_int;
    dir = false;
  } else {
    distance = max_idx_int - min_idx_int;
    dir = true;
  }

  let one_perc_delta: f32 = max_item_f / 100.0;
  let max_perc_delta: f32 = delta / one_perc_delta;
  let one_perc_distance: f32 = max_distance / 100.0;


  t::DataOps {
    max_perc_delta: max_perc_delta,
    max_val_delta: delta,
    one_perc_delta: one_perc_delta,
    max_idx: max_idx_int,
    min_idx: min_idx_int,
    extrem_distance_idx: distance,
    one_perc_distance: one_perc_distance,
    dir: dir,
  }
}

pub fn is_good_min_delta(ops: &t::GlobalOps,
                         data_ops: &t::DataOps)
                         -> bool {
  data_ops.max_perc_delta >= ops.min_delta
}


pub fn add_dirs_to_file_data(all_piks: &Vec<p::Pik>,
                             file_data: &Vec<t::FileData>)
                             -> Vec<t::FileData> {

  let mut wrapped_file_data: Vec<t::FileData> = Vec::new();

  for (idx, one_data) in file_data.iter().enumerate() {
    let dir: i32;

    let data_pos_is_in_pik_pos = all_piks.iter()
                                         .position(|pik| {
                                           pik.pos <
                                           idx as i32 &&
                                           pik.end_pos >
                                           idx as i32
                                         });
    match data_pos_is_in_pik_pos {
      Some(found_pik_pos) => {
        if all_piks[found_pik_pos].upward {
          dir = 1;
        } else {
          dir = -1;
        }
      }
      None => dir = 0,
    }

    let tail_dir: i32;
    let vol_dir: i32;

    if idx as i32 != 0 {

      if one_data.avg >= file_data[idx - 1].avg {
        tail_dir = 1;
      } else {
        tail_dir = 0;
      }

      if one_data.vol >= file_data[idx - 1].vol {
        vol_dir = 1;
      } else {
        vol_dir = 0;
      }

    } else {
      tail_dir = 0;
      vol_dir = 0;
    }

    let tmp_one_data = t::FileData {
      pos: idx as i32,
      date: one_data.date,
      avg: one_data.avg,
      vol: one_data.vol,
      dir: dir,
      tail_dir: tail_dir,
      vol_dir: vol_dir,
    };


    wrapped_file_data.push(tmp_one_data);
  }

  wrapped_file_data

}
