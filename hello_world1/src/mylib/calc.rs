use std::thread;
use std::sync::Arc;
use mylib::types as t;
use mylib::pik as p;
use mylib::log;
// use std::process;


pub fn make_piks(ops: &'static t::GlobalOps,
                 data_ops: t::DataOps,
                 file_avgs: Vec<f32>,
                 calc_ops: Vec<Vec<t::CalcOps>>)
                 -> Vec<p::Pik> {

  let shared_file_avgs = Arc::new(file_avgs);
  let shared_calc_ops = Arc::new(calc_ops);
  let shared_data_ops = Arc::new(data_ops);
  let mut all_piks: Vec<p::Pik> = vec![];

  // ToDo: use thread pool and send jobs to them
  // Pass the amount of indexes and the shift to each thread
  // Get every pik within thread, no min max
  let handles: Vec<_> = (0..ops.threads)
                          .map(|thread| {

                            let thread_file_avgs =
                              shared_file_avgs.clone();
                            let thread_calc_ops =
                              shared_calc_ops.clone();
                            let thread_data_ops =
                              shared_data_ops.clone();

                            thread::spawn(move || {
                              let tpointer = thread as usize;
                              let mut thread_piks: Vec<p::Pik> =
            thread_calc_ops[tpointer]
              .iter()
              .map(|map_calc_ops| {
                p::Pik::new(map_calc_ops.pos,
                            map_calc_ops.hor_size,
                            map_calc_ops.tail_size,
                            ops.min_delta,
                            thread_data_ops.one_perc_delta,
                            &thread_file_avgs)
              })
              .filter(|pik| pik.is_pik)
              .collect();

                              let thread_piks_len =
                                thread_piks.len() as i32;
                              let mut filter_prev_pos = 0;

                              // Filter
                              for thread_pik_pos in
                                  0..thread_piks_len {
                                let point =
                                  thread_pik_pos as usize;
                                let prev_point =
                                  filter_prev_pos as usize;
                                if thread_pik_pos == 0 {
                                  filter_prev_pos =
                                    thread_pik_pos;
                                }
                                if thread_piks[point].pos ==
                                   thread_piks[prev_point].pos &&
                                   thread_piks[point]
                                     .delta_avg >=
                                   thread_piks[prev_point]
                                     .delta_avg {
                                  filter_prev_pos =
                                    thread_pik_pos;
                                  thread_piks[prev_point]
                                    .is_pik = false;
                                }
                                if thread_piks[point].pos !=
                                   thread_piks[prev_point].pos {
                                  filter_prev_pos =
                                    thread_pik_pos;
                                }
                              }

                              thread_piks.retain(|pik| {
                                pik.is_pik
                              });

                              let calc_ops_len =
            thread_calc_ops[tpointer].len() as i32;
                              let filtered_thread_piks_len =
                                thread_piks.len() as i32;
                              if ops.c_thread_info {
                                log::c_thread(&thread,
                           &calc_ops_len,
                           &filtered_thread_piks_len);
                              }
                              thread_piks
                            })
                          })
                          .collect();
  for h in handles {
    let res = h.join()
               .map_err(|_| "Could not join a thread!")
               .unwrap();
    for pik in res {
      all_piks.push(pik);
    }
  }
  return all_piks;
}



pub fn make_dir_piks(data_ops: t::DataOps,
                     file_data: &Vec<t::FileData>)
                     -> Vec<p::Pik> {

  let mut all_piks: Vec<p::Pik> = vec![];
  let mut pik: p::Pik;

  let groups =
    file_data.iter()
             .fold(t::IState {
                     groups: vec![t::ICurState {
                                    value: 0,
                                    start_idx: 0,
                                    length: 0,
                                  }],
                     cur_state: t::ICurState {
                       value: 0,
                       start_idx: 0,
                       length: 0,
                     },
                   },
                   |immut_state, x| {

                     let mut state = immut_state.clone();

                     // if dir is 0 skip element
                     if x.dir == 0 {
                       state.cur_state = t::ICurState {
                         value: 0,
                         start_idx: 0,
                         length: 0,
                       };
                       return state;
                     }

                     // if dir is 1 or -1 add val, idx, length,
                     if x.dir == 1 || x.dir == -1 {

                       let last_group_len = state.groups.len();
                       let last_group_idx =
                         (last_group_len - 1) as usize;

                       // if dir is different start new state
                       if state.cur_state.value != x.dir {
                         state.cur_state = t::ICurState {
                           value: x.dir,
                           start_idx: x.pos,
                           length: 1,
                         };
                         state.groups
                              .push(state.cur_state.clone());
                         // if dir is same add to current state
                       } else {
                         state.cur_state = t::ICurState {
                           value: x.dir,
                           start_idx:
                             state.groups[last_group_idx]
                               .start_idx,
                           length: state.groups[last_group_idx]
                                     .length +
                                   1,
                         };
                         state.groups.remove(last_group_idx);
                         state.groups
                              .push(state.cur_state.clone());
                       }
                     }
                     state
                   });

  // print!("\n groups len, first is empty {} \n",
  //        groups.groups.len() as i32);

  for group in groups.groups {
    // print!("\n start_idx {}, val {}, pik len {} \n",
    //                              group.start_idx,
    //                              group.value,
    //                              group.length,
    //                         );
    if group.value != 0 {
      pik = p::Pik::from_dirs(group.start_idx,
                              group.start_idx + group.length,
                              group.length,
                              if group.value == 1 {
                                true
                              } else {
                                false
                              },
                              true,
                              data_ops.one_perc_delta,
                              data_ops.one_perc_distance,
                              &file_data);
      all_piks.push(pik);
    }
  }

  all_piks
}


