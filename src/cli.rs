use crossterm::{
    event::{self, KeyCode, KeyEvent, MouseEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Stylize, Terminal},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    CompletedFrame,
};
use std::{
    io::stdout,
    time::{Duration, Instant},
};

use crate::{
    grid::Grid,
    seed::{Oscillator, Seed, Spaceship, Still},
};

const FRAMETIME_MILIS: u64 = 16; // 60 fps
const TITLE: &str = "Conway's Game of Life";
const INSTRUCTIONS: &str = concat!(
    "\n",
    r#"Esc or Q (quit) | 0-9 A-F (select seed) | "#,
    r#"Arrows (move seed) | Shift+Arrows (move faster) | "#,
    r#"Space (place seed) | P (play or pause) | Enter (tick) | Delete (clear)"#
);

#[derive(Debug)]
struct State {
    play: PlayState,
    origin: (usize, usize),
    seed_index: u8,
    last_update: Instant,
    target_framerate: u64,
    game: Grid,
}

impl Default for State {
    fn default() -> Self {
        State {
            seed_index: 0,
            origin: (0, 0),
            target_framerate: 60,
            last_update: Instant::now(),
            play: PlayState::Paused,
            game: Grid::new(0, 0),
        }
    }
}

#[derive(Debug, Default)]
enum PlayState {
    #[default]
    Paused,
    Playing,
}

struct ExitSignal(bool);

pub fn run() -> std::io::Result<()> {
    let mut terminal = setup()?;
    let size = terminal.size()?;
    let width = size.width as usize;
    let height = size.height as usize;

    let mut state = State {
        // a cell's char width is 2 chars
        game: Grid::new(width / 2, height / 2),
        // place the cursor at the center of the screen
        origin: (width / 4, height / 2 - (height / 15)),
        ..Default::default()
    };

    loop {
        draw(&mut terminal, &mut state)?;
        let ExitSignal(should_exit) = handle_input(&mut state)?;

        if should_exit {
            break;
        }
    }

    teardown()
}

#[inline]
fn setup() -> std::io::Result<Terminal<CrosstermBackend<impl std::io::Write>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    Ok(terminal)
}

#[inline]
fn draw<'t>(
    terminal: &'t mut Terminal<CrosstermBackend<impl std::io::Write>>,
    state: &mut State,
) -> std::io::Result<CompletedFrame<'t>> {
    let game = &mut state.game;

    terminal.draw(|frame| {
        let area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ])
            .split(frame.size());

        let block = Block::default()
            .title(TITLE)
            .borders(Borders::BOTTOM)
            .title_style(Style::default().add_modifier(Modifier::BOLD))
            .title_alignment(Alignment::Center)
            .bg(Color::Blue)
            .bold();

        frame.render_widget(block, area[0]);

        game.resize(area[1].width as usize, area[1].height as usize);

        match state.play {
            PlayState::Playing => {
                let now = Instant::now();
                let frametime = Duration::from_secs_f64(state.target_framerate as f64 / 1000.0);
                match frametime.checked_sub(state.last_update.elapsed()) {
                    None => {
                        game.tick();
                        state.last_update = now;
                    }
                    Some(_) => {}
                };
            }
            _ => {
                game.preview(select_seed(state.seed_index), state.origin);
            }
        }

        frame.render_widget(Paragraph::new(format!("{}", game)).white(), area[1]);

        frame.render_widget(
            Paragraph::new(INSTRUCTIONS)
                .black()
                .on_gray()
                .bold()
                .alignment(Alignment::Center),
            area[2],
        );
    })
}

