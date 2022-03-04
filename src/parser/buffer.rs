//! A circular buffer, adapted from the [`circular`] crate to support recording spans.
//!
//! [`circular`]: https://docs.rs/circular

use std::io::Read;
use std::io::Write;
use std::ptr;

use super::QuickFind;

/// the Buffer contains the underlying memory and data positions
///
/// In all cases, `0 ≤ position ≤ end ≤ capacity` should be true
#[derive(Debug)]
pub struct Buffer {
    /// The Vec containing the data
    memory: Vec<u8>,
    /// The current capacity of the `Buffer`.
    capacity: usize,
    /// The current beginning of the available data.
    position: usize,
    /// The current beginning of the available space.
    end: usize,
    /// The total number of bytes already consumed.
    consumed: usize,
    /// The total number of lines already consumed.
    consumed_lines: usize,
}

impl Buffer {
    /// Allocates a new buffer of maximum size `capacity`.
    pub fn with_capacity(capacity: usize) -> Buffer {
        let mut v = vec![0; capacity];
        Buffer {
            memory: v,
            capacity: capacity,
            position: 0,
            end: 0,
            consumed: 0,
            consumed_lines: 0,
        }
    }

    /// Increases the size of the buffer
    ///
    /// This does nothing if the buffer is already large enough.
    pub fn grow(&mut self, new_size: usize) -> bool {
        if self.capacity >= new_size {
            return false;
        }

        self.memory.resize(new_size, 0);
        self.capacity = new_size;
        true
    }

    /// Returns how much data can be read from the buffer.
    pub fn available_data(&self) -> usize {
        self.end - self.position
    }

    ///Rreturns how much free space is available to write to.
    pub fn available_space(&self) -> usize {
        self.capacity - self.end
    }

    /// Returns the size of the underlying vector.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns `true` if there is no available data.
    pub fn empty(&self) -> bool {
        self.position == self.end
    }

    /// Advances the position tracker.
    ///
    /// If the position gets past the buffer's half, this will call [`Buffer::shift`]
    /// to move the rest of the data to the beginning of the buffer.
    pub fn consume(&mut self, count: usize) -> usize {
        let cnt = self.available_data().min(count);
        self.consumed_lines += (&self.data()[..cnt]).quickcount(b'\n');
        self.position += cnt;
        self.consumed += cnt;
        if self.position > self.capacity / 2 {
            self.shift();
        }
        cnt
    }

    /// Indicate to the buffer how many bytes were written into ``space``.
    pub fn fill(&mut self, count: usize) -> usize {
        let cnt = self.available_space().min(count);
        self.end += cnt;
        cnt
    }

    /// Get the current position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Return a slice with all the available data.
    pub fn data(&self) -> &[u8] {
        &self.memory[self.position..self.end]
    }

    /// Return a mutable slice with all the available space to write to.
    pub fn space(&mut self) -> &mut [u8] {
        &mut self.memory[self.end..self.capacity]
    }

    /// Move the data at the beginning of the buffer.
    pub fn shift(&mut self) {
        if self.position > 0 {
            unsafe {
                let length = self.end - self.position;
                ptr::copy(
                    (&self.memory[self.position..self.end]).as_ptr(),
                    (&mut self.memory[..length]).as_mut_ptr(),
                    length,
                );
                self.position = 0;
                self.end = length;
            }
        }
    }

    /// Get the total number consumed so far.
    pub fn consumed(&self) -> usize {
        self.consumed
    }

    /// Get the total number of lines consumed so far.
    pub fn consumed_lines(&self) -> usize {
        self.consumed_lines
    }
}

// impl Write for Buffer {
//   fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//     match self.space().write(buf) {
//       Ok(size) => { self.fill(size); Ok(size) },
//       err      => err
//     }
//   }
//
//   fn flush(&mut self) -> std::io::Result<()> {
//     Ok(())
//   }
// }
//
// impl Read for Buffer {
//   fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//     let len = self.available_data().min(buf.len());
//     unsafe {
//       ptr::copy((&self.memory[self.position..self.position+len]).as_ptr(), buf.as_mut_ptr(), len);
//       self.position += len;
//     }
//     Ok(len)
//   }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    // #[test]
    // fn fill_and_consume() {
    //   let mut b = Buffer::with_capacity(10);
    //   assert_eq!(b.available_data(), 0);
    //   assert_eq!(b.available_space(), 10);
    //   let res = b.write(&b"abcd"[..]);
    //   assert_eq!(res.ok(), Some(4));
    //   assert_eq!(b.available_data(), 4);
    //   assert_eq!(b.available_space(), 6);
    //
    //   assert_eq!(b.data(), &b"abcd"[..]);
    //
    //   b.consume(2);
    //   assert_eq!(b.available_data(), 2);
    //   assert_eq!(b.available_space(), 6);
    //   assert_eq!(b.data(), &b"cd"[..]);
    //
    //   b.shift();
    //   assert_eq!(b.available_data(), 2);
    //   assert_eq!(b.available_space(), 8);
    //   assert_eq!(b.data(), &b"cd"[..]);
    //
    //   assert_eq!(b.write(&b"efghijklmnop"[..]).ok(), Some(8));
    //   assert_eq!(b.available_data(), 10);
    //   assert_eq!(b.available_space(), 0);
    //   assert_eq!(b.data(), &b"cdefghijkl"[..]);
    //   b.shift();
    //   assert_eq!(b.available_data(), 10);
    //   assert_eq!(b.available_space(), 0);
    //   assert_eq!(b.data(), &b"cdefghijkl"[..]);
    // }
    //
    // #[test]
    // fn set_position() {
    //   let mut output = [0;5];
    //   let mut b = Buffer::with_capacity(10);
    //   let _ = b.write(&b"abcdefgh"[..]);
    //   let _ = b.read(&mut output);
    //   assert_eq!(b.available_data(), 3);
    //   println!("{:?}", b.position());
    // }
}
