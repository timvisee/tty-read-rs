extern crate libc;
extern crate termios;

use std::io::{Read, Result, stdin};
use std::os::unix::io::RawFd;

use self::termios::{
    cfmakeraw,
    ECHO,
    TCSANOW,
    tcsetattr,
    Termios,
};

use super::ReaderOptions;



/// A terminal reader, that reads raw terminal input.
///
/// Because this reader reads in raw mode, input can immediately be processed
/// as soon as it comes in. The user doesn't have to press enter/return in
/// to make the input available to the program. This might be useful in 
/// real-time applications, or in applications that use raw input.
///
/// When this reader is opened, raw mode is enabled in the terminal.
/// This might cause wierd formatting when printing to stdout in this mode.
/// The original mode is automatically reverted when the reader is dropped.
pub struct TermReader<'a> {
    /// Reader options object
    options: &'a ReaderOptions,

    /// Raw terminal file descriptor
    fd: RawFd,

    /// The original terminal state, before the reader started
    original: Termios,

    /// The terminal state while the reader is active
    raw: Termios,
}

impl<'a> TermReader<'a> {
    /// Open the terminal reader, from the given raw file descriptor.
    ///
    /// This puts the terminal in a raw mode, so raw input can be handled.
    /// When the TermReader instance is dropped, the terminal state is
    /// reverted.
    ///
    /// Use `TermReader::open_stdin();` instead to open a reader for stdin.
    pub fn open(fd: RawFd, options: &'a ReaderOptions) -> Result<TermReader> {
        // Get the current terminal state
        let original = Termios::from_fd(fd)?;
        let raw = Termios::from_fd(fd)?;

        // Instantiate the terminal state
        let mut state = TermReader {
            options,
            fd,
            original,
            raw,
        };

        // Enable raw mode and return
        state.start_raw()?;
        Ok(state)
    }

    /// Open the terminal reader for stdin.
    ///
    /// This puts the terminal in a raw mode, so raw input can be handled.
    /// When the TermReader instance is dropped, the terminal state is
    /// reverted.
    pub fn open_stdin(options: &'a ReaderOptions) -> Result<TermReader<'a>> {
        TermReader::open(libc::STDIN_FILENO, options)
    }

    /// Start the raw terminal mode.
    ///
    /// This is an internal method that should be called when the reader is
    /// created.
    fn start_raw(&mut self) -> Result<()> {
        // Enable raw mode in the current terminal
        cfmakeraw(&mut self.raw);

        // Enable input echoing
        if self.options.echo {
            self.raw.c_lflag |= ECHO;
        }

        // Push the raw terminal state
        tcsetattr(self.fd, TCSANOW, &self.raw)
    }

    /// Read the given number of bytes from the terminal.
    ///
    /// This method blocks until the given number of bytes is read.
    pub fn read_bytes(&self, amount: usize) -> Result<Vec<u8>> {
        stdin().bytes().take(amount).collect()
    }

    /// Read the same number of bytes, as the size of the given `buffer`.
    /// The read bytes will be written to the buffer.
    ///
    /// This method blocks until the buffer is filled.
    pub fn read_bytes_buffer(&self, buffer: &mut [u8]) -> Result<()> {
        stdin().read_exact(buffer)
    }

    /// Read a single byte from the terminal.
    ///
    /// This method blocks until a byte is read.
    pub fn read_byte(&self) -> Result<u8> {
        // Allocate a buffer
        let mut buffer = [0; 1];

        // Read and output
        self.read_bytes_buffer(&mut buffer)?;
        Ok(buffer[0])
    }
}

impl<'a> Drop for TermReader<'a> {
    /// Revert the terminal state form raw mode, when the reader is dropped.
    fn drop(&mut self) {
        // Revert the terminal, panic if failed
        tcsetattr(self.fd, TCSANOW, &self.original)
            .expect("failed to revert terminal state, \
                please run 'reset' to recover");
    }
}