#[inline]
fn handle_input(state: &mut State) -> std::io::Result<ExitSignal> {
    if event::poll(std::time::Duration::from_millis(FRAMETIME_MILIS))? {
        let game = &mut state.game;
        match event::read()? {
            //
            //
            event::Event::Mouse(MouseEvent {
                kind,
                row,
                column,
                modifiers: _,
            }) => match kind {
                event::MouseEventKind::Down(_) => {
                    game.seed(
                        select_seed(state.seed_index),
                        (row as usize, column as usize),
                    );
                }
                event::MouseEventKind::ScrollDown => {
                    next_seed(state);
                }
                event::MouseEventKind::ScrollUp => {
                    previous_seed(state);
                }
                event::MouseEventKind::Moved => {
                    game.preview(
                        select_seed(state.seed_index),
                        (row as usize, column as usize),
                    );
                }
                _ => {}
            },
            //
            //
            event::Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                state: _,
            }) => {
                let speed = match modifiers {
                    event::KeyModifiers::SHIFT => 5,
                    _ => 1,
                };

                if kind == event::KeyEventKind::Press {
                    match code {
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                            return Ok(ExitSignal(true))
                        }
                        KeyCode::Pause | KeyCode::Char('p') | KeyCode::Char('P') => {
                            match state.play {
                                PlayState::Paused => {
                                    state.play = PlayState::Playing;
                                }
                                PlayState::Playing => {
                                    state.play = PlayState::Paused;
                                    game.preview(select_seed(state.seed_index), state.origin);
                                }
                            }
                        }
                        KeyCode::Insert | KeyCode::Char(' ') => {
                            game.seed(select_seed(state.seed_index), state.origin);
                        }
                        KeyCode::Left => {
                            state.origin.0 = state.origin.0.saturating_sub(speed);
                            game.preview(select_seed(state.seed_index), state.origin);
                        }
                        KeyCode::Right => {
                            if state.origin.0 + speed <= game.width {
                                state.origin.0 += speed;
                            }
                            game.preview(select_seed(state.seed_index), state.origin);
                        }
                        KeyCode::Up => {
                            state.origin.1 = state.origin.1.saturating_sub(speed);
                            game.preview(select_seed(state.seed_index), state.origin);
                        }
                        KeyCode::Down => {
                            if state.origin.1 + speed <= game.height {
                                state.origin.1 += speed;
                            }
                            game.preview(select_seed(state.seed_index), state.origin);
                        }
                        KeyCode::Delete => {
                            game.clear();
                        }
                        KeyCode::Enter => match state.play {
                            PlayState::Paused => {
                                game.tick();
                            }
                            PlayState::Playing => {
                                state.play = PlayState::Paused;
                                game.preview(select_seed(state.seed_index), state.origin);
                            }
                        },
                        KeyCode::Char(ch) => {
                            if ch.is_digit(16) {
                                state.seed_index = ch.to_digit(16).unwrap() as u8;
                            }
                            game.preview(select_seed(state.seed_index), state.origin);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(ExitSignal(false))
}

const MAX_SEEDS: u8 = 14;

fn next_seed(state: &mut State) {
    match state.seed_index {
        MAX_SEEDS => state.seed_index = 0,
        _ => state.seed_index += 1,
    }
}

fn previous_seed(state: &mut State) {
    match state.seed_index {
        0 => state.seed_index = MAX_SEEDS,
        _ => state.seed_index -= 1,
    }
}

fn select_seed(index: u8) -> Seed {
    match index {
        // Still lifes are patterns that do not change from one generation to the next.
        1 => Seed::Still(Still::Block),
        2 => Seed::Still(Still::Beehive),
        3 => Seed::Still(Still::Loaf),
        4 => Seed::Still(Still::Boat),
        5 => Seed::Still(Still::Tub),

        // Oscillators are patterns that return to their original configuration
        6 => Seed::Oscillator(Oscillator::Blinker),
        7 => Seed::Oscillator(Oscillator::Toad),
        8 => Seed::Oscillator(Oscillator::Beacon),
        9 => Seed::Oscillator(Oscillator::Pulsar),
        10 => Seed::Oscillator(Oscillator::PentaDecathlon),

        // Spaceships are patterns that translate themselves across the grid.
        11 => Seed::Spaceship(Spaceship::Glider),
        12 => Seed::Spaceship(Spaceship::LwSpaceship),
        13 => Seed::Spaceship(Spaceship::MwSpaceship),
        14 => Seed::Spaceship(Spaceship::HwSpaceship),

        // A single cell.
        _ => Seed::Cell((0, 0)),
    }
}

#[inline]
fn teardown() -> std::io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
