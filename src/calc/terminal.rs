
use crossterm::{cursor::{self, Hide, Show}, queue, style::Print, terminal::{Clear, ClearType}};
use std::{fmt::Display, io::{Write, stdout}};
use anyhow::Context;

fn clear_screen() -> anyhow::Result<()>{
    queue!(stdout(),Clear(ClearType::All)).context("Unable to clear the screen")?;
    Ok(())
}

fn execute_queue() -> anyhow::Result<()>{
    stdout().flush().context("Unable to flush the buffer")?;
    Ok(())
}
fn hide_cursor() -> anyhow::Result<()>{
    queue!(stdout(),Hide).context("Unable to hide cursor")?;
    Ok(())
}
fn show_cursor() -> anyhow::Result<()>{
    queue!(stdout(),Show).context("Unable to restore cursor")?;
    Ok(())
}
pub fn print_newline() -> anyhow::Result<()>{

    queue!(stdout(),Print("\r\n")).context("Unable to print")?;
    execute_queue()?;
    Ok(())
}
pub fn dual_clear() -> anyhow::Result<()>{
    queue!(stdout(),cursor::MoveToPreviousLine(2)).context("Unable to move up")?;
    queue!(stdout(),Clear(ClearType::FromCursorDown)).context("Unable to clear line")?;
    execute_queue()?;
    Ok(())

}
pub fn print_char<T: Display>(c: T) -> anyhow::Result<()>{
    queue!(stdout(),Print(c)).context("Unable to print")?;
    execute_queue()?;
    Ok(())
}
pub fn print_backspace() -> anyhow::Result<()>{
    queue!(stdout(),cursor::MoveLeft(1)).context("Unable to move cursor")?;
    queue!(stdout(),Print(' ')).context("Unable to print")?;
    queue!(stdout(),cursor::MoveLeft(1)).context("Unable to move cursor")?;
    execute_queue()?;
    Ok(())
}
pub fn initialize() -> anyhow::Result<()>{
    hide_cursor()?;
    clear_screen()?;
    crossterm::terminal::enable_raw_mode().context("Unable to enter raw mode")?;
    execute_queue()?;
    Ok(())
}

pub fn terminate() -> anyhow::Result<()>{
    clear_screen()?;
    show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;
    execute_queue()?;
    Ok(())
}
