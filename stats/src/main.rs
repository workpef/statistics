use rand::Rng;




fn main() {
  //  let args: Vec<String> = std::env::args().collect();
    let mut rnd_nbrs = generate_rand(11, 12);

   let  mean = calc_mean(&rnd_nbrs);
   dbg!(&rnd_nbrs);
   println!("mean -> {}", mean);
   let stddev = calc_stddev(&rnd_nbrs,&mean);
   println!("std dev -> {}", stddev);
   display_nbrs(&rnd_nbrs,5);
   rnd_nbrs.sort();
   display_nbrs(&rnd_nbrs,5);
   print!("\n{}",calc_stddev(&rnd_nbrs,&mean).round( ));
}

fn calc_mean (nbrs: &Vec<i64>) -> f64 {
   
    let mut nbr_sum: i64 = 0;
    let mut cntr = 0;
    for n in nbrs.iter() {
        nbr_sum += n;
        cntr += 1;
    }
    return (nbr_sum/cntr as i64) as f64;
}

/*
Step 1: Find the mean.
Step 2: For each data point, find the square of its distance to the mean.
Step 3: Sum the values from Step 2.
Step 4: Divide by the number of data  .
Step 5: Take the square root.
*/
fn calc_stddev (nbrs: &Vec<i64>, mean: &f64 ) -> f64 {
    let mut sum: i64 = 0;
    let mn = *mean as i64;
    for n in nbrs.iter() {
        let  dist =  num::abs_sub(mn,*n);
        sum += (dist*dist) as i64;
    } 
    return ((sum/nbrs.len() as i64) as f64).sqrt();
}

fn display_nbrs(nbrs: &Vec<i64>, cols: u32) {
    let mut index:u32 = 0;
    print!("\n{:10} -> ",index);
    while  index < nbrs.len() as u32 {
        print!("{:10}",&nbrs[index as usize]);
        index += 1;
        if index % cols  == 0 {
            print!("\n{:10} -> ",index +1);
        }
    }
}

fn generate_rand( nbr_of_nbrs: usize ,largest_nbr: u64) -> Vec<i64> {
    let mut nbrs: Vec<i64>= Vec::new();
    let mut p: i64;
    while nbrs.len() < nbr_of_nbrs {
        p = rand::thread_rng().gen_range(1, largest_nbr) as i64;
        if !nbrs.iter().any(|&i| i == p) {
            nbrs.push(p);
        }
    }
    return nbrs;
}