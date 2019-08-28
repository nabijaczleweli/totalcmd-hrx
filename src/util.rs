//! Various utility functions


use std::time::{SystemTime, Duration};
use libc::{tm, time_t, c_int};
use std::{slice, ptr};
use num_traits::Num;


/// Iterate over a NUL-terminated list of C-strings
///
/// Supplying a pointer that doesn't terminate in two zeroes results in undefined behaviour
///
/// The element slices don't include the terminating NUL character
///
/// # Examples
///
/// ```
/// # use totalcmd_hrx::util::CListIter;
/// # use std::ffi::CStr;
/// # use std::slice;
/// assert_eq!(CListIter(
///     [b'O', b'w', b'O', b'\0', b'h', b'e', b'w', b'w', b'o', b'\0', b'\0'].as_ptr())
///         .map(|s| unsafe { CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(s.as_ptr(), s.len() + 1)) })
///         .map(CStr::to_string_lossy)
///         .collect::<Vec<_>>(),
///     &["OwO", "hewwo"]);
/// ```
pub struct CListIter<T>(pub *const T);

impl<T: 'static + Num> Iterator for CListIter<T> {
    type Item = &'static [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_null() {
            return None;
        }

        let mut len = 0;
        while unsafe { !(*self.0.offset(len)).is_zero() } {
            len += 1;
        }

        if len != 0 {
            let ret = unsafe { slice::from_raw_parts(self.0, len as usize) };
            self.0 = unsafe { self.0.offset(len).offset(1) };
            Some(ret)
        } else {
            self.0 = ptr::null();
            None
        }
    }
}


/// `FileTime` contains the date and the time of the file’s last update. Use the following algorithm to set the value:
///
/// ```c
/// FileTime = (year - 1980) << 25 | month << 21 | day << 16 | hour << 11 | minute << 5 | second/2;
/// ```
///
/// Make sure that:
///
/// `year` is in the four digit format between 1980 and 2100
///
/// `month` is a number between 1 and 12
///
/// `hour` is in the 24 hour format
///
/// # Examples
///
/// ```
/// # use totalcmd_hrx::util::system_time_to_totalcmd_time;
/// # use std::time::SystemTime;
/// println!("{}", system_time_to_totalcmd_time(&SystemTime::now()));
/// ```
pub fn system_time_to_totalcmd_time(tm: &SystemTime) -> c_int {
    let time = unsafe { *localtime(&(tm.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_else(|_| Duration::new(0, 0)).as_secs() as time_t)) };

    let year = (1900 + time.tm_year).max(1980).min(2100);
    let month = time.tm_mon + 1;

    // fs::write("system_time_to_totalcmd_time.log",
    //           format!("year: {}\nmon : {}\nmday: {}\nhour: {}\nmin : {}\nsec : {}",
    //                   year, month, time.tm_mday, time.tm_hour, time.tm_min, time.tm_sec).as_bytes()).unwrap();

    (year - 1980) << 25 | month << 21 | time.tm_mday << 16 | time.tm_hour << 11 | time.tm_min << 5 | time.tm_sec / 2
}


extern "C" {
    fn localtime(time_p: *const time_t) -> *mut tm;
}
