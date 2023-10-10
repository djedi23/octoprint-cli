use crud_api::{Api, ApiInput};
use crud_pretty_struct::{
  formatters::{byte_formatter, duration_formatter, timestamp_formatter},
  PrettyPrint,
};
use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Debug, Api, Deserialize, Serialize, Default)] // PrettyPrint
#[api(
  endpoint(
    route = "/api/job",
    cli_route = "/job",
    cli_help = "Retrieve information about the current job (if there is one)."
  ),
  endpoint(
    route = "/api/job",
    method = "POST",
    result_ok_status = "NO_CONTENT",
    result_struct = "EmptyResponse",
    payload_struct = "JobCommand",
    cli_route = "/job/command",
    cli_help = "Send a job command. Default: start"
  )
)]
#[derive(PrettyPrint)]
pub(crate) struct Job {
  //  estimatedPrintTime: Optionu32,
  state: String,
  #[pretty(is_pretty)]
  job: JobInformation,
  #[pretty(is_pretty)]
  progress: Progress,
}

#[derive(Debug, Deserialize, Serialize, Default, PrettyPrint)]
#[allow(non_snake_case)]
struct JobInformation {
  #[pretty(skip_none, formatter=duration_formatter)]
  averagePrintTime: Option<f32>,
  #[pretty(skip_none, formatter=duration_formatter)]
  estimatedPrintTime: Option<f32>,
  #[pretty(is_pretty, skip_none)]
  file: Option<FileInformation>,
}

impl Display for JobInformation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(file) = &self.file {
      f.write_fmt(format_args!(
        "{} [{}]",
        file.display.as_ref().unwrap_or(&String::new()),
        file.size.unwrap_or_default()
      ))
    } else {
      f.write_str("--")
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Default, PrettyPrint)] // skip_none formatter
struct FileInformation {
  #[pretty(skip_none,formatter=timestamp_formatter)]
  date: Option<u32>,
  #[pretty(skip_none)]
  display: Option<String>,
  #[pretty(skip)]
  name: Option<String>,
  #[pretty(skip_none)]
  origin: Option<String>,
  #[pretty(skip)]
  path: Option<String>,
  #[pretty(skip_none, formatter=byte_formatter)]
  size: Option<u32>,
}
#[derive(Debug, Deserialize, Serialize, Default, PrettyPrint)]
#[allow(non_snake_case)]
struct Progress {
  #[pretty(skip_none, label="🚀 Completion", formatter=|x,_|Ok((format!("{:.2}%", x.to_string().parse::<f32>().into_diagnostic()?),false)))]
  completion: Option<f32>,
  #[pretty(skip_none, label="   File Position", formatter=byte_formatter)]
  filepos: Option<u32>,
  #[pretty(skip_none, label="⏳ Print Time", formatter=duration_formatter)]
  printTime: Option<u32>,
  #[pretty(skip_none, label="⌛ Print Time Left", formatter=duration_formatter)]
  printTimeLeft: Option<i32>, // Time left can be late.
  #[pretty(skip_none, label = "   Print Time Left Origin")]
  printTimeLeftOrigin: Option<String>,
}

impl Display for Progress {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(completion) = self.completion {
      f.write_fmt(format_args!("{}%", completion))
    } else {
      f.write_str("--")
    }
  }
}

#[derive(ApiInput, Debug, Deserialize, Serialize)]
#[serde(tag = "command")]
#[serde(rename_all = "lowercase")]
#[api(no_input_file)]
pub(crate) enum JobCommand {
  #[api(
    heading = "COMMAND",
    help = "Starts the print of the currently selected file.",
    long_help = "Starts the print of the currently selected file. For selecting a file, see Issue a file command. If a print job is already active, a 409 Conflict will be returned."
  )]
  Start,
  #[api(
    heading = "COMMAND",
    help = "Cancels the current print job.",
    long_help = "Cancels the current print job. If no print job is active (either paused or printing), a 409 Conflict will be returned."
  )]
  Cancel,
  #[api(
    heading = "COMMAND",
    help = "Restart the print of the currently selected file from the beginning.",
    long_help = "Restart the print of the currently selected file from the beginning. There must be an active print job for this to work and the print job must currently be paused. If either is not the case, a 409 Conflict will be returned.

    Equivalent to issuing a `cancel` command while paused, directly followed by a `start` command.
"
  )]
  Restart,
  #[api(
    heading = "COMMAND",
    help = "Pauses/resumes/toggles the current print job.",
    long_help = "Pauses/resumes/toggles the current print job. Accepts one optional additional parameter action specifying which action to take. "
  )]
  Pause(PauseAction),
}

impl Default for JobCommand {
  fn default() -> Self {
    Self::Start
  }
}

#[derive(ApiInput, Debug, Deserialize, Serialize)]
#[serde(tag = "action")]
#[serde(rename_all = "lowercase")]
#[api(no_input_file)]
pub(crate) enum PauseAction {
  #[api(no_short, long = "pause")]
  #[api(
    heading = "PAUSE ACTION",
    help = "Pauses the current job if it’s printing, does nothing if it’s already paused."
  )]
  Pause,
  #[api(no_short)]
  #[api(
    heading = "PAUSE ACTION",
    help = "Resumes the current job if it’s paused, does nothing if it’s printing."
  )]
  Resume,
  #[api(no_short)]
  #[api(
    heading = "PAUSE ACTION",
    help = "Toggles the pause state of the job, pausing it if it’s printing and resuming it if it’s currently paused."
  )]
  Toggle,
}

impl Default for PauseAction {
  fn default() -> Self {
    Self::Toggle
  }
}
