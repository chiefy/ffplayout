use std::process::Command;

use simplelog::*;

use crate::utils::get_data_map;
use ffplayout_lib::utils::{config::PlayoutConfig, Media};

pub fn run(config: PlayoutConfig, node: Media, server_running: bool) {
    let obj = serde_json::to_string(&get_data_map(&config, node, server_running)).unwrap();
    trace!("Run task: {obj}");

    if let Err(e) = Command::new(config.task.path).arg(obj).spawn() {
        error!("Couldn't spawn task runner: {e}");
    };
}
