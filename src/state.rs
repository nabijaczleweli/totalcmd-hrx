use linked_hash_map::Iter as LinkedHashMapIter;
use hrx::{HrxArchive, HrxEntry, HrxPath};
use std::time::SystemTime;
use self::super::wcxhead;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use libc::c_int;


pub struct ArchiveState {
    pub arch: HrxArchive,
    pub mod_time: SystemTime,

    arch_iter: Option<Box<LinkedHashMapIter<'static, HrxPath, HrxEntry>>>,
}

impl ArchiveState {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<ArchiveState, c_int> {
        ArchiveState::open_impl(path.as_ref())
    }

    fn open_impl(path: &Path) -> Result<ArchiveState, c_int> {
        let mut file = File::open(path).map_err(|_| wcxhead::E_EOPEN)?;
        let (file_len, file_time) = match file.metadata() {
            Ok(metadata) => (metadata.len() as usize + 1 /* stolen from std::fs::read() */, metadata.modified().ok().unwrap_or_else(SystemTime::now)),
            Err(_) => (0, SystemTime::now()),
        };

        let mut bytes = Vec::with_capacity(file_len);
        file.read_to_end(&mut bytes).map_err(|_| wcxhead::E_EREAD)?;

        let string = String::from_utf8(bytes).map_err(|_| wcxhead::E_BAD_ARCHIVE)?;

        Ok(ArchiveState {
            arch: string.parse().map_err(|_| wcxhead::E_UNKNOWN_FORMAT)?, // TODO: right value?
            mod_time: file_time,
            arch_iter: None,
        })
    }

    pub fn next_entry(&'static mut self) -> Option<(&HrxPath, &HrxEntry)> {
        if self.arch_iter.is_none() {
            self.arch_iter = Some(Box::new(self.arch.entries.iter()));
        }

        self.arch_iter.as_mut().unwrap().next()
    }
}
