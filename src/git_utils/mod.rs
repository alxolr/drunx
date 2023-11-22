use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Git<'a> {
    path: &'a PathBuf,
    dry_run: bool,
    no_verify: bool,
}

impl<'a> Git<'a> {
    pub fn new(path: &'a PathBuf, dry_run: bool, no_verify: bool) -> Git<'a> {
        Git {
            path,
            dry_run,
            no_verify,
        }
    }

    pub fn run_release(&self, from: &str, to: &str) -> Result<(), Box<dyn Error>> {
        println!("git fetch");
        println!("git checkout {}", to);
        println!("git reset --hard origin/{}", from);
        println!("git push -f");

        if !self.dry_run {
            Command::new("git")
                .args(&["fetch"])
                .stdout(Stdio::null())
                .current_dir(&self.path.as_path())
                .spawn()
                .expect("git fetch failed")
                .wait()
                .expect("failed to finish git fetch");

            Command::new("git")
                .args(&["checkout", to])
                .stdout(Stdio::null())
                .current_dir(&self.path.as_path())
                .spawn()
                .expect("git checkout failed")
                .wait()
                .expect("failed to finsih git checkout");

            Command::new("git")
                .args(&["reset", "--hard", &format!("origin/{}", from)])
                .stdout(Stdio::null())
                .current_dir(&self.path.as_path())
                .spawn()
                .expect("git reset failed")
                .wait()
                .expect("failed to finish git reset");

            self.run_git_push(false, true);
        }

        Ok(())
    }

    pub fn run(&self, version: &str) -> Result<(), Box<dyn Error>> {
        println!("git add . ");
        println!("git commit -a -m \"@{} release\"", version);
        println!(
            "git tag -a \"@{}\" -m \"Added '@{} release' version\"",
            version, version,
        );
        println!("git push -f");
        println!("git push --tags");

        if !self.dry_run {
            self.run_git_add();
            self.run_git_commit(version);
            self.run_git_tag(version);
            self.run_git_push(false, true);
            self.run_git_push(true, false);
        }

        Ok(())
    }

    pub fn git_fetch(&self, prune: bool) -> Result<(), Box<dyn Error>> {
        println!("git fetch --prune");

        if !self.dry_run {
            let mut cmd = Command::new("git");
            if prune {
                cmd.args(["fetch", "--prune"]);
            } else {
                cmd.args(["fetch"]);
            }
            cmd.spawn()?.wait().expect("git fetch failed");
        }

        Ok(())
    }

    fn run_git_add(&self) {
        Command::new("git")
            .args(&["add", "."])
            .stdout(Stdio::null())
            .current_dir(&self.path.as_path())
            .spawn()
            .expect("git add failed")
            .wait()
            .expect("failed to finish 'git add'");
    }

    fn run_git_commit(&self, version: &str) {
        let mut cmd = Command::new("git");

        cmd.args(["commit", "-a", "-m", &format!("@{} release", version)]);

        if self.no_verify {
            cmd.arg("--no-verify");
        };

        cmd.stdout(Stdio::null())
            .current_dir(&self.path.as_path())
            .spawn()
            .expect("git commit failed")
            .wait()
            .expect("failed to finish 'git commit'");
    }

    fn run_git_tag(&self, version: &str) {
        let mut cmd = Command::new("git");
        cmd.args([
            "tag",
            "-a",
            &format!("@{}", version),
            "-m",
            &format!("Added \"@{} release\" version", version),
        ]);

        cmd.stdout(Stdio::null())
            .current_dir(&self.path.as_path())
            .spawn()
            .expect("git tag failed")
            .wait()
            .expect("Failed to finish 'git tag'");
    }

    fn run_git_push(&self, only_tags: bool, use_force: bool) {
        let mut cmd = Command::new("git");
        cmd.arg("push")
            .stdout(Stdio::null())
            .current_dir(&self.path.as_path());

        if only_tags {
            cmd.arg("--tags");
        }

        if use_force {
            cmd.arg("--force");
        }

        if self.no_verify {
            cmd.arg("--no-verify");
        }

        cmd.spawn()
            .expect("git push failed")
            .wait()
            .expect("failed to finish git push");
    }
}
