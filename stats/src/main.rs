use rand::Rng;




fn main() {
  //  let args: Vec<String> = std::env::args().collect();
    let mut rnd_nbrs = generate_rand(5000, 50000);
  //  display_nbrs(&rnd_nbrs, 10);

   rnd_nbrs.sort();
   display_nbrs(&rnd_nbrs, 11);


    
}

fn display_nbrs(nbrs: &Vec<u64>, cols: u32) {
    let mut index:u32 = 0;
    print!("\n{:10} -> ",index);
    while  index < nbrs.len() as u32 {
        print!("{:10}",&nbrs[index as usize]);
        index += 1;
        if index % cols  == 0 {
            print!("\n{:10} -> ",index);
        }
    }
}

fn generate_rand( nbr_of_nbrs: usize ,largest_nbr: u64) -> Vec<u64> {
    let mut nbrs = Vec::new();
    let mut p: u64;
    while nbrs.len() < nbr_of_nbrs {
        p = rand::thread_rng().gen_range(1, largest_nbr);
        if !nbrs.iter().any(|&i| i == p) {
            nbrs.push(p);
        }
    }
    return nbrs;
}