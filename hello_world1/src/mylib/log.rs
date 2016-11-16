use mylib::pik as p;
use mylib::types as t;

pub fn conts(data: &Vec<f32>) {
  print!("File length {}, contains:\nDate {}, Avg: {}, Vol: {}\n",
           data.len(), data[0], data[1], data[2],
          );
}

pub fn avgs(avg_data: &Vec<f32>) {
  print!("Avgs length {}, contains:\nFirst Avg: {}, Last Avg: {}\n",
           avg_data.len(), avg_data[0], avg_data[avg_data.len() - 1],
          );
}

pub fn no_delta() {
  println!("You have no possible min delta in the file! \
            Exiting :(");
}

pub fn c_pik(pik: &p::Pik) {
  println!("Pik pos: {}, end_pos: {}, hor_size: {}, \
            tail_size: {}, delta_avg: {}, delta_perc: {}%, \
            dist_perc: {}%, upward: {}",
           pik.pos,
           pik.end_pos,
           pik.hor_size,
           pik.tail_size,
           pik.delta_avg,
           pik.delta_perc,
           pik.dist_perc,
           pik.upward);
}

pub fn c_avg_and_dir(avg: &f32, up_down: &i32) {
  println!("{}, {}", avg, up_down);
}

pub fn c_data_ops(data_ops: &t::DataOps) {
  print!("\n");
  println!("DataOps dir upward {}, max_idx {}, min_idx \
            {}, extrem_distance_idx {}, one_perc_distance {}, one_perc_delta {}, max_val_delta {}, max_perc_delta {}",
           data_ops.dir,
           data_ops.max_idx,
           data_ops.min_idx,
           data_ops.extrem_distance_idx,
           data_ops.one_perc_distance,
           data_ops.one_perc_delta,
           data_ops.max_val_delta,
           data_ops.max_perc_delta,
          );
  print!("\n");
}

pub fn c_piks(piks_only: bool,
              all_piks: &Vec<p::Pik>,
              file_data: &Vec<t::FileData>) {
  let all_piks_len = all_piks.len() as i32;
  println!("Total amount of piks: {}", all_piks_len);
  if piks_only {
    for pik in all_piks {
      c_pik(&pik);
    }
  } else {
    for one_data in file_data {
      c_avg_and_dir(&one_data.avg, &one_data.dir);
    }
  }
}

pub fn c_thread(thread: &i32,
                calc_ops_len: &i32,
                filtered_thread_piks_len: &i32) {
  println!("Thread {}, total iterations: {}, found piks: {}",
           thread,
           calc_ops_len,
           filtered_thread_piks_len);
}

pub fn c_deltas_prop(deltas_prop: &Vec<t::DeltasPropagation>) {
  let max_amount_prop = deltas_prop.iter()
                                   .max_by_key(|item| {
                                     item.delta_round_perc_amount
                                   })
                                   .unwrap();
  let max_value_prop = deltas_prop.iter()
                                  .max_by_key(|item| {
                                    item.delta_round_perc
                                  })
                                  .unwrap();
  println!("Total DeltasPropagation: {}, Max \
            DeltasPropagation Amount: {}, Max \
            DeltasPropagation Value {}",
           deltas_prop.len() as i32,
           max_amount_prop.delta_round_perc_amount,
           max_value_prop.delta_round_perc);
  for one_prop in deltas_prop {
    println!("Amount {}, Value: {}",
             one_prop.delta_round_perc_amount,
             one_prop.delta_round_perc);
  }
}

pub fn c_piks_dists_deltas(props_perc: &t::PercPropagation) {
  print!("{}, {}, {}, {}, {}",
         props_perc.piks_amount,
         props_perc.dist_amount,
         props_perc.dist_perc,
         props_perc.delta_amount,
         props_perc.delta_perc);
}

pub fn c_dists_prop(dists_prop: &Vec<t::DistPropagation>) {
  let max_amount_prop = dists_prop.iter()
                                  .max_by_key(|item| {
                                    item.dist_round_perc_amount
                                  })
                                  .unwrap();
  let max_value_prop = dists_prop.iter()
                                 .max_by_key(|item| {
                                   item.dist_round_perc
                                 })
                                 .unwrap();
  println!("Total DistPropagation: {}, Max DistPropagation \
            Amount: {}, Max DistPropagation Value {}",
           dists_prop.len() as i32,
           max_amount_prop.dist_round_perc_amount,
           max_value_prop.dist_round_perc);
  for one_prop in dists_prop {
    println!("Amount {}, Value: {}",
             one_prop.dist_round_perc_amount,
             one_prop.dist_round_perc);
  }
}

pub fn c_tails(with_vol: bool, tails: &Vec<Vec<t::TailDirs>>) {
  for one_tail in tails {
    print!("\nTail: \n");
    for one_value in one_tail {
      if with_vol {
        print!("{}, {}\n", one_value.avg_dir, one_value.vol_dir);
      } else {
        print!("{}\n", one_value.avg_dir);
      }
    }
  }
}
