use std::io::{stdout, Cursor, Stdout, Write};

use anyhow::Ok;
use crossterm::{cursor, event::{self, read}, queue, style::{self, style}, terminal, ExecutableCommand, QueueableCommand};

enum Mode {
    Command,
    Insert, 
    Visual
}

enum Action {
    Quit,

    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,

    EnterMode(Mode)
}

fn handle_event(mode: &Mode, ev: event::Event, _stdout: &mut Stdout, cx: &mut u16, cy: &mut u16) -> anyhow::Result<Option<Action>>{
    match mode {
        Mode::Command => handle_command_mode_event(ev),
        Mode::Insert => handle_insert_mode_event(ev, _stdout, cx, cy),
        Mode::Visual => handle_visual_mode_event(ev)
    }
}


fn handle_command_mode_event(ev: event::Event) -> anyhow::Result<Option<Action>> {
    //unimplemented!("Command Event: {ev:?}");

    match ev {
        event::Event::Key(event) => match event.code {
            event::KeyCode::Char('q') => Ok(Some(Action::Quit)),
            event::KeyCode::Char('h') | event::KeyCode::Left => Ok(Some(Action::MoveLeft)),
            event::KeyCode::Char('j') | event::KeyCode::Down => Ok(Some(Action::MoveDown)),
            event::KeyCode::Char('k') | event::KeyCode::Up => Ok(Some(Action::MoveUp)),
            event::KeyCode::Char('l') | event::KeyCode::Right => Ok(Some(Action::MoveRight)),
            event::KeyCode::Char('i') => {Ok(Some(Action::EnterMode(Mode::Insert)))},
            
            _ => Ok(None),
        },
        _ => Ok(None)
    }
}
fn handle_insert_mode_event(ev: event::Event, _stdout: &mut Stdout, cx: &mut u16, cy: &mut u16) -> anyhow::Result<Option<Action>> {
    match ev {
        crossterm::event::Event::Key(event) =>
        match event.code {
            event::KeyCode::Esc => Ok(Some(Action::EnterMode(Mode::Command))),
            event::KeyCode::Char(c) => {
                let _ = _stdout.queue(style::Print(c));
                *cx += 1;
                Ok(None)
            },
            event::KeyCode::Enter => {
                *cx = 0;
                *cy += 1;
                Ok(None)
            },
            _ => Ok(None)
        },
        _ => Ok(None)
    }
}
fn handle_visual_mode_event(ev: event::Event) -> anyhow::Result<Option<Action>> {
    unimplemented!("Visual Event: {ev:?}");
}


fn main() -> anyhow::Result<()> {
    let mut stdout: std::io::Stdout = stdout();

    let mut cx: u16 = 0;
    let mut cy: u16 = 0;

    let mut current_mode: Mode = Mode::Command;

    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;


    loop {
        stdout.queue(cursor::MoveTo(cx, cy))?;
        stdout.flush()?;

        if let Some(action) = handle_event(&current_mode, read()?, &mut stdout, &mut cx, &mut cy)? {
            match action {
                Action::Quit => break,
                Action::MoveDown => cy += 1,
                Action::MoveUp => {cy = cy.saturating_sub(1);},
                Action::MoveLeft => {cx = cx.saturating_sub(1);},
                Action::MoveRight => cx += 1,
                Action::EnterMode(m) => {current_mode = m}
            }
        }
        

    }


    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}


