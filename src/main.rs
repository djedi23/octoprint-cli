mod job;
mod printer;
mod system;
mod version;

use crate::{
  job::{Job, JobCommand},
  printer::{
    Printer, PrinterBed, PrinterBedCommand, PrinterHead, PrinterHeadCommand, PrinterQuery, PrinterSd,
    PrinterSdCommand, PrinterTool, PrinterToolCommand,
  },
  system::SystemCommand,
  version::Version,
};
use crud_api::{Api, ApiInput, ApiRun, EmptyResponse, Query};
use crud_auth::CrudAuth;
use crud_auth_bearer::Auth;
use miette::Result;
use std::fmt::Debug;

#[derive(Debug, ApiRun)]
#[api(infos(
  base_url = "http://octopi.local",
  name = "octoprint-cli",
  qualifier = "org",
  organisation = "djedi",
  env_prefix = "OP",
  author = "Moise Valvassori<moise.valvassori@gmail.com>",
  about = "Octoprint API in CLI",
))]
#[allow(dead_code)]
struct Apis {}

#[tokio::main]
async fn main() -> Result<()> {
  Apis::run().await?;
  Ok(())
}
