use ordered_float::*;
// use std::process;
use std::sync::Arc;

use mylib::types as t;

#[derive(PartialEq)]
pub struct Pik {
  // Init data
  pub pos: i32,
  pub end_pos: i32,
  pub hor_size: i32,
  pub tail_size: i32,
  pub delta_avg: f32,
  pub delta_perc: f32,

  // Processed
  pub start_date: f32,
  pub end_date: f32,
  pub delta_date: f32,
  pub top_avg: f32,
  pub bottom_avg: f32,
  pub dist_perc: f32,
  pub upward: bool,
  pub is_pik: bool,
}

impl Pik {
  pub fn new(pos: i32,
             hor_size: i32,
             tail_size: i32,
             min_delta: f32,
             one_perc_delta: f32,
             file_avgs: &Arc<Vec<f32>>)
             -> Pik {

    let mut is_pik = false;
    let mut upward = false;

    let mut begin = pos - tail_size;
    let mut end = pos + hor_size;
    let file_avgs_len = (file_avgs.len() as i32) - 1;

    if begin < 0 {
      begin = 0;
    }
    if end >= file_avgs_len {
      end = file_avgs_len;
    }

    let begin_pos = begin as usize;
    let end_pos = end as usize;
    let slice_top_pik_pos = (tail_size - 1) as usize;
    let slice_end_pik_pos = (end - begin - 1) as usize;

    let (max_idx, max_item) =
      file_avgs[begin_pos..end_pos]
        .iter()
        .enumerate()
        .max_by_key(|&(_, item)| {
          // It's possible to pass some inner
          // property of struct here, like *item.value
          // And iterate through array of structs!
          OrderedFloat(*item)
        })
        .unwrap();

    let (min_idx, min_item) = file_avgs[begin_pos..end_pos]
                                .iter()
                                .enumerate()
                                .min_by_key(|&(_, item)| {
                                  OrderedFloat(*item)
                                })
                                .unwrap();

    let cur_delta = max_item - min_item;
    let cur_delta_perc = cur_delta / one_perc_delta;

    if cur_delta_perc >= min_delta {

      // Check if it's an downward pik
      if slice_top_pik_pos == max_idx &&
         slice_end_pik_pos == min_idx {
        is_pik = true;
      }
      // Check if it's an upward pik
      if slice_top_pik_pos == min_idx &&
         slice_end_pik_pos == max_idx {
        is_pik = true;
        upward = true;
      }
    }

    Pik {
      pos: pos,
      end_pos: end - 1,
      hor_size: hor_size,
      tail_size: tail_size,
      upward: upward,
      is_pik: is_pik,
      delta_avg: cur_delta,
      delta_perc: cur_delta_perc,
      top_avg: *max_item,
      bottom_avg: *min_item,

      // fill that later, only if it's a pik
      start_date: 0.0,
      end_date: 0.0,
      delta_date: 0.0,
      dist_perc: 0.0,
    }
  }

  pub fn from_dirs(pos: i32,
                   end: i32,
                   hor_size: i32,
                   upward: bool,
                   is_pik: bool,
                   one_perc_delta: f32,
                   one_perc_distance: f32,
                   file_data: &Vec<t::FileData>)
                   -> Pik {

    let begin_pos = pos as usize;
    let end_pos = end as usize;

    let max_item = file_data[begin_pos..end_pos]
                     .iter()
                     .max_by_key(|item| OrderedFloat(item.avg))
                     .unwrap();

    let min_item = file_data[begin_pos..end_pos]
                     .iter()
                     .min_by_key(|item| OrderedFloat(item.avg))
                     .unwrap();

    let cur_delta = max_item.avg - min_item.avg;
    let cur_delta_perc = cur_delta / one_perc_delta;
    let start_date = file_data[begin_pos].date;
    let end_date = file_data[end_pos].date;
    let delta_date = end_date - start_date;
    let dist_perc = hor_size as f32 / one_perc_distance;

    Pik {
      pos: pos,
      end_pos: end,
      hor_size: hor_size,
      dist_perc: dist_perc,
      upward: upward,
      is_pik: is_pik,
      delta_avg: cur_delta,
      delta_perc: cur_delta_perc,
      top_avg: max_item.avg,
      bottom_avg: min_item.avg,
      start_date: start_date,
      end_date: end_date,
      delta_date: delta_date,
      // ToDo: Tail is unknown!
      // Try to guess it from avg tail,
      // or from avg pik?
      tail_size: 0,
    }
  }
}
