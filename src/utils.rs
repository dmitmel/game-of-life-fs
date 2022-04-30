pub trait ResultContextExt<T, E> {
  fn context(self, ctx: &str) -> Self;
}

use std::io;
impl<T> ResultContextExt<T, io::Error> for io::Result<T> {
  fn context(self, ctx: &str) -> Self {
    self.map_err(|e| {
      use io::Error;
      Error::new(e.kind(), format!("{}: {}", ctx, e))
    })
  }
}
