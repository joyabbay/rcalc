mod stack;
mod terminal;
use anyhow::Context;
use crossterm::event::{
    Event::Key, KeyCode::Backspace, KeyCode::Char, KeyCode::Enter, KeyEvent, read,
};
use stack::Stack;

pub struct Calc {
    should_exit: bool,
    num_buffer: String,
    num_stack: Stack,
}
impl Calc {
    fn parse_enter_stroke(&mut self) -> anyhow::Result<()> {
        let curr_num = self.num_buffer.parse::<i64>();
        match curr_num {
            Ok(num) => {
                self.num_stack.push(num);
                terminal::print_newline()?;
            }
            _ => (),
        }
        self.num_buffer.clear();
        Ok(())
    }
    pub fn default() -> Calc {
        Calc {
            should_exit: false,
            num_buffer: String::with_capacity(40),
            num_stack: Stack::default(),
        }
    }
    fn process_key(&mut self, k: KeyEvent) -> anyhow::Result<()> {
        match k.code {
            Char('q') => {
                self.should_exit = true;
            }
            Char(c @ '0'..='9') => {
                self.num_buffer.push(c);
                terminal::print_char(c)?;
            }
            Enter => {
                self.parse_enter_stroke()?;
            }
            Backspace => {
                if let Some(_) = self.num_buffer.pop() {
                    terminal::print_backspace()?;
                }
            }
            Char('+') => {
                self.parse_enter_stroke()?;
                match self.num_stack.bipop() {
                    Some((a, b)) => {
                        terminal::dual_clear()?;
                        let res = a.saturating_add(b);
                        self.num_stack.push(res);
                        terminal::print_char(res)?;
                        terminal::print_newline()?;
                    }
                    None => (),
                }
            }
            Char('-') => {
                self.parse_enter_stroke()?;
                match self.num_stack.bipop() {
                    Some((a, b)) => {
                        terminal::dual_clear()?;
                        let res = b.saturating_sub(a);
                        self.num_stack.push(res);
                        terminal::print_char(res)?;
                        terminal::print_newline()?;
                    }
                    None => (),
                }
            }
            Char('*') => {
                self.parse_enter_stroke()?;
                match self.num_stack.bipop() {
                    Some((a, b)) => {
                        terminal::dual_clear()?;
                        let res = b.saturating_mul(a);
                        self.num_stack.push(res);
                        terminal::print_char(res)?;
                        terminal::print_newline()?;
                    }
                    None => (),
                }
            },
            
            Char('/') => {
                self.parse_enter_stroke()?;
                match self.num_stack.bipop() {
                    Some((a, b)) => {
                        terminal::dual_clear()?;
                        if a!=0{
                        let res = b.saturating_div(a);
                        self.num_stack.push(res);
                        terminal::print_char(res)?;
                        terminal::print_newline()?;
                        }
                    }
                    None => (),
                }
            },

            _ => (),
        }
        Ok(())
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        terminal::initialize()?;
        while !self.should_exit {
            if let Key(k) = read().context("Unable to read")? {
                self.process_key(k)?;
            }
        }
        terminal::terminate()?;
        Ok(())
    }
}
