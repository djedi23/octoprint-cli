use crud_api::{Api, ApiInput};
use crud_pretty_struct::{formatters::bool_check_formatter, PrettyPrint};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Api, Deserialize, Serialize, Default)] // Prettyprint
#[api(endpoint(
  route = "/api/printer/sd",
//  payload_struct = "PrinterToolCommand",
  cli_route = "/printer/sd",
  cli_help = "Retrieves the current state of the printer’s SD card."
),endpoint(
  route = "/api/sd",
  method = "POST",
  result_ok_status = "NO_CONTENT",
    result_ko_status(
      status = "CONFLICT",
      message = "Command is issued but the SD card has not been initialized"
    ),
  payload_struct = "PrinterSdCommand",
  cli_route = "/printer/sd/command",
  cli_help = "SD commands allow initialization, refresh and release of the printer’s SD card (if available)."
))]
#[derive(PrettyPrint)]
pub(crate) struct PrinterSd {
  #[pretty(formatter=bool_check_formatter)]
  ready: bool,
}
#[derive(ApiInput, Debug, Deserialize, Serialize)]
#[serde(tag = "command")]
#[serde(rename_all = "lowercase")]
#[api(no_input_file, heading = "COMMANDS")]
pub(crate) enum PrinterSdCommand {
  #[api(
    no_short,
    help = "Initializes the printer’s SD card, making it available for use. This also includes an initial retrieval of the list of files currently stored on the SD card, so after issuing that command a retrieval of the files on SD card will return a successful result."
  )]
  Init,
  #[api(
    no_short,
    help = "Refreshes the list of files stored on the printer’s SD card."
  )]
  Refresh,
  #[api(
    no_short,
    help = "Releases the SD card from the printer. The reverse operation to init. After issuing this command, the SD card won’t be available anymore, hence and operations targeting files stored on it will fail."
  )]
  Release,
}

impl Default for PrinterSdCommand {
  fn default() -> Self {
    Self::Refresh
  }
}
