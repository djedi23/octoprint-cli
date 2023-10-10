mod bed;
mod printerhead;
mod sd;
mod tool;

pub(crate) use bed::{PrinterBed, PrinterBedCommand};
use crud_api::{Api, ApiInput};
use crud_pretty_struct::{
  formatters::{bool_check_formatter, timestamp_formatter},
  PrettyPrint,
};
use miette::{IntoDiagnostic, Result};
use parse_display::Display;
pub(crate) use printerhead::{PrinterHead, PrinterHeadCommand};
pub(crate) use sd::{PrinterSd, PrinterSdCommand};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
pub(crate) use tool::{PrinterTool, PrinterToolCommand};

#[derive(Debug, Api, Deserialize, Serialize, Default)] // Prettyprint
#[api(endpoint(
  route = "/api/printer",
  query_struct = "PrinterQuery",
  result_ko_status(status = "CONFLICT", message = "printer is not operational"),
  cli_route = "/printer",
  cli_help = "Retrieves the current state of the printer.",
  cli_long_help = "Retrieves the current state of the printer. Returned information includes:
  -  temperature information
  -  sd state
  -  general printer state
Temperature information can also be made to include the printer’s temperature history by supplying the history query parameter. The amount of data points to return here can be limited using the limit query parameter.
Clients can specific a list of attributes to not return in the response (e.g. if they don’t need it) via the exclude query parameter."
))]
#[derive(PrettyPrint)]
pub(crate) struct Printer {
  #[pretty(is_pretty)]
  state: PrinterState,
  #[pretty(is_pretty)]
  sd: SDState,
  #[pretty(is_pretty, label = "🌡️ Temperature")]
  temperature: TemperatureState,
}

#[derive(Debug, Display, Deserialize, Serialize, Default, PrettyPrint)]
#[display("{text}")]
#[allow(non_snake_case)]
struct PrinterState {
  #[pretty(label = "🔥 State")]
  text: String,
  #[pretty(is_pretty)]
  flags: PrinterStateFlags,
}

#[derive(Debug, Deserialize, Serialize, Default, PrettyPrint)]
#[allow(non_snake_case)]
struct PrinterStateFlags {
  #[pretty(formatter=bool_check_formatter)]
  operational: bool,
  #[pretty(formatter=bool_check_formatter)]
  paused: bool,
  #[pretty(formatter=bool_check_formatter)]
  printing: bool,
  #[pretty(formatter=bool_check_formatter)]
  cancelling: bool,
  #[pretty(formatter=bool_check_formatter)]
  pausing: bool,
  #[pretty(formatter=bool_check_formatter)]
  sdReady: bool,
  #[pretty(formatter=bool_check_formatter)]
  error: bool,
  #[pretty(formatter=bool_check_formatter)]
  ready: bool,
  #[pretty(formatter=bool_check_formatter)]
  closedOrError: bool,
}

#[derive(Debug, Display, Deserialize, Serialize, Default, PrettyPrint)]
#[allow(non_snake_case)]
struct SDState {
  #[pretty(formatter=bool_check_formatter)]
  ready: bool,
}

#[derive(Debug, Display, Deserialize, Serialize, Default, PrettyPrint)]
#[display("{tool0}")]
#[allow(non_snake_case)]
struct TemperatureState {
  #[pretty(is_pretty, label = "🪛 Tool 0")]
  tool0: TemperatureData,
  #[pretty(is_pretty, skip_none)]
  tool1: Option<TemperatureData>,
  #[pretty(is_pretty, label = "🛏️ Bed")]
  bed: TemperatureData,
  #[pretty(is_pretty, skip_none)]
  history: Option<Vec<TemperatureHistory>>,
}

fn xxx(x: &dyn ToString, colored: bool) -> Result<(String, bool)> {
  use owo_colors::OwoColorize;
  let temp = x.to_string().parse::<f32>().into_diagnostic()?;
  Ok((
    if colored {
      if temp > 60.0 {
        format!("{}°", temp.red())
      } else if temp > 40.0 {
        format!("{}°", temp.yellow())
      } else {
        format!("{}°", temp.blue())
      }
    } else {
      format!("{:.2}°", temp)
    },
    colored,
  ))
}

#[derive(Debug, Display, Deserialize, Serialize, Default, PrettyPrint)]
#[display("{actual}")]
#[allow(non_snake_case)]
struct TemperatureData {
  #[pretty(formatter=xxx)]
  actual: f32,
  #[pretty(formatter=|x,_|Ok((format!("{:.2}°", x.to_string().parse::<f32>().into_diagnostic()?),false)))]
  target: f32,
  offset: Option<f32>,
}

#[derive(Debug, Display, Deserialize, Serialize, Default, PrettyPrint)]
#[display("{time}")]
#[allow(non_snake_case)]
struct TemperatureHistory {
  #[pretty(formatter=timestamp_formatter)]
  time: u32,
  #[pretty(is_pretty)]
  tool0: TemperatureData,
  #[pretty(is_pretty)]
  tool1: Option<TemperatureData>,
  #[pretty(is_pretty)]
  bed: TemperatureData,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct VecTemperatureHistory(Option<Vec<TemperatureHistory>>);
impl std::fmt::Display for VecTemperatureHistory {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

#[derive(Debug, ApiInput, Serialize, Deserialize, Default)]
#[api(heading = "ARGUMENTS")]
pub(crate) struct PrinterQuery {
  #[api(
    help = "only the last n data points from the printer’s temperature history will be returned.",
    long_help = "If set to an integer (n), only the last n data points from the printer’s temperature history will be returned. Will be ignored if history is not enabled."
  )]
  limit: Option<u32>,
  #[api(
    short = "y",
    help = "include history information in the response too.",
    long_help = "If set to true (or: yes, y, 1), history information will be included in the response too. If no limit parameter is given, all available temperature history data will be returned."
  )]
  history: Option<bool>,
  #[api(
    help = "An optional list of fields to exclude from the response",
    long_help = "An optional comma-separated list of fields to exclude from the response (e.g. if not needed by the client).",
    possible_values = "temperature,sd,state"
  )]
  exclude: Option<String>,
}
