use crate::errors::{ACResult, Error};
use std::io::{BufRead, Read};

pub fn read_line<T: BufRead>(data: T) -> ACResult<String> {
    let line = data
        .lines()
        .take(1)
        .collect::<Result<String, _>>()
        .map_err(|_| Error::new_str("Failed to read lines"))?;
    Ok(line)
}

pub fn read_lines<T: BufRead>(data: T) -> ACResult<Vec<String>> {
    let lines = data
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(|_| Error::new_str("Failed to read lines"))?;
    Ok(lines)
}

pub fn read_all<T: Read>(mut data: T) -> ACResult<String> {
    let mut contents = String::new();
    data.read_to_string(&mut contents)
        .map_err(|_| Error::new_str("Failed to read stdin"))?;
    Ok(contents)
}

pub struct Field<T> {
    field: Vec<T>,
    width: u32,
    height: u32,
}

impl<T: Default + Clone> Field<T> {
    pub fn new(width: u32, height: u32) -> Self {
        Field {
            field: vec![T::default(); (width * height) as usize],
            width,
            height,
        }
    }
}

impl<T> Field<T> {
    pub fn from(field: Vec<T>, width: u32, height: u32) -> Self {
        assert_eq!(field.len(), (width * height) as usize);

        Self {
            field,
            width,
            height,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> &T {
        &self.field[(y * self.width + x) as usize]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut T {
        &mut self.field[(y * self.width + x) as usize]
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl<T> Into<Field<T>> for Field<Option<T>> {
    fn into(self) -> Field<T> {
        Field {
            field: self.field.into_iter().map(|c| c.unwrap()).collect(),
            width: self.width,
            height: self.height,
        }
    }
}
