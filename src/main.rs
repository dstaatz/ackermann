/* Copyright (c) 2020 Dylan Staatz */


use std::collections::HashMap;
use std::fs::File;
use std::time::Instant;
use std::str::FromStr;

use ron::ser::{to_writer_pretty, PrettyConfig};


type I = u32;


fn ackermann(m: I, n: I) -> I {
    if m == 0 {
        return n+1;
    } else if n == 0 {
        return ackermann(m-1, 1);
    } else {
        return ackermann(m-1, ackermann(m, n-1));
    }
}


fn ackermann_cached(cache: &mut HashMap<(I, I), I>, m: I, n: I) -> I {

    // Cache lookup
    if let Some(ret) = cache.get(&(m,n)) {
        return *ret;
    }

    if m == 0 {
        let ret = n+1;
        cache.insert((m,n), ret);
        ret
    } else if n == 0 {
        let ret = ackermann_cached(cache, m-1, 1);
        cache.insert((m,n), ret);
        ret
    } else {
        let temp = ackermann_cached(cache, m, n-1);
        let ret = ackermann_cached(cache, m-1, temp);
        cache.insert((m,n), ret);
        ret
    }
}


fn main() {

    let args: Vec<String> = std::env::args().collect();

    let now = Instant::now();

    if args.len() == 1 {

        // Non-Cached version
        for i in 0..6 {
            for j in 0..6 {
                let result = ackermann(i, j);
                let time = now.elapsed().as_secs_f32();
                println!("{:10.6}: ackermann ({},{}) is: {}", time, i, j, result);
            }
        }

    } else if args.len() == 2 && &args[1] == "cached" {
        
        // Cached version
        let mut cache = HashMap::new();
        for i in 0..6 {
            for j in 0..6 {
                let result = ackermann_cached(&mut cache, i, j);
                let time = now.elapsed().as_secs_f32();
                println!("{:10.6}: ackermann_cached ({},{}) is: {}", time, i, j, result);
                let file = File::create("data/cache").unwrap();
                to_writer_pretty(file, &cache, PrettyConfig::new()).unwrap();
            }
        }

    } else if args.len() == 3 {

        let i = I::from_str(&args[1]).unwrap();
        let j = I::from_str(&args[2]).unwrap();
        let result = ackermann(i, j);
        let time = now.elapsed().as_secs_f32();
        println!("{:10.6}: ackermann ({},{}) is: {}", time, i, j, result);

    } else if args.len() == 4 && &args[1] == "cached" {

        let mut cache = HashMap::new();
        let i = I::from_str(&args[2]).unwrap();
        let j = I::from_str(&args[3]).unwrap();
        let result = ackermann_cached(&mut cache, i, j);
        let time = now.elapsed().as_secs_f32();
        println!("{:10.6}: ackermann_cached ({},{}) is: {}", time, i, j, result);
        let file = File::create(format!("data/cache_{}-{}", i,j)).unwrap();
        to_writer_pretty(file, &cache, PrettyConfig::new()).unwrap();

    } else {
        println!("useage: ackermann [cached] [[m] [n]]");
    }
}
