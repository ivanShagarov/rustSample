pub struct GlobalOps {
  pub threads: i32,
  pub min_delta: f32,
  pub min_size: i32,
  pub max_size: i32,
  pub min_tail: i32,
  pub max_tail: i32,
  pub tail: i32,
  pub file: String,

  pub c_piks_with_data: bool,
  pub c_file_piks: bool,
  pub c_dir_piks: bool,
  pub c_deltas_prop: bool,
  pub c_dists_prop: bool,
  pub c_tails: bool,
  pub c_tails_with_vol: bool,
  pub c_tails_upward: bool,
  pub c_data_ops: bool,
  pub c_piks_dists_deltas: bool,
  pub c_thread_info: bool,
}


#[derive(Clone)]
pub struct Otype {
  pub num: i32,
  pub flt: f32,
  pub strn: String,
  pub bln: bool,
}

#[derive(Clone)]
pub struct DataOps {
  pub max_perc_delta: f32,
  pub max_val_delta: f32,
  pub one_perc_delta: f32,
  pub max_idx: i32,
  pub min_idx: i32,
  pub extrem_distance_idx: i32,
  pub one_perc_distance: f32,
  pub dir: bool,
}

pub struct CalcOps {
  pub amount: i32,
  pub shift: i32,
  pub end: i32,
  pub pos: i32,
  pub hor_size: i32,
  pub tail_size: i32,
}

pub struct FileData {
  pub pos: i32,
  pub date: f32,
  pub avg: f32,
  pub vol: f32,
  pub dir: i32,
  pub tail_dir: i32,
  pub vol_dir: i32,
}

#[derive(Clone)]
pub struct ICurState {
  pub value: i32,
  pub start_idx: i32,
  pub length: i32,
}

#[derive(Clone)]
pub struct IState {
  pub groups: Vec<ICurState>,
  pub cur_state: ICurState,
}

#[derive(Clone)]
pub struct Deltas {
  pub max_avg: f32,
  pub min_avg: f32,
  pub delta_avg: f32,
  pub delta_perc: f32,
}

#[derive(Clone)]
pub struct DeltasPropagation {
  pub delta_round_perc: i32,
  pub delta_round_perc_amount: i32,
}

#[derive(Clone)]
pub struct DistPropagation {
  pub dist_round_perc: i32,
  pub dist_round_perc_amount: i32,
}

#[derive(Clone)]
pub struct PercPropagation {
  pub piks_amount: i32,
  pub dist_amount: i32,
  pub dist_perc: f32,
  pub delta_amount: i32,
  pub delta_perc: f32,
}

#[derive(Clone)]
pub struct TailDirs {
  pub avg_dir: i32,
  pub vol_dir: i32,
}
