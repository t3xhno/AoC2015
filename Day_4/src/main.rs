extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let mut hasher = Md5::new();

    let key = "iwrupvqb".as_bytes();
    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        
        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        // >> 0 za part 2, a >> 4 za part 1
        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 0) as i32;
        if first_five == 0 {
            println!("{}", i);
            break;
        }
        hasher.reset();
    }
}