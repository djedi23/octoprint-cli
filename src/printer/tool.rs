use super::TemperatureData;
use crud_api::{Api, ApiInput};
use crud_pretty_struct::PrettyPrint;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Api, Deserialize, Serialize, Default)] // Prettyprint
#[api(
  endpoint(
    route = "/api/printer/tool",
    cli_route = "/printer/tool",
    cli_help = "Query the tool temperature data and also include the temperature history but limit it to two entries.",
  ),
  endpoint(
    route = "/api/printer/tool",
    method = "POST",
    result_ok_status = "NO_CONTENT",
    result_ko_status(
      status = "CONFLICT",
      message = "printer is not operational or currently printing."
    ),
    result_ko_status(
      status = "BAD_REQUEST",
      message = "Invalid axis specified, invalid value for travel amount for a jog command or factor for feed rate or otherwise invalid request."
    ),
    payload_struct = "PrinterToolCommand",
    cli_route = "/printer/tool/command",
    cli_no_output,
    cli_help = "Tool commands allow setting the temperature and temperature offsets for the printer’s tools (hotends).",
    cli_output_formats = "",
    cli_long_help = "Tool commands allow setting the temperature and temperature offsets for the printer’s tools (hotends), selecting the current tool and extruding/retracting from the currently selected tool."
  )
)]
#[derive(PrettyPrint)]
pub(crate) struct PrinterTool {
  #[pretty(is_pretty)]
  tool0: TemperatureData,
  //  tool1: Option<TemperatureData>,
  // history: VecTemperatureHistory,
}

#[derive(ApiInput, Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "command")]
#[serde(rename_all = "lowercase")]
#[api(no_input_file, heading = "COMMANDS")]
pub(crate) enum PrinterToolCommand {
  #[api(
    no_short,
    help = "Sets the given target temperature on the printer’s tools."
  )]
  Target(TargetParameter),
  #[api(
    no_short,
    help = "Sets the given temperature offset on the printer’s tools."
  )]
  Offset(OffsetParameter),
  #[api(no_short, help = "Selects the printer’s current tool.")]
  Select(SelectParameter),
  #[api(
    no_short,
    help = "Extrudes the given amount of filament from the currently selected tool."
  )]
  Extrude(ExtrudeParameter),
  #[api(
    no_short,
    help = "Changes the flow rate factor to apply to extrusion of the tool."
  )]
  Flowrate(FlowrateParameter),
}

impl Default for PrinterToolCommand {
  fn default() -> Self {
    Self::Target(TargetParameter {
      targets: TargetTool {
        tool0: Some(0.0),
        //      tool1: Some(0.0),
      },
    })
  }
}

#[derive(ApiInput, Clone, Debug, Deserialize, Serialize)]
#[api(no_input_file, heading = "TARGET PARAMETERS")]
pub(crate) struct TargetTool {
  #[api(no_short)]
  tool0: Option<f32>,
  // #[api(no_short)]
  // tool1: Option<f32>,
}

#[derive(ApiInput, Clone, Debug, Deserialize, Serialize)]
#[api(no_input_file, heading = "TARGET PARAMETERS")]
pub(crate) struct TargetParameter {
  #[api(
    no_short,
    help = "Target temperature(s) to set.",
    long_help = "Target temperature(s) to set. A value of 0 will turn the heater off."
  )]
  targets: TargetTool,
}

#[derive(ApiInput, Clone, Debug, Deserialize, Serialize)]
#[api(no_input_file, heading = "OFFSET PARAMETER")]
pub(crate) struct OffsetParameter {
  #[api(
    no_short,
    help = "Offset(s) to set",
    long_help = "Offset(s) to set, properties must match the format tool{n} with n being the tool’s index starting with 0."
  )]
  offsets: TargetTool,
}

#[derive(ApiInput, Clone, Debug, Deserialize, Serialize)]
#[api(no_input_file, heading = "SELECT PARAMETER")]
pub(crate) struct SelectParameter {
  #[api(
    no_short,
    help = "Tool to select",
    long_help = "Tool to select, format tool{n} with n being the tool’s index starting with 0."
  )]
  tool: String,
}

#[derive(ApiInput, Clone, Debug, Deserialize, Serialize)]
#[api(no_input_file, heading = "EXTRUDE PARAMETERS")]
pub(crate) struct ExtrudeParameter {
  #[api(
    no_short,
    help = "The amount of filament to extrude in mm.",
    long_help = "The amount of filament to extrude in mm. May be negative to retract."
  )]
  amount: f32,
  #[api(
    no_short,
    help = "Speed at which to extrude.",
    long_help = "Optional. Speed at which to extrude. If not provided, maximum speed for E axis from printer profile will be used. Otherwise interpreted as an integer signifying the speed in mm/min, to append to the command."
  )]
  speed: Option<f32>,
}

#[derive(ApiInput, Clone, Debug, Deserialize, Serialize)]
#[api(no_input_file, heading = "FLOWRATE PARAMETER")]
pub(crate) struct FlowrateParameter {
  #[api(
    no_short,
    help = "The new flowrate factor.",
    long_help = "The new flowrate factor, percentage between 75 and 125% as integer (75 to 125) or float (0.75 to 1.25)."
  )]
  factor: f32,
}
