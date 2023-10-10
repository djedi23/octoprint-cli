use super::TemperatureData;
use crud_api::{Api, ApiInput};
use crud_pretty_struct::PrettyPrint;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Api, Deserialize, Serialize, Default)] // Prettyprint
#[api(endpoint(
  route = "/api/printer/bed",
//  payload_struct = "PrinterToolCommand",
  cli_route = "/printer/bed",
  cli_help = "Retrieves the current temperature data (actual, target and offset)."
),endpoint(
  route = "/api/printer/bed",
  method = "POST",
  result_ok_status = "NO_CONTENT",
    result_ko_status(
      status = "CONFLICT",
      message = "The printer is not operational or the selected printer profile does not have a heated bed."
    ),
  payload_struct = "PrinterBedCommand",
  cli_route = "/printer/bed/command",
  cli_help = "Bed commands allow setting the temperature and temperature offsets for the printer’s heated bed."
))]
#[derive(PrettyPrint)]
pub(crate) struct PrinterBed {
  #[pretty(is_pretty)]
  bed: TemperatureData,
  // history: VecTemperatureHistory,
}

#[derive(ApiInput, Debug, Deserialize, Serialize)]
#[serde(tag = "command")]
#[serde(rename_all = "lowercase")]
#[api(no_input_file, heading = "COMMANDS")]
pub(crate) enum PrinterBedCommand {
  #[api(
    no_short,
    help = "Sets the given target temperature on the printer’s bed."
  )]
  Target(TargetParameter),
  #[api(
    no_short,
    help = "Sets the given temperature offset on the printer’s bed."
  )]
  Offset(OffsetParameter),
}

impl Default for PrinterBedCommand {
  fn default() -> Self {
    Self::Target(TargetParameter { temperature: 0.0 })
  }
}

#[derive(ApiInput, Debug, Deserialize, Serialize)]
#[api(no_input_file, heading = "TARGET PARAMETERS")]
pub(crate) struct TargetParameter {
  #[api(
    no_short,
    help = "Target temperature to set. A value of 0 will turn the heater off."
  )]
  #[serde(rename = "target")]
  temperature: f32,
}

#[derive(ApiInput, Debug, Deserialize, Serialize)]
#[api(no_input_file, heading = "OFFSET PARAMETER")]
pub(crate) struct OffsetParameter {
  #[api(no_short, help = "Offset(s) to set")]
  offsets: f32,
}
