// Copyright 2021 Wondwise Authors. All rights reserved. GPL-3.0 license.

use clap::ArgMatches;
use std::{thread, time::Duration};

pub fn setup(_args: &ArgMatches) {
    println!("Starting session");

    for i in 1..6 {
        thread::sleep(Duration::from_millis(50));
        println!("Starting... {}", i);
    }

    println!("Finished session");
}
