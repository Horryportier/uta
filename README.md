# uta

Simple mpv utiliy cil 

# Instalation 
```bash
cargo install --git https://github.com/Horryportier/uta

or 

git clone https://github.com/Horryportier/uta
cd uta 
cargo install --path .

```
# Usage 
run `uta URL/PATH` to start `-t` to pause/unpause `-n/p` to switch `-h` for help `-h [any flag]` help for flgas.
## env variable
- `UTA_VOLUME=50` set default volume 
- `UTA_VIDEO=true` if set to `true` mpv will lunch with video. when not set will default to `false` 
- `UTA_DOWNLAND` use it to set yt-dlp downland args "none are set by default"
# Tmux embading 

```bash
# .tmux.conf

# update the status bar every second
set -g status-interval 1

# show our widget
set -g status-right ' #(uta --print)'

```
