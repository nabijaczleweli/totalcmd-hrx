use hrx::{HrxEntryData, HrxArchive, HrxEntry, HrxPath};
use linked_hash_map::Iter as LinkedHashMapIter;
use std::io::{Write, Read};
use std::time::SystemTime;
use std::borrow::Cow;
use std::path::Path;
use std::fs::File;
use libc::c_int;


pub static mut GLOBAL_PROCESS_DATA_CALLBACK: Option<wcxhead::tProcessDataProc> = None;
pub static mut GLOBAL_PROCESS_DATA_CALLBACK_W: Option<wcxhead::tProcessDataProcW> = None;


pub struct ArchiveState {
    pub arch: HrxArchive,
    pub mod_time: SystemTime,

    pub process_data_callback: Option<wcxhead::tProcessDataProc>,
    pub process_data_callback_w: Option<wcxhead::tProcessDataProcW>,

    arch_iter: Option<LinkedHashMapIter<'static, HrxPath, HrxEntry>>,
    cur_entry: Option<(&'static HrxPath, &'static HrxEntry)>,
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

        let string = String::from_utf8(bytes).map_err(|_| wcxhead::E_UNKNOWN_FORMAT)?;

        Ok(ArchiveState {
            arch: string.parse().map_err(|_| wcxhead::E_BAD_ARCHIVE)?,
            mod_time: file_time,
            process_data_callback: None,
            process_data_callback_w: None,
            arch_iter: None,
            cur_entry: None,
        })
    }

    pub fn next_entry(&'static mut self) -> Option<(&HrxPath, &HrxEntry)> {
        if self.arch_iter.is_none() {
            self.arch_iter = Some(self.arch.entries.iter());
        }

        self.cur_entry = self.arch_iter.as_mut().unwrap().next();
        self.cur_entry
    }

    pub fn extract_current_entry<Pd: AsRef<Path>, Pn: AsRef<Path>>(&self, dest_path: Option<Pd>, dest_name: Option<Pn>) -> Result<(), c_int> {
        self.extract_current_entry_impl(dest_path.as_ref().map(AsRef::as_ref), dest_name.as_ref().map(AsRef::as_ref))
    }

    fn extract_current_entry_impl(&self, dest_path: Option<&Path>, dest_name: Option<&Path>) -> Result<(), c_int> {
        let data = match &self.cur_entry.ok_or(wcxhead::E_END_ARCHIVE)?.1.data {
            HrxEntryData::File { body } => body.as_ref().map(|s| &s[..]).unwrap_or(""),
            HrxEntryData::Directory => "",
        };

        let dest_name = dest_name.ok_or(wcxhead::E_NOT_SUPPORTED)?;
        let mut out_f = File::create(if let Some(dest_path) = dest_path {
                Cow::from(dest_path.join(dest_name))
            } else {
                Cow::from(dest_name)
            }).map_err(|_| wcxhead::E_ECREATE)?;

        out_f.write_all(data.as_bytes()).map_err(|_| wcxhead::E_EWRITE)
    }
}
