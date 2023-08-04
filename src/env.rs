pub fn get_env_mpv_args() -> Vec<String> {
    let mut args = Vec::new();

    args.push(match option_env!("UTA_VOLUME") {
        None => "--volume=50".into(),
        Some(v) => format!("--volume={v}"),
    });

    args.push(match option_env!("UTA_VIDEO") {
       None => "--no-video".into(),
       Some(v) => match v {
           "true" => "",
           "false" => "--no-video",
           _ => "",
       }.into()
    });

    args
}
