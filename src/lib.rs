#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::{collections::HashMap, fs, path::Path};
use serde::Deserialize;
use workspaces::get_workspaces_package_json_files;

pub mod workspaces;

#[napi(object)]
#[derive(Deserialize, Debug)]
pub struct PackageInfo {
  pub dependencies: Option<HashMap<String, String>>,
  pub dev_dependencies: Option<HashMap<String, String>>,
  pub name: String,
  pub private: Option<bool>,
  pub peer_dependencies: Option<HashMap<String, String>>,
  pub version: String,
  pub package_json_path: Option<String>,
}

#[napi]
pub fn get_workspaces(cwd: String) -> Vec<PackageInfo> {
  let package_json_files_result = get_workspaces_package_json_files(cwd.as_str());
  let mut workspaces = vec![];

  if let Ok(package_json_files) = package_json_files_result {
    for package_json_file in package_json_files.iter() {
      let contents_result = fs::read_to_string(Path::new(cwd.as_str()).join(package_json_file));
      if let Ok(contents) = contents_result {
        let mut value: PackageInfo = serde_json::from_str(contents.as_str()).unwrap();
        value.package_json_path = Some(package_json_file.clone());

        workspaces.push(value);
      }
    }
  }

  workspaces
}
