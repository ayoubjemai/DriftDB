use indicatif::{ProgressBar, ProgressStyle};
use std::{process::Command, time::Duration};

pub struct Spinner {}

impl Spinner {
    fn create_spinner(&self, message: &str) -> ProgressBar {
        let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&spinner_frames)
                .template(&format!("{{spinner}} {}", message))
                .unwrap(),
        );
        spinner.enable_steady_tick(Duration::from_millis(100));
        spinner
    }

    pub fn execute_with_spinner(
        &self,
        command: &mut Command,
        message: &str,
    ) -> std::process::Output {
        let spinner = self.create_spinner(message);

        let result = command.output();

        // Stop the spinner
        spinner.finish_and_clear();

        match result {
            Ok(output) => output,
            Err(err) => panic!("Failed to execute command: {:?}", err),
        }
    }
}
