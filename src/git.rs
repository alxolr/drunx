use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Git<'a, 'b> {
    path: &'a PathBuf,
    version: &'b str,
    dry_run: bool,
}

impl<'a, 'b> Git<'a, 'b> {
    pub fn new(path: &'a PathBuf, version: &'b str, dry_run: bool) -> Git<'a, 'b> {
        Git {
            path,
            version,
            dry_run,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("git add . ");
        if !self.dry_run {
            self.run_git_add();
        }

        println!("git commit -a -m \"{}\"", &self.version);
        if !self.dry_run {
            self.run_git_commit();
        }

        println!(
            "git tag -a \"{}\" -m \"{}\"",
            &self.version,
            format!("Released version 'v{}'", &self.version)
        );

        if !self.dry_run {
            self.run_git_tag();
        }

        println!("git push");
        if !self.dry_run {
            self.run_git_push(false);
        }

        println!("git push --tags");
        if !self.dry_run {
            self.run_git_push(true);
        }

        Ok(())
    }

    fn run_git_add(&self) {
        Command::new("git")
            .args(&["add", "."])
            .stdout(Stdio::null())
            .current_dir(&self.path.as_path())
            .spawn()
            .expect("git add failed");
    }

    fn run_git_commit(&self) {
        Command::new("git")
            .args(&["commit", "-a", "-m", &format!("v{}", &self.version)])
            .stdout(Stdio::null())
            .current_dir(&self.path.as_path())
            .spawn()
            .expect("git commit failed");
    }

    fn run_git_tag(&self) {
        Command::new("git")
            .args(&[
                "tag",
                "-a",
                &format!("v{}", &self.version),
                "-m",
                &format!("Released version 'v{}'", &self.version),
            ])
            .stdout(Stdio::null())
            .current_dir(&self.path.as_path())
            .spawn()
            .expect("git tag failed");
    }

    fn run_git_push(&self, only_tags: bool) {
        let mut cmd = Command::new("git");

        cmd.arg("push")
            .stdout(Stdio::null())
            .current_dir(&self.path.as_path());

        if only_tags {
            cmd.arg("--tags");
        }

        cmd.spawn().expect("git push failed");
    }
}
