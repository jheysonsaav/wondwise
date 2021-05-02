// Copyright (C) 2021 Wondwise Authors. All rights reserved. GPL-3.0 license.

use std::{path::Path, process::exit};

use clap::ArgMatches;
use wondwise_utils::logs::{Log, LogLevel};

pub fn setup(args: &ArgMatches) {
    let file = args.value_of("file").unwrap_or("main.wd");
    let file_path = Path::new(&file);

    if !file_path.exists() {
        Log::new(LogLevel::Error, 1, "This path does not exist").show();
        exit(1);
    } else {
        Log::new(LogLevel::Debug, 2, "This path exist").show();
    }

    if !file_path.is_file() {
        Log::new(LogLevel::Error, 1, "This path is not a file").show();
        exit(1);
    } else {
        Log::new(LogLevel::Debug, 2, "This path is a file").show();
    }

    if !file.ends_with(".wd") {
        Log::new(LogLevel::Error, 1, "this file is not a valid wondwise file").show();
        exit(1);
    } else {
        Log::new(LogLevel::Debug, 2, "This file is a valid wondwise file").show();
    }
}
