use self::super::wcxhead;
use std::path::Path;
use hrx::HrxArchive;
use std::fs::File;
use std::io::Read;
use libc::c_int;


#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArchiveState {
    pub arch: HrxArchive,
}

impl ArchiveState {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<ArchiveState, c_int> {
        ArchiveState::open_impl(path.as_ref())
    }

    fn open_impl(path: &Path) -> Result<ArchiveState, c_int> {
        let mut file = File::open(path).map_err(|_| wcxhead::E_EOPEN)?;

        let mut bytes = Vec::with_capacity(file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0));
        file.read_to_end(&mut bytes).map_err(|_| wcxhead::E_EREAD)?;

        let string = String::from_utf8(bytes).map_err(|_| wcxhead::E_BAD_ARCHIVE)?;

        let arch = string.parse().map_err(|_| wcxhead::E_UNKNOWN_FORMAT)?; // TODO: right value?
        Ok(ArchiveState { arch: arch })
    }
}
