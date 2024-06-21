// use tqdm::tqdm;
use simple_tqdm::ParTqdm;
use rayon::prelude::*;

fn is_prime(n: u64) -> bool {
    if n < 2 {
         return false;
	}
    if n % 2 == 0 {
         return n == 2;
	}
    let mut k = 3;
    while k*k <= n {
         if n % k == 0 {
             return false;
	     }
         k += 2;
	}
    return true;
}

fn is_caboose(m: u64) -> bool {
    for n in 0 .. m {
        let x = n.pow(2) - n + m;
        if !is_prime(x) {
            return false
		}
	}
    return true;
}

fn main() {
	(0 .. 1_000_000_000 as u32).into_par_iter().tqdm().for_each(|n| {
		let p = is_prime(n.into());
		if p {
			let c = is_caboose(n.into());
			if c {
				println!("{}\t{}\t{}", n, p, c);
			}
		}
	});
}
