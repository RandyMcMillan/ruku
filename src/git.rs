use crate::logger::Logger;
use crate::misc::sanitize_app_name;
use crate::server_config::ServerConfig;
use cmd_lib::run_cmd;
use std::fs::File;
use std::io::{BufRead, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{fs, io};

pub struct Git<'a> {
    log: &'a Logger,
    config: &'a ServerConfig,
}

impl<'a> Git<'a> {
    pub fn new(log: &'a Logger, config: &'a ServerConfig) -> Self {
        Self { log, config }
    }

    pub fn cmd_git_receive_pack(&self, app: &str) -> io::Result<()> {
        let app = sanitize_app_name(app);
        let git_root = self.config.git_root.as_path().to_str().unwrap();
        let hook_path = self.config.git_root.join(&app).join("hooks").join("post-receive");

        if !hook_path.exists() {
            self.log.step("Initializing git repository");
            fs::create_dir_all(hook_path.parent().unwrap())?;

            run_cmd!(
                cd $git_root;
                git init --quiet --bare $app;
            )
            .unwrap();

            let hook_content = format!(
                r#"#!/usr/bin/env bash
set -e; set -o pipefail;
cat | RUKU_ROOT="{}" {} git-hook {}
"#,
                self.config.ruku_root.display(),
                self.config.ruku_binary.display(),
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
        let git_command = format!("git-receive-pack '{}'", app);
        run_cmd!(
            cd $git_root;
            git-shell -c "$git_command";
        )?;

        Ok(())
    }

    pub fn cmd_git_upload_pack(&self, app: &str) -> io::Result<()> {
        let app = sanitize_app_name(app);
        let git_root = self.config.git_root.as_path().to_str().unwrap();

        let git_command = format!("git-upload-pack '{}'", app);
        run_cmd!(
            cd $git_root;
            git-shell -c "$git_command";
        )?;

        Ok(())
    }

    pub fn cmd_git_hook(&self, app: &str) -> io::Result<()> {
        let app = sanitize_app_name(app);

        let repo_path = self.config.git_root.join(&app);
        let app_path = self.config.apps_root.join(&app);
        let data_path = self.config.data_root.join(&app);

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split(' ').collect();
            if parts.len() != 3 {
                continue;
            }
            let (_, new_rev, _) = (parts[0], parts[1], parts[2]);

            if !app_path.exists() {
                fs::create_dir_all(&app_path)?;

                if !data_path.exists() {
                    fs::create_dir_all(&data_path)?;
                }

                self.log.step("Cloning git repository");
                run_cmd!(git clone --quiet --no-checkout $repo_path $app_path)?;
            }

            self.checkout_latest(&app_path, new_rev)?;
        }

        Ok(())
    }

    fn checkout_latest(&self, app_path: &Path, new_rev: &str) -> io::Result<()> {
        self.log.step("Checking out the latest code");
        run_cmd!(
            git --git-dir=$app_path/.git fetch --quiet;
            git --git-dir=$app_path/.git reset --hard $new_rev;
        )?;

        Ok(())
    }
}
