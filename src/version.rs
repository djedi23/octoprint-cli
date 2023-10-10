use crud_api::Api;
use crud_pretty_struct::PrettyPrint;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Api, Deserialize, Serialize, Default)] // Prettyprint
#[api(endpoint(
  route = "/api/version",
  cli_route = "/version",
  cli_help = "Retrieve information regarding server and API version."
))]
#[derive(PrettyPrint)]
pub(crate) struct Version {
  api: String,
  server: String,
  text: String,
}
