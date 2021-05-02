// Copyright (C) 2021 Wondwise Authors. All rights reserved. GPL-3.0 license.

use crate::dirs::WondwiseDirs;
use std::fs;
use uuid::Uuid;

pub struct Session {
    id: Uuid,
}

impl Session {
    pub fn new() -> Self {
        let session = Self { id: Uuid::new_v4() };

        #[cfg(unix)]
        let session_path = format!(
            "{}/sessions/{}",
            WondwiseDirs::load().cache_dir().to_str().unwrap(),
            session.id
        );

        #[cfg(windows)]
        let session_path = format!(
            "{}\\sessions\\{}",
            WondwiseDirs::load().cache_dir().to_str().unwrap(),
            session.id
        );

        // Create session dir
        fs::create_dir_all(session_path).unwrap();

        return session;
    }
}
