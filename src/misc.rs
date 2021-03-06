use crate::common::*;

pub fn show_whitespace(text: &str) -> String {
  text
    .chars()
    .map(|c| match c {
      '\t' => '␉',
      ' ' => '␠',
      _ => c,
    })
    .collect()
}

pub fn default<T: Default>() -> T {
  Default::default()
}

pub fn empty<T, C: iter::FromIterator<T>>() -> C {
  iter::empty().collect()
}

pub fn ticks<T: Display>(ts: &[T]) -> Vec<Tick<T>> {
  ts.iter().map(Tick).collect()
}

pub fn maybe_s(n: usize) -> &'static str {
  if n == 1 {
    ""
  } else {
    "s"
  }
}

pub fn conjoin<T: Display>(
  f: &mut Formatter,
  values: &[T],
  conjunction: &str,
) -> Result<(), fmt::Error> {
  match values.len() {
    0 => {}
    1 => write!(f, "{}", values[0])?,
    2 => write!(f, "{} {} {}", values[0], conjunction, values[1])?,
    _ => {
      for (i, item) in values.iter().enumerate() {
        write!(f, "{}", item)?;
        if i == values.len() - 1 {
        } else if i == values.len() - 2 {
          write!(f, ", {} ", conjunction)?;
        } else {
          write!(f, ", ")?
        }
      }
    }
  }
  Ok(())
}

pub fn write_error_context(
  f: &mut Formatter,
  text: &str,
  offset: usize,
  line: usize,
  column: usize,
  width: usize,
) -> Result<(), fmt::Error> {
  let width = if width == 0 { 1 } else { width };

  let line_number = line.ordinal();
  let red = Color::fmt(f).error();
  match text.lines().nth(line) {
    Some(line) => {
      let mut i = 0;
      let mut space_column = 0;
      let mut space_line = String::new();
      let mut space_width = 0;
      for c in line.chars() {
        if c == '\t' {
          space_line.push_str("    ");
          if i < column {
            space_column += 4;
          }
          if i >= column && i < column + width {
            space_width += 4;
          }
        } else {
          if i < column {
            space_column += UnicodeWidthChar::width(c).unwrap_or(0);
          }
          if i >= column && i < column + width {
            space_width += UnicodeWidthChar::width(c).unwrap_or(0);
          }
          space_line.push(c);
        }
        i += c.len_utf8();
      }
      let line_number_width = line_number.to_string().len();
      writeln!(f, "{0:1$} |", "", line_number_width)?;
      writeln!(f, "{} | {}", line_number, space_line)?;
      write!(f, "{0:1$} |", "", line_number_width)?;
      write!(
        f,
        " {0:1$}{2}{3:^<4$}{5}",
        "",
        space_column,
        red.prefix(),
        "",
        space_width,
        red.suffix()
      )?;
    }
    None => {
      if offset != text.len() {
        write!(
          f,
          "internal error: Error has invalid line number: {}",
          line_number
        )?
      }
    }
  }
  Ok(())
}

pub struct And<'a, T: 'a + Display>(pub &'a [T]);

impl<'a, T: Display> Display for And<'a, T> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    conjoin(f, self.0, "and")
  }
}

pub struct Or<'a, T: 'a + Display>(pub &'a [T]);

impl<'a, T: Display> Display for Or<'a, T> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    conjoin(f, self.0, "or")
  }
}

pub struct Tick<'a, T: 'a + Display>(pub &'a T);

impl<'a, T: Display> Display for Tick<'a, T> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "`{}`", self.0)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn conjoin_or() {
    assert_eq!("1", Or(&[1]).to_string());
    assert_eq!("1 or 2", Or(&[1, 2]).to_string());
    assert_eq!("1, 2, or 3", Or(&[1, 2, 3]).to_string());
    assert_eq!("1, 2, 3, or 4", Or(&[1, 2, 3, 4]).to_string());
  }

  #[test]
  fn conjoin_and() {
    assert_eq!("1", And(&[1]).to_string());
    assert_eq!("1 and 2", And(&[1, 2]).to_string());
    assert_eq!("1, 2, and 3", And(&[1, 2, 3]).to_string());
    assert_eq!("1, 2, 3, and 4", And(&[1, 2, 3, 4]).to_string());
  }
}
