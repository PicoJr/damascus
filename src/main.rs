use itertools::{EitherOrBoth, Itertools};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use structopt::StructOpt;

pub struct BufChunks<'a, R>
where
    R: BufRead,
{
    reader: R,
    buffer: &'a mut [u8],
}

impl<'a, R> BufChunks<'a, R>
where
    R: BufRead,
{
    pub fn new(reader: R, buffer: &'a mut [u8]) -> Self {
        BufChunks { reader, buffer }
    }
}

impl<'b, R> Iterator for BufChunks<'b, R>
where
    R: BufRead,
{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_exact(self.buffer) {
            Ok(()) => Some(self.buffer.to_vec()),
            Err(_) => None,
        }
    }
}

#[derive(StructOpt)]
struct Cli {
    /// first stream
    #[structopt(parse(from_os_str))]
    s0: std::path::PathBuf,
    /// 2nd stream
    #[structopt(parse(from_os_str))]
    s1: std::path::PathBuf,
    /// default
    #[structopt(parse(from_os_str))]
    default: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let stream0 = File::open(args.s0)?;
    let stream1 = File::open(args.s1)?;
    let mut default: Vec<u8> = vec![];
    File::open(args.default)?.read_to_end(&mut default)?;
    let bufreader0 = BufReader::new(stream0);
    let bufreader1 = BufReader::new(stream1);
    let mut buffer0 = vec![0; default.len()];
    let mut buffer1 = vec![0; default.len()];
    let chunks0 = BufChunks::new(bufreader0, buffer0.as_mut_slice());
    let chunks1 = BufChunks::new(bufreader1, buffer1.as_mut_slice());
    let zipped = chunks0.zip_longest(chunks1);
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());
    for zip in zipped {
        match zip {
            EitherOrBoth::Both(left, right) => {
                handle.write_all(left.as_slice())?;
                handle.write_all(right.as_slice())?;
            }
            EitherOrBoth::Left(left) => {
                handle.write_all(left.as_slice())?;
                handle.write_all(default.as_slice())?;
            }
            EitherOrBoth::Right(right) => {
                handle.write_all(default.as_slice())?;
                handle.write_all(right.as_slice())?;
            }
        };
    }
    Ok(())
}
