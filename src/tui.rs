use crossterm::{
    event::{poll, read, KeyCode, KeyEvent},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, WindowSize},
    ExecutableCommand,
};
use mpvipc::ipc::PlaylistEntry;
use std::{io::stdout, time::Duration};

use crate::Error;

enum Events {
    Key(KeyEvent),
    Resize(u16, u16),
}

#[derive(Default)]
pub struct Tui {
    pub playlist_data: Option<Vec<PlaylistEntry>>,
    index: usize,
    window: Option<(u16, u16)>,
}

impl Tui {
    pub fn new(entries: Vec<PlaylistEntry>) -> Tui {
        Tui {
            playlist_data: Some(entries),
            ..Default::default()
        }
    }

    pub fn render(&mut self) -> Result<(), Error> {
        stdout().execute(EnterAlternateScreen)?;
        render_playlist_entryies(&self.playlist_data, self.index);

        loop {
            let keycode = match handle_input()? {
                Some(event) => match event {
                    Events::Key(k) => Some(k.code),
                    Events::Resize(c, r) => {
                        self.window = Some((c, r));
                        None
                    }
                },
                None => None,
            };

            if keycode.is_some() {
                match keycode.unwrap() {
                    KeyCode::Esc => {
                        stdout().execute(LeaveAlternateScreen)?;
                        return Ok(());
                    }
                    KeyCode::Up => {}
                    _ => {}
                }
            }
        }
    }

    fn up(&mut self) {
        if self.playlist_len() > self.index {
            self.index += 1;
        }
    }
    fn down(&mut self) {
        if 0 != self.index {
            self.index -= 1;
        }
    }

    fn playlist_len(&self) -> usize {
        if let Some(d) = &self.playlist_data {
            return d.len();
        }
        0
    }
}

fn handle_input() -> Result<Option<Events>, Error> {
    if poll(Duration::from_millis(1000))? {
        let event = read()?;
        let key = match event {
            crossterm::event::Event::Key(k) => Some(Events::Key(k)),
            crossterm::event::Event::Resize(c, r) => Some(Events::Resize(c, r)),
            _ => None,
        };
        return Ok(key);
    }
    Ok(None)
}

fn render_playlist_entryies(entires: &Option<Vec<PlaylistEntry>>, selected: usize) {
    match entires {
        Some(s) => s.iter().for_each(|f| {
            if f.id == selected {
                display_playlist_entry(f, true)
            } else {
                display_playlist_entry(f, false)
            }
        }),
        None => print!("there is no playilst data"),
    }
}

fn display_playlist_entry(entry: &PlaylistEntry, is_selected: bool) {
    match is_selected {
        true => println!("=> [{}] {}", entry.id, entry.filename),
        false => println!("[{}] {}", entry.id, entry.filename),
    }
}
