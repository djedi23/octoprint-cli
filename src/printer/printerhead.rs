use crud_api::{Api, ApiInput};
use crud_pretty_struct::PrettyPrint;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Api, Deserialize, Serialize, Default)] // Prettyprint
#[api(endpoint(
  route = "/api/printer/printhead",
  method = "POST",
  result_ok_status = "NO_CONTENT",
  result_ko_status(
    status = "CONFLICT",
    message = "printer is not operationalor currently printing."
  ),
  result_ko_status(
    status = "BAD_REQUEST",
    message = "Invalid axis specified, invalid value for travel amount for a jog command or factor for feed rate or otherwise invalid request."
  ),
  payload_struct = "PrinterHeadCommand",
  cli_route = "/printer/printhead",
  cli_help = "Print head commands allow jogging and homing the print head in all three axes."
))]
#[derive(PrettyPrint)]
pub(crate) struct PrinterHead {}

#[derive(ApiInput, Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "command")]
#[serde(rename_all = "lowercase")]
#[api(no_input_file, heading = "COMMANDS")]
pub(crate) enum PrinterHeadCommand {
  #[api(
    no_short,
    help = "Jogs the print head (relatively) by a defined amount in one or more axes"
  )]
  Jog(JogParameter),
  #[api(no_short, help = "Homes the print head in all of the given axes.")]
  Home(HomeParameter),
  #[api(
    no_short,
    help = "Changes the feedrate factor to apply to the movements of the axes."
  )]
  Feedrate(FeedrateParameter),
}

impl Default for PrinterHeadCommand {
  fn default() -> Self {
    Self::Home(HomeParameter { axes: None })
  }
}

#[derive(ApiInput, Clone, Debug, Deserialize, Serialize)]
#[api(no_input_file, heading = "JOG PARAMETERS")]
pub(crate) struct JogParameter {
  #[api(
    no_short,
    help = "Amount/coordinate to jog print head on x axis.",
    long_help = "Optional. Amount/coordinate to jog print head on x axis, must be a valid number corresponding to the distance to travel in mm."
  )]
  x: Option<f32>,
  #[api(
    no_short,
    help = "Amount/coordinate to jog print head on y axis.",
    long_help = "Optional. Amount/coordinate to jog print head on y axis, must be a valid number corresponding to the distance to travel in mm."
  )]
  y: Option<f32>,
  #[api(
    no_short,
    help = "Amount/coordinate to jog print head on z axis.",
    long_help = "Optional. Amount/coordinate to jog print head on z axis, must be a valid number corresponding to the distance to travel in mm."
  )]
  z: Option<f32>,
  #[api(
    no_short,
    help = "Whether to move relative to current position or to absolute position",
    long_help = "Optional. Boolean value specifying whether to move relative to current position (provided axes values are relative amounts) or to absolute position (provided axes values are coordinates)"
  )]
  absolute: Option<bool>,
  #[api(
    no_short,
    help = "Speed at which to move.",
    long_help = "Optional. Speed at which to move."
  )]
  speed: Option<f32>,
}

#[derive(ApiInput, Clone, Debug, Deserialize, Serialize)]
#[api(no_input_file, heading = "HOME PARAMETER")]
pub(crate) struct HomeParameter {
  #[api(
    no_short,
    help = "A list of axes which to home.",
    long_help = "A list of axes which to home, valid values are one or more of x, y, z."
  )]
  axes: Option<Vec<String>>,
}

#[derive(ApiInput, Clone, Debug, Deserialize, Serialize)]
#[api(no_input_file, heading = "FEDERATE PARAMETER")]
pub(crate) struct FeedrateParameter {
  #[api(
    no_short,
    help = "The new feedrate factor.",
    long_help = "The new feedrate factor, percentage between 50 and 200% as integer (50 to 200) or float (0.5 to 2.0)."
  )]
  factor: f32,
}
