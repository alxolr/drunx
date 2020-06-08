use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Git<'a> {
    path: &'a PathBuf,
    dry_run: bool,
}

impl<'a> Git<'a> {
    pub fn new(path: &'a PathBuf, dry_run: bool) -> Git<'a> {
        Git { path, dry_run }
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
        println!("git commit -a -m \"{}\"", version);
        println!(
            "git tag -a \"{}\" -m \"{}\"",
            version,
            format!("Released version 'v{}'", version)
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
        Command::new("git")
            .args(&["commit", "-a", "-m", &format!("v{}", version)])
            .stdout(Stdio::null())
            .current_dir(&self.path.as_path())
            .spawn()
            .expect("git commit failed")
            .wait()
            .expect("failed to finish 'git commit'");
    }

    fn run_git_tag(&self, version: &str) {
        Command::new("git")
            .args(&[
                "tag",
                "-a",
                &format!("v{}", version),
                "-m",
                &format!("Released version 'v{}'", version),
            ])
            .stdout(Stdio::null())
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

        cmd.spawn()
            .expect("git push failed")
            .wait()
            .expect("failed to finish git push");
    }
}
