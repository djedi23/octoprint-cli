use crud_api::Api;
use crud_pretty_struct::PrettyPrint;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Api, Deserialize, Serialize, Default)] // Prettyprint
#[api(
  endpoint(
    route = "/api/system",
    result_struct = "EmptyResponse",
    result_ok_status = "METHOD_NOT_ALLOWED",
    cli_no_output,
    cli_route = "system",
    cli_help = "System commands"
  ),
  endpoint(
    route = "/api/system/commands",
    cli_route = "system/commands",
    cli_help = "Retrieves all configured system commands"
  ),
  endpoint(
    route = "/api/system/commands/core/{action}",
    method = "POST",
    result_ok_status = "NO_CONTENT",
    result_struct = "EmptyResponse",
    cli_no_output,
    cli_route = "system/commands/{action}",
    cli_help = "Execute the system command"
  )
)]
#[derive(PrettyPrint)]
pub(crate) struct SystemCommand {
  #[pretty(is_pretty)]
  #[api(table_skip)]
  core: Vec<Command>,
  #[pretty(is_pretty)]
  #[api(table_skip)]
  custom: Vec<Command>,
}

#[derive(Debug, Deserialize, Serialize, PrettyPrint)]
pub(crate) struct Command {
  action: String,
  name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  confirm: Option<String>,
  source: String,
  resource: String,
}
