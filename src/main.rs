use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, *},
};
use ratatui_game_of_life::{Coords, GameOfLife};

struct App {
    board: GameOfLife,
}

impl App {
    fn new(width: usize, height: usize) -> App {
        let starting_coords = vec![
            // Blinker
            Coords { x: 0, y: 1 },
            Coords { x: 1, y: 1 },
            Coords { x: 2, y: 1 },
            //Glider
            Coords { x: 6, y: 0 },
            Coords { x: 6, y: 1 },
            Coords { x: 6, y: 2 },
            Coords { x: 5, y: 2 },
            Coords { x: 4, y: 1 },
            //Pulsar
            Coords { x: 5, y: 13 },
            Coords { x: 5, y: 14 },
            Coords { x: 5, y: 15 },
            Coords { x: 5, y: 19 },
            Coords { x: 5, y: 20 },
            Coords { x: 5, y: 21 },
            Coords { x: 10, y: 13 },
            Coords { x: 10, y: 14 },
            Coords { x: 10, y: 15 },
            Coords { x: 10, y: 19 },
            Coords { x: 10, y: 20 },
            Coords { x: 10, y: 21 },
            Coords { x: 12, y: 13 },
            Coords { x: 12, y: 14 },
            Coords { x: 12, y: 15 },
            Coords { x: 12, y: 19 },
            Coords { x: 12, y: 20 },
            Coords { x: 12, y: 21 },
            Coords { x: 17, y: 13 },
            Coords { x: 17, y: 14 },
            Coords { x: 17, y: 15 },
            Coords { x: 17, y: 19 },
            Coords { x: 17, y: 20 },
            Coords { x: 17, y: 21 },
            Coords { x: 7, y: 11 },
            Coords { x: 8, y: 11 },
            Coords { x: 9, y: 11 },
            Coords { x: 13, y: 11 },
            Coords { x: 14, y: 11 },
            Coords { x: 15, y: 11 },
            Coords { x: 7, y: 16 },
            Coords { x: 8, y: 16 },
            Coords { x: 9, y: 16 },
            Coords { x: 13, y: 16 },
            Coords { x: 14, y: 16 },
            Coords { x: 15, y: 16 },
            Coords { x: 7, y: 18 },
            Coords { x: 8, y: 18 },
            Coords { x: 9, y: 18 },
            Coords { x: 13, y: 18 },
            Coords { x: 14, y: 18 },
            Coords { x: 15, y: 18 },
            Coords { x: 7, y: 23 },
            Coords { x: 8, y: 23 },
            Coords { x: 9, y: 23 },
            Coords { x: 13, y: 23 },
            Coords { x: 14, y: 23 },
            Coords { x: 15, y: 23 },
        ];
        App {
            board: GameOfLife::new(width, height, starting_coords),
        }
    }

    fn on_tick(&mut self) {
        self.board = self.board.next_frame();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(150);
    let width = terminal.get_frame().size().width as usize;
    let height = terminal.get_frame().size().height as usize;

    let app = App::new(width, height);
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.size();

    let block = Block::default().black();
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(size);

    let gameoflife = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("World"))
        .marker(Marker::Block)
        .paint(|ctx| ctx.draw(&app.board))
        .x_bounds([0.0, size.width as f64])
        .y_bounds([0.0, size.height as f64]);
    f.render_widget(gameoflife, chunks[0]);
}
