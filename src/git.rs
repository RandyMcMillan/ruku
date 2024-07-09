use std::fs::File;
use std::io::{BufRead, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::{fs, io};

use crate::misc::sanitize_app_name;
use crate::server_config::ServerConfig;

pub struct Git<'a> {
    config: &'a ServerConfig,
}

impl<'a> Git<'a> {
    pub fn new(config: &'a ServerConfig) -> Self {
        Self { config }
    }

    fn cmd_git_receive_pack(&self, app: &str) -> io::Result<()> {
        let app = sanitize_app_name(app);
        let hook_path = self.config.git_root.join(&app).join("hooks").join("post-receive");

        if !hook_path.exists() {
            fs::create_dir_all(hook_path.parent().unwrap())?;

            // Initialize the repository with a hook to this script
            Command::new("git")
                .args(["init", "--quiet", "--bare", &app])
                .current_dir(self.config.git_root.display().to_string())
                .status()?;

            let hook_content = format!(
                r#"#!/usr/bin/env bash
set -e; set -o pipefail;
cat | RUKU_ROOT="{}" {} git-hook {}"#,
                self.config.ruku_root.display().to_string(),
                self.config.ruku_binary.display().to_string(),
                app
            );

            let mut file = File::create(&hook_path)?;
            file.write_all(hook_content.as_bytes())?;

            // Make the hook executable by our user
            let mut perms = fs::metadata(&hook_path)?.permissions();
            perms.set_mode(perms.mode() | 0o100);
            fs::set_permissions(&hook_path, perms)?;
        }

        // Handle the actual receive. We'll be called with 'git-hook' after it happens
        let git_command = format!("git-shell -c \"git-receive-pack '{}'\"", app);
        Command::new("sh")
            .arg("-c")
            .arg(&git_command)
            .current_dir(self.config.git_root.display().to_string())
            .status()?;

        Ok(())
    }

    fn cmd_git_hook(&self, app: &str) -> io::Result<()> {
        let app = sanitize_app_name(app);
        let repo_path = self.config.git_root.join(&app);
        let app_path = self.config.apps_root.join(&app);
        let data_path = self.config.data_root.join(&app);

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split(' ').collect();
            if parts.len() != 3 {
                eprintln!("Invalid input line: {}", line);
                continue;
            }
            let (old_rev, new_rev, ref_name) = (parts[0], parts[1], parts[2]);

            if !app_path.exists() {
                println!("-----> Creating app '{}'", app);
                fs::create_dir_all(&app_path)?;

                if !data_path.exists() {
                    fs::create_dir_all(&data_path)?;
                }

                Command::new("git")
                    .args([
                        "clone",
                        "--quiet",
                        repo_path.to_str().unwrap(),
                        app_path.to_str().unwrap(),
                    ])
                    .current_dir(self.config.apps_root.display().to_string())
                    .status()?;
            }
        }

        Ok(())
    }
}
