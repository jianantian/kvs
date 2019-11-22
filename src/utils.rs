use std::io;
use std::io::{BufReader, BufWriter, SeekFrom, Read, Seek, Write};


pub struct BufReaderWithPos<W: Read + Seek> {
  reader: BufReader<W>,
  pub pos: u64,
}

pub struct BufWriterWithPos<W: Write + Seek> {
  writer: BufWriter<W>,
  pub pos: u64,
}

impl<R: Read + Seek> BufReaderWithPos<R> {
  pub fn new(mut inner: R) -> io::Result<Self> {
      let pos = inner.seek(SeekFrom::Current(0))?;
      Ok(BufReaderWithPos {
          reader: BufReader::new(inner),
          pos,
      })
  }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
      let len = self.reader.read(buf)?;
      self.pos += len as u64;
      Ok(len)
  }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
  fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
      self.pos = self.reader.seek(pos)?;
      Ok(self.pos)
  }
}

impl<W: Write + Seek> BufWriterWithPos<W> {
  pub fn new(mut inner: W) -> io::Result<Self> {
      let pos = inner.seek(SeekFrom::Current(0))?;
      Ok(BufWriterWithPos {
          writer: BufWriter::new(inner),
          pos,
      })
  }
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
      let len = self.writer.write(buf)?;
      self.pos += len as u64;
      Ok(len)
  }

  fn flush(&mut self) -> io::Result<()> {
      self.writer.flush()
  }
}

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
  fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
      self.pos = self.writer.seek(pos)?;
      Ok(self.pos)
  }
}