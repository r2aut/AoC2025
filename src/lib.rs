use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// pub trait AnyhowLogExt<T, E> {
//     fn log_err(self) -> Self
//     where
//         E: Display;
//     fn log_context<C>(self, context: C) -> Self
//     where
//         E: Display,
//         C: Display;
// }

// // for any Result
// impl<T, E> AnyhowLogExt<T, E> for Result<T, E> {
//     fn log_err(self) -> Self
//     where
//         E: Display,
//     {
//         if let Err(ref e) = self {
//             error!("Ошибка: {}", e);
//         }
//         self
//     }

//     fn log_context<C>(self, context: C) -> Self
//     where
//         E: Display,
//         C: Display,
//     {
//         if let Err(ref e) = self {
//             error!("{}: {}", context, e);
//         }
//         self
//     }
// }

#[cfg(not(tarpaulin_include))]
pub fn get_reader<P: AsRef<Path>>(path: P) -> Result<impl BufRead> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
