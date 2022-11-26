use anyhow::{anyhow, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

use glob::glob;

fn collect_workspace_array(workspace_array: &Vec<Value>) -> Result<Vec<String>> {
  Ok(
    workspace_array
      .iter()
      .map(|val| val.as_str().unwrap().into())
      .collect(),
  )
}

fn get_workspaces_globs(cwd: &str) -> Result<Vec<String>> {
  let path = Path::new(cwd);
  let path_buf = path.join("package.json");

  let contents = fs::read_to_string(path_buf)?;

  let package_json: Value = serde_json::from_str(&contents)?;

  match &package_json["workspaces"] {
    Value::Array(workspace_array) => collect_workspace_array(&workspace_array),
    Value::Object(workspace_object) => {
      println!("{:?}", workspace_object);
      match workspace_object.get("packages") {
        Some(Value::Array(packages_array)) => collect_workspace_array(&packages_array),
        _ => Err(anyhow!(
          "invalid workspaces configuration as an object in package.json"
        )),
      }
    }
    _ => Err(anyhow!("invalid workspaces configuration in package.json")),
  }
}

pub fn get_workspaces_package_json_files(cwd: &str) -> Result<Vec<String>> {
  let globs = get_workspaces_globs(cwd)?;
  let mut package_json_files: Vec<String> = vec![];

  for pattern in globs.iter() {
    let pattern_with_package_json = format!("{}/{}/{}", cwd, pattern, "package.json");

    if let Ok(paths) = glob(&pattern_with_package_json) {
      for path_result in paths {
        if let Ok(path_buf) = path_result {
          package_json_files.push(
            path_buf
              .strip_prefix(cwd)? // calculate relative paths from cwd
              .to_str()
              .unwrap()
              .into(),
          );
        }
      }
    }
  }

  Ok(package_json_files)
}
