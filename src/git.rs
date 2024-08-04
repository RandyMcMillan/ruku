use std::fs::File;
use std::io::{BufRead, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{fs, io};

use cmd_lib::run_cmd;

use crate::logger::Logger;
use crate::misc::sanitize_app_name;
use crate::server_config::ServerConfig;

pub struct Git<'a> {
    log: &'a Logger,
    config: &'a ServerConfig,
}

impl<'a> Git<'a> {
    pub fn new(log: &'a Logger, config: &'a ServerConfig) -> Self {
        Self { log, config }
    }

    pub fn cmd_git_receive_pack(&self, app: &str) {
        let app = sanitize_app_name(app);
        let git_root = self.config.git_root.as_path().to_str().unwrap();
        let hook_path = self.config.git_root.join(&app).join("hooks").join("post-receive");

        if !hook_path.exists() {
            self.log.step("Initializing git repository");
            fs::create_dir_all(hook_path.parent().unwrap()).unwrap_or_else(|e| {
                self.log.error(&format!("Error creating directory: {}", e));
                std::process::exit(1);
            });

            run_cmd!(
                cd $git_root;
                git init --quiet --bare $app;
            )
            .unwrap_or_else(|e| {
                self.log.error(&format!("Error executing git init: {}", e));
                std::process::exit(1);
            });

            let hook_content = format!(
                r#"#!/usr/bin/env bash
set -e; set -o pipefail;
cat | RUKU_ROOT="{}" {} git-hook {}
"#,
                self.config.ruku_root.display(),
                self.config.ruku_binary.display(),
                app
            );

            let mut file = File::create(&hook_path).unwrap_or_else(|e| {
                self.log.error(&format!("Error creating file: {}", e));
                std::process::exit(1);
            });
            file.write_all(hook_content.as_bytes()).unwrap_or_else(|e| {
                self.log.error(&format!("Error writing to file: {}", e));
                std::process::exit(1);
            });

            // Make the hook executable by our user
            let mut perms = fs::metadata(&hook_path).unwrap().permissions();
            perms.set_mode(perms.mode() | 0o100);
            fs::set_permissions(&hook_path, perms).unwrap_or_else(|e| {
                self.log.error(&format!("Error setting permissions: {}", e));
                std::process::exit(1);
            });
        }

        // Handle the actual receive. We'll be called with 'git-hook' after it happens
        self.run_git_shell(git_root, format!("git-receive-pack '{}'", app));
    }

    pub fn cmd_git_upload_pack(&self, app: &str) {
        let app = sanitize_app_name(app);
        let git_root = self.config.git_root.as_path().to_str().unwrap();

        self.run_git_shell(git_root, format!("git-upload-pack '{}'", app));
    }

    fn run_git_shell(&self, git_root: &str, git_command: String) {
        run_cmd!(
            cd $git_root;
            git-shell -c "$git_command";
        )
        .unwrap_or_else(|e| {
            self.log.error(&format!("Error executing git shell: {}", e));
            std::process::exit(1);
        });
    }

    pub fn cmd_git_hook(&self, app: &str) {
        let app = sanitize_app_name(app);

        let repo_path = self.config.git_root.join(&app);
        let app_path = self.config.apps_root.join(&app);
        let data_path = self.config.data_root.join(&app);

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split(' ').collect();
            if parts.len() != 3 {
                continue;
            }
            let (_, new_rev, _) = (parts[0], parts[1], parts[2]);

            if !app_path.exists() {
                fs::create_dir_all(&app_path).unwrap_or_else(|e| {
                    self.log.error(&format!("Error creating directory: {}", e));
                    std::process::exit(1);
                });

                if !data_path.exists() {
                    fs::create_dir_all(&data_path).unwrap_or_else(|e| {
                        self.log.error(&format!("Error creating directory: {}", e));
                        std::process::exit(1);
                    });
                }

                self.log.step("Cloning git repository");
                run_cmd!(git clone --quiet --no-checkout $repo_path $app_path).unwrap_or_else(|e| {
                    self.log.error(&format!("Error cloning git repo: {}", e));
                    std::process::exit(1);
                });
            }

            self.checkout_latest(&app_path, new_rev);
        }
    }

    fn checkout_latest(&self, app_path: &Path, new_rev: &str) {
        self.log.step("Checking out the latest code");
        run_cmd!(
            git --git-dir=$app_path/.git fetch --quiet;
            git --git-dir=$app_path/.git reset --hard $new_rev;
        )
        .unwrap_or_else(|e| {
            self.log.error(&format!("Error checking out latest code: {}", e));
            std::process::exit(1);
        });
    }
}
