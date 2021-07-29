use std::error::Error;
use std::process::Command;

pub struct Notification {}
pub enum Icon {
    FaceWink,
}

pub enum Urgency {
    Critical,
    Normal,
    Low,
}

impl Notification {
    pub fn new() -> Self {
        Notification {}
    }

    pub fn send(&self, msg: &str, urgency: Urgency, icon: Icon) -> Result<(), Box<dyn Error>> {
        let urgency = self.get_urgency(urgency);
        let icon = self.get_icon(icon);

        Command::new("notify-send")
            .args(["-u", urgency, "-i", icon, msg])
            .output()?;

        Ok(())
    }

    fn get_icon<'a>(&self, icon: Icon) -> &'a str {
        let icon = match icon {
            Icon::FaceWink => "face-wink",
        };

        icon
    }

    fn get_urgency<'a>(&self, urgency: Urgency) -> &'a str {
        let urgency = match urgency {
            Urgency::Critical => "critical",
            Urgency::Normal => "normal",
            Urgency::Low => "low",
        };

        urgency
    }
}
