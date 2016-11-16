extern crate ordered_float;
use std::process;
use std::env;
mod mylib;
use mylib::{dto, log, calc, types as t, pik as p};
#[macro_use]
extern crate lazy_static;

fn main() {

  lazy_static! {
		static ref OPS:t::GlobalOps = {
  // Put all dynamic vars inside lazy_static
		  let default_ops = t::GlobalOps {
		    threads: 4,
		    min_delta: 2.0,
		    min_size: 8,
		    max_size: 533,
		    min_tail: 3,
		    max_tail: 18,
		    tail: 55,
		    file: "FILE_NAME".to_owned(),
		    c_piks_with_data: false,
		    c_file_piks: false,
		    c_dir_piks: false,
		    c_deltas_prop: false,
		    c_dists_prop: false,
		    c_tails: false,
		    c_tails_with_vol: false,
		    c_tails_upward: false,
        c_data_ops: false,
        c_piks_dists_deltas: false,
        c_thread_info: false,
		  };
		  let args: Vec<_> = env::args().collect();
		  let named_args = dto::get_named_args(&args);
	    dto::get_global_ops(&default_ops, &named_args)
		};
	}

  let raw_file_data: Vec<f32> = dto::read_file(&OPS.file);

  let file_data: Vec<t::FileData> =
    dto::wrap_file_data(&raw_file_data);

  let file_avgs: Vec<f32> = dto::make_avgs(&file_data);

  let thread_avgs_per_thread: Vec<i32> =
    dto::get_avgs_per_thread(&OPS, &file_avgs);

  let calc_ops_per_thread: Vec<Vec<t::CalcOps>> =
    dto::get_calc_ops_per_thread(&OPS, &thread_avgs_per_thread);

  let data_ops: t::DataOps = dto::get_data_ops(&file_avgs);

  let is_good_min_delta: bool =
    dto::is_good_min_delta(&OPS, &data_ops);

  if OPS.c_data_ops {
    log::c_data_ops(&data_ops);
    log::conts(&raw_file_data);
    log::avgs(&file_avgs);
  }

  if !is_good_min_delta {
    log::no_delta();
    process::exit(1);
  }

  let c_piks_only = !OPS.c_piks_with_data;

  let all_piks = calc::make_piks(&OPS,
                                 data_ops.clone(),
                                 file_avgs,
                                 calc_ops_per_thread);
  if OPS.c_file_piks {
    log::c_piks(c_piks_only, &all_piks, &file_data);
  }
  let file_data: Vec<t::FileData> =
    dto::add_dirs_to_file_data(&all_piks, &file_data);

  let all_dir_piks: Vec<p::Pik> =
    calc::make_dir_piks(data_ops.clone(), &file_data);

  if OPS.c_dir_piks {
    log::c_piks(c_piks_only, &all_dir_piks, &file_data);
  }

  let deltas_prop: Vec<t::DeltasPropagation> =
    calc::make_deltas_prop(&all_dir_piks);

  if OPS.c_deltas_prop {
    log::c_deltas_prop(&deltas_prop);
  }

  let dists_prop: Vec<t::DistPropagation> =
    calc::make_dists_prop(&all_dir_piks);

  if OPS.c_dists_prop {
    log::c_dists_prop(&dists_prop);
  }

  let props_perc: t::PercPropagation =
    calc::make_props_perc(all_dir_piks.len() as i32,
                          &deltas_prop,
                          &dists_prop);

  if OPS.c_piks_dists_deltas {
    log::c_piks_dists_deltas(&props_perc);
  }

  if OPS.c_tails {
    let is_upward = OPS.c_tails_upward;
    let with_vol = OPS.c_tails_with_vol;

    let up_tails: Vec<Vec<t::TailDirs>> =
      calc::make_tails(is_upward,
                       &OPS.tail,
                       &file_data,
                       &all_dir_piks);
    log::c_tails(with_vol, &up_tails);
  }



}
