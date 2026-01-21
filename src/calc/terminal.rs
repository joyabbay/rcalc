use super::Calc;
use anyhow::Context;
use crossterm::{
    cursor::{self, Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{Clear, ClearType, ScrollDown},
};
use std::{
    fmt::Display,
    io::{Write, stdout},
};

fn clear_screen() -> anyhow::Result<()> {
    queue!(stdout(), Clear(ClearType::All)).context("Unable to clear the screen")?;
    Ok(())
}

fn execute_queue() -> anyhow::Result<()> {
    stdout().flush().context("Unable to flush the buffer")?;
    Ok(())
}
fn hide_cursor() -> anyhow::Result<()> {
    queue!(stdout(), Hide).context("Unable to hide cursor")?;
    Ok(())
}
fn show_cursor() -> anyhow::Result<()> {
    queue!(stdout(), Show).context("Unable to restore cursor")?;
    Ok(())
}
pub fn print_newline(calc: &mut Calc) -> anyhow::Result<()> {
    let height = crossterm::terminal::size()?.1;
    let cursor_pos = cursor::position()?.1;
    if cursor_pos == height.saturating_sub(1) {
        calc.top_element_offset += 1;
    }
    queue!(stdout(), Print("\r\n")).context("Unable to print")?;
    execute_queue()?;
    Ok(())
}
pub fn dual_clear(calc: &mut Calc) -> anyhow::Result<()> {
    let cursor_pos = cursor::position()?.1 - 2;
    queue!(stdout(), cursor::MoveToPreviousLine(2)).context("Unable to move up")?;
    queue!(stdout(), Clear(ClearType::FromCursorDown)).context("Unable to clear line")?;
    if calc.top_element_offset > 0 {
        calc.top_element_offset -= 1;
        if let Some(top_element) = calc.num_stack.get_nth(calc.top_element_offset) {
            queue!(stdout(), ScrollDown(1))?;
            queue!(stdout(), MoveTo(0, 0))?;
            queue!(stdout(), Print(top_element))?;
            queue!(stdout(), MoveTo(0, cursor_pos))?;
        }
    }
    execute_queue()?;
    Ok(())
}
pub fn print_char<T: Display>(c: T) -> anyhow::Result<()> {
    queue!(stdout(), Print(c)).context("Unable to print")?;
    execute_queue()?;
    Ok(())
}
pub fn print_backspace() -> anyhow::Result<()> {
    queue!(stdout(), cursor::MoveLeft(1)).context("Unable to move cursor")?;
    queue!(stdout(), Print(' ')).context("Unable to print")?;
    queue!(stdout(), cursor::MoveLeft(1)).context("Unable to move cursor")?;
    execute_queue()?;
    Ok(())
}
pub fn initialize() -> anyhow::Result<()> {
    hide_cursor()?;
    queue!(stdout(), cursor::MoveTo(0, 0)).context("Unable to move cursor ")?;
    clear_screen()?;
    crossterm::terminal::enable_raw_mode().context("Unable to enter raw mode")?;
    execute_queue()?;
    Ok(())
}

pub fn terminate() -> anyhow::Result<()> {
    clear_screen()?;
    show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;
    execute_queue()?;
    Ok(())
}
