use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, queue, terminal};
use std::io::{self, Write, stdout};
use std::time::Duration;

const VERSION: &str = "0.1.0";

struct CleanUp;
struct Reader;
struct Editor {
    reader: Reader,
    output: Output,
}
struct Output {
    win_size: (usize, usize),
    editor_contents: EditorContents,
}
struct EditorContents {
    content: String,
}

impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        Output::clear_screen().expect("Error");
    }
}

impl Output {
    fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            editor_contents: EditorContents::new(),
        }
    }

    fn clear_screen() -> std::io::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0; // add this line
        for i in 0..screen_rows {
            // add the following
            if i == screen_rows / 3 {
                let mut welcome = format!("Pound Editor --- Version {}", VERSION);
                if welcome.len() > screen_columns {
                    welcome.truncate(screen_columns)
                }
                self.editor_contents.push_str(&welcome);
            } else {
                self.editor_contents.push('~');
            }
            /* end */
            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            )
            .unwrap();
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n");
            }
        }
    }

    fn refresh_screen(&mut self) -> std::io::Result<()> {
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        queue!(self.editor_contents, cursor::MoveTo(0, 0), cursor::Show)?;
        self.editor_contents.flush()
    }
}

impl Reader {
    fn read_key(&self) -> std::io::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    fn process_keypress(&self) -> std::io::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    fn run(&mut self) -> std::io::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}

fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