pub fn make_deltas_prop(all_dir_piks: &Vec<p::Pik>)
                        -> Vec<t::DeltasPropagation> {
  all_dir_piks.iter()
              .fold(vec![t::DeltasPropagation {
                           delta_round_perc: 0,
                           delta_round_perc_amount: 0,
                         }],
                    |immut_state, pik| {

                      let pik_delta_round_perc =
                        pik.delta_perc.round() as i32;
                      let mut state = immut_state.to_owned();

                      let pos_in_state =
                        state.iter()
                             .position(|state_el| {
                               state_el.delta_round_perc ==
                               pik_delta_round_perc
                             });

                      match pos_in_state {
                        Some(found_state_pos) => {
                          state.push(t::DeltasPropagation {
                            delta_round_perc:
                              pik_delta_round_perc,
                            delta_round_perc_amount:
                              immut_state[found_state_pos]
                                .delta_round_perc_amount +
                              1,
                          });
                          state.remove(found_state_pos);
                        }
                        None => {
                          if state[0].delta_round_perc_amount ==
                             0 {
                            state.remove(0);
                          }
                          state.push(t::DeltasPropagation {
                            delta_round_perc:
                              pik_delta_round_perc,
                            delta_round_perc_amount: 1,
                          });
                        }
                      }
                      state
                    })
}

pub fn make_dists_prop(all_dir_piks: &Vec<p::Pik>)
                       -> Vec<t::DistPropagation> {
  all_dir_piks.iter()
              .fold(vec![t::DistPropagation {
                           dist_round_perc: 0,
                           dist_round_perc_amount: 0,
                         }],
                    |immut_state, pik| {

                      let pik_dist_round_perc =
                        pik.dist_perc.round() as i32;
                      let mut state = immut_state.to_owned();

                      let pos_in_state =
                        state.iter()
                             .position(|state_el| {
                               state_el.dist_round_perc ==
                               pik_dist_round_perc
                             });

                      match pos_in_state {
                        Some(found_state_pos) => {
                          state.push(t::DistPropagation {
                            dist_round_perc: pik_dist_round_perc,
                            dist_round_perc_amount:
                              immut_state[found_state_pos]
                                .dist_round_perc_amount +
                              1,
                          });
                          state.remove(found_state_pos);
                        }
                        None => {
                          if state[0].dist_round_perc_amount ==
                             0 {
                            state.remove(0);
                          }
                          state.push(t::DistPropagation {
                            dist_round_perc: pik_dist_round_perc,
                            dist_round_perc_amount: 1,
                          });
                        }
                      }
                      state
                    })
}

pub fn make_props_perc(piks_amount: i32,
                       deltas_prop: &Vec<t::DeltasPropagation>,
                       dists_prop: &Vec<t::DistPropagation>)
                       -> t::PercPropagation {
  let one_perc = piks_amount as f32 / 100.0;
  let dist_perc: f32 =
    dists_prop.iter().fold(0.0, |state, prop| {
      if prop.dist_round_perc_amount > 1 {
        state + (prop.dist_round_perc_amount as f32 / one_perc)
      } else {
        state
      }
    });
  let delta_perc: f32 =
    deltas_prop.iter().fold(0.0, |state, prop| {
      if prop.delta_round_perc_amount > 1 {
        state + (prop.delta_round_perc_amount as f32 / one_perc)
      } else {
        state
      }
    });

  let dist_amount: i32 = dists_prop.len() as i32;
  let delta_amount: i32 = deltas_prop.len() as i32;

  t::PercPropagation {
    piks_amount: piks_amount,
    dist_amount: dist_amount,
    dist_perc: dist_perc,
    delta_amount: delta_amount,
    delta_perc: delta_perc,
  }
}

pub fn make_tails(is_upward: bool,
                  max_tail: &i32,
                  file_data: &Vec<t::FileData>,
                  all_dir_piks: &Vec<p::Pik>)
                  -> Vec<Vec<t::TailDirs>> {
  all_dir_piks.iter()
              .fold(vec![vec![t::TailDirs {
                                avg_dir: 88,
                                vol_dir: 88,
                              }]],
                    |immut_state, pik| {
                      let tail_start_pos = pik.pos - max_tail;
                      let mut state = immut_state.to_owned();

                      if tail_start_pos <= 0 ||
                         pik.upward != is_upward {
                        return immut_state;
                      }

                      let start = tail_start_pos as usize;
                      let end = pik.pos as usize;

                      let tail =
                        file_data[start..end]
                          .iter()
                          .map(|x| {
                            t::TailDirs {
                              avg_dir: x.tail_dir,
                              vol_dir: x.vol_dir,
                            }
                          })
                          .collect::<Vec<t::TailDirs>>();
                      state.push(tail);
                      if state[0][0].avg_dir == 88 {
                        state.remove(0);
                      }
                      state
                    })
}
