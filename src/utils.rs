pub fn print_help() {
    let text = r#"
            UTA
____________________________

uta is an simple mvp cli wrraper for esier use  

# ARGS

!make sure to put link as first arg
uta [link to youtbe vido/playlist or path to folder/file] --start

"-h" | "--help" =>  print help,
"--start" =>  start mpv with socket at `/tmp/mpvsocket`
"-k" | "--kill" => kill mpv 
"-n" | "--next" => play next video 
"-p" | "--prev" => play prev video
"-t" | "--toogle" => pause/unpause
"-r" | "--rand" => play random entry in play list
"-l" | "--loop" => loop single video 
"--loop_playlist"  => loop playlist
"--print" =>  prinst  in format [0/100% | video title]
"-s" | "-seek" =>  jump to moment in video by procentage 0..100 
"#;

    println!("{text}")
}
