extern crate rand;

use rand::Rng;

// Shuffle function, but logically not necessary to shuffle the array
// as first person always takes a random seat
// fn shuffle<T>(vec: &mut Vec<T>) {
//     let mut rng = rand::thread_rng();
//     let len = vec.len();
//     for i in 0..len {
//         vec.swap(rng.gen_range(0,len-i),len-i-1);
//     }
// }

fn assign_seats(n: usize)-> Vec<bool>{
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(false);
    }
    //shuffle(&mut vec);
    return vec;
}

// First person takes a random seat
// All following people attempt to take their seat, if it's taken, pick a random seat
fn grab_seats(n: usize)->bool{
    let mut rng = rand::thread_rng();
    let mut seats = assign_seats(n);

    seats[rng.gen_range(0,n)] = true;//first person takes a random seat
    for i in 1..n-1 {
        if seats[n-1] == true {//If that last seat is taken, the last person could never get their seat
            return false;
        }

        if seats[i] == true {//if person i's seat was taken, pick a random one
            seats[rng.gen_range(i,n)] = true;
        } else {
            seats[i] = true;
        }
    }
    return true;//last seat was left open
}

fn main() {
    let iterations = 1000000;
    let mut count: u32 = 0;
    for _ in 0..iterations {
        if grab_seats(100) {
            count+=1;
        }
    }
    println!("Attempted {}\nSucesses {}\nChance {}%",iterations,count,100.0*(count as f64 / iterations as f64));
}
