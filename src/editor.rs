use std::io::{stdout, Stdout, Write};

use anyhow::Ok;
use crossterm::{cursor, event::{self, read}, style::{self, Stylize}, terminal, ExecutableCommand, QueueableCommand};


#[derive(Debug)]
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

pub struct Editor {
    stdout: Stdout,
    current_mode: Mode,
    cx: u16,
    cy: u16,

    window_size: (u16, u16)
}

impl Drop for Editor {
    fn drop(&mut self) {
        _ = self.stdout.flush();

        _ = self.stdout.execute(terminal::LeaveAlternateScreen);
        _ = terminal::disable_raw_mode();
    }
}

impl Editor {
    pub fn new() -> Editor {
        Editor {stdout: stdout(), current_mode: Mode::Command, cx: 0, cy: 0, window_size: (0,0)}
    }

    fn handle_event(&mut self, ev: event::Event) -> anyhow::Result<Option<Action>>{
        match self.current_mode {
            Mode::Command => Self::handle_command_mode_event(self,ev),
            Mode::Insert => Self::handle_insert_mode_event(self, ev),
            Mode::Visual => Self::handle_visual_mode_event(self, ev)
        }
    }
    
    fn handle_command_mode_event(&mut self, ev: event::Event) -> anyhow::Result<Option<Action>> {
        match ev {
            event::Event::Key(event) => match event.code {
                event::KeyCode::Char('q') => {
                    Ok(Some(Action::Quit))
                },
                event::KeyCode::Char('h') | event::KeyCode::Left => Ok(Some(Action::MoveLeft)),
                event::KeyCode::Char('j') | event::KeyCode::Down => Ok(Some(Action::MoveDown)),
                event::KeyCode::Char('k') | event::KeyCode::Up => Ok(Some(Action::MoveUp)),
                event::KeyCode::Char('l') | event::KeyCode::Right => Ok(Some(Action::MoveRight)),
                event::KeyCode::Char('i') => {Ok(Some(Action::EnterMode(Mode::Insert)))},
                event::KeyCode::Char('v') => {Ok(Some(Action::EnterMode(Mode::Visual)))},

                _ => Ok(None)
            },
            _ => Ok(None)
        }
    }
    
    fn handle_insert_mode_event(&mut self, ev: event::Event) -> anyhow::Result<Option<Action>> {
        match ev {
            crossterm::event::Event::Key(event) =>
            match event.code {
                event::KeyCode::Esc => Ok(Some(Action::EnterMode(Mode::Command))),
                event::KeyCode::Char(c) => {
                    let _ = &self.stdout.queue(style::Print(c));
                    self.cx += 1;
                    Ok(None)
                },
                event::KeyCode::Enter => {
                    self.cx = 0;
                    self.cy += 1;
                    Ok(None)
                },
                _ => Ok(None)
            },
            _ => Ok(None)
        }
    }

    fn handle_visual_mode_event(&mut self, ev: event::Event) -> anyhow::Result<Option<Action>> {
        match ev {
            crossterm::event::Event::Key(event) =>
            match event.code {
                event::KeyCode::Esc => Ok(Some(Action::EnterMode(Mode::Command))),
                _ => Ok(None)
            },
            _ => Ok(None)
        }
    }

    fn clear_stylized_status_line(&mut self) {
        _ = self.stdout.flush();
        _ = self.stdout.queue(cursor::MoveTo(0, self.window_size.1-2));
        let clear_status = 
        format!("{}", " "
                    .repeat(self.window_size.0 as usize))
                    .on(style::Color::Rgb{r:225,g:148,b:148});

        _ = self.stdout.execute(style::PrintStyledContent(clear_status));
    }

    fn clear_status_line(&mut self) {
        _ = self.stdout.flush();
        _ = self.stdout.queue(cursor::MoveTo(0, self.window_size.1-2));
        let clear_status = " ".repeat(self.window_size.0 as usize);

        _ = self.stdout.execute(style::Print(clear_status));
    }

    fn draw_status_line(&mut self) -> anyhow::Result<()> {
        
        let separator = "►".to_uppercase();

        self.clear_status_line();

        let status = format!(" {:?} {}", self.current_mode, separator)
        .to_uppercase()
        .with(style::Color::Black)
        .bold()
        .on(style::Color::Rgb{r:225,g:148,b:148});

        self.stdout.queue(cursor::MoveTo(0, self.window_size.1-2))?;
        self.stdout
            .queue(style::PrintStyledContent(status))?;
        
        self.stdout.flush()?;

        Ok(())
    }

    pub fn start(&mut self) -> anyhow::Result<()> {

        self.window_size = terminal::size()?;

        terminal::enable_raw_mode()?;
        self.stdout.execute(terminal::EnterAlternateScreen)?;

        self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;


        loop {
            let _ = self.draw_status_line();
            
            self.stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
            self.stdout.flush()?;

            if let Some(action) = Self::handle_event(self,read()?)? {
                match action {
                    Action::Quit => {
                        
                        
                        return Ok(());
                    },
                    Action::MoveDown => self.cy += 1,
                    Action::MoveUp => {self.cy = self.cy.saturating_sub(1);},
                    Action::MoveLeft => {self.cx = self.cx.saturating_sub(1);},
                    Action::MoveRight => self.cx += 1,
                    Action::EnterMode(m) => {self.current_mode = m}
                }
            }
        }
    }
    
}
