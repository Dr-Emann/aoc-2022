use std::fmt;
use std::fmt::Write;

// Newline Seperated Grid
#[derive(Copy, Clone)]
pub struct NlGrid<'a> {
    width: usize,
    data: &'a [u8],
}

impl<'a> NlGrid<'a> {
    pub fn new(data: &'a str) -> Self {
        let width = data.find('\n').unwrap();
        Self {
            width,
            data: data.as_bytes(),
        }
    }

    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x > self.width as i32 {
            return None;
        }
        let idx = (y * (self.width + 1) as i32 + x).try_into().ok()?;
        if idx >= self.data.len() {
            return None;
        }

        Some(idx)
    }

    pub fn get(&self, x: i32, y: i32) -> Option<u8> {
        let idx = self.idx(x, y)?;
        Some(self.data[idx])
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        (self.data.len() + 1) / (self.width + 1)
    }

    pub fn position_of(&self, byte: u8) -> Option<(i32, i32)> {
        let i = self.data.iter().position(|&b| b == byte)?;

        let stride = self.width + 1;
        Some(((i % stride) as i32, (i / stride) as i32))
    }

    pub fn multi_position(&self, byte: u8) -> impl Iterator<Item = (i32, i32)> + '_ {
        let stride = self.width + 1;

        self.data
            .iter()
            .enumerate()
            .filter_map(move |(i, &b)| (b == byte).then_some(i))
            .map(move |i| ((i % stride) as i32, (i / stride) as i32))
    }
}

impl<'a> fmt::Debug for NlGrid<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stride = self.width + 1;
        for row in self.data.chunks(stride) {
            f.write_char('\n')?;
            f.write_str(std::str::from_utf8(&row[..self.width]).unwrap())?;
        }
        Ok(())
    }
}
