use std::time::{SystemTime, Duration};
use libc::{tm, time_t, c_int};


/// `FileTime` contains the date and the time of the file’s last update. Use the following algorithm to set the value:
///
/// FileTime = (year - 1980) << 25 | month << 21 | day << 16 | hour << 11 | minute << 5 | second/2;
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
