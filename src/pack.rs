use linked_hash_map::Entry as LinkedHashMapEntry;
use hrx::{HrxEntryData, HrxArchive, HrxEntry};
use self::super::{wcxhead, state};
use std::path::{PathBuf, Path};
use std::num::NonZeroUsize;
use libc::{c_int, INT_MAX};
use std::convert::TryInto;
use std::fs::{self, File};
use std::borrow::Cow;
use std::io::Read;
use std::ptr;


pub fn pack_archive<Pf, Sup, Srp, Al, AlE>(packed_file: Pf, sub_path: Option<Sup>, source_path: Srp, add_list: Al, flags: c_int) -> Result<(), c_int>
    where Pf: Into<PathBuf>,
          Sup: AsRef<str>,
          Srp: AsRef<Path>,
          Al: Iterator<Item = AlE>,
          AlE: AsRef<str>
{
    let (delete_originals, save_paths) = pack_archive_parse_flags(flags)?;

    let packed_file = packed_file.into();
    let mut archive = pack_archive_load_archive(&packed_file)?;

    let sub_path = sub_path.as_ref().map(AsRef::as_ref);
    let source_path = source_path.as_ref();
    for add_list_elem in add_list {
        if pack_archive_add_element_to_archive(&mut archive, sub_path, source_path, add_list_elem.as_ref(), delete_originals, save_paths)? {
            return Err(wcxhead::E_EABORTED);
        }
    }

    pack_archive_write_archive(archive, packed_file)
}

fn pack_archive_parse_flags(flags: c_int) -> Result<(bool, bool), c_int> {
    if (flags & wcxhead::PK_PACK_ENCRYPT) != 0 {
        return Err(wcxhead::E_NOT_SUPPORTED);
    }

    Ok(((flags & wcxhead::PK_PACK_MOVE_FILES) != 0, (flags & wcxhead::PK_PACK_SAVE_PATHS) != 0))
}

fn pack_archive_load_archive(packed_file: &Path) -> Result<HrxArchive, c_int> {
    if !packed_file.exists() {
        Ok(HrxArchive::new(NonZeroUsize::new(3).unwrap()))
    } else {
        load_archive(packed_file)
    }
}

fn pack_archive_add_element_to_archive(archive: &mut HrxArchive, sub_path: Option<&str>, source_path: &Path, add_list_elem: &str, delete_originals: bool,
                                       save_paths: bool)
                                       -> Result<bool, c_int> {
    let fs_path = source_path.join(add_list_elem);

    let file_data = read_file_string(&fs_path)?;
    let file_data_len = file_data.len();
    let file_data = HrxEntryData::File { body: Some(file_data) };

    let add_list_elem = if add_list_elem.contains('\\') {
        Cow::from(add_list_elem.replace('\\', "/"))
    } else {
        Cow::from(add_list_elem)
    };
    let add_list_elem = if save_paths {
        match add_list_elem.rfind('/') {
            Some(last_slash) => &add_list_elem[last_slash + 1..],
            None => &add_list_elem[..],
        }
    } else {
        &add_list_elem[..]
    };

    let file_path = match sub_path {
            Some(sub_path) => format!("{}/{}", sub_path, add_list_elem).parse(),
            None => add_list_elem.parse(),
        }.map_err(|_| wcxhead::E_UNKNOWN_FORMAT)?;

    match archive.entries.entry(file_path) {
        LinkedHashMapEntry::Occupied(oe) => oe.into_mut().data = file_data,
        LinkedHashMapEntry::Vacant(ve) => {
            ve.insert(HrxEntry {
                comment: None,
                data: file_data,
            });
        }
    }

    if delete_originals {
        fs::remove_file(fs_path).map_err(|_| wcxhead::E_EOPEN)?;
    }

    Ok(data_processed(file_data_len))
}

fn pack_archive_write_archive(mut archive: HrxArchive, packed_file: PathBuf) -> Result<(), c_int> {
    if archive.validate_content().is_err() {
        let mut boundlen = archive.boundary_length().get() + 1;

        while archive.set_boundary_length(NonZeroUsize::new(boundlen).unwrap()).is_err() {
            boundlen += 1;
        }
    }

    write_archive(archive, packed_file)
}


pub fn modify_archive<Pf, Dl, DlE>(packed_file: Pf, delete_list: Dl) -> Result<(), c_int>
    where Pf: Into<PathBuf>,
          Dl: Iterator<Item = DlE>,
          DlE: AsRef<str>
{
    let packed_file = packed_file.into();
    let mut archive = load_archive(&packed_file)?;

    for delete_list_elem in delete_list {
        if modify_archive_delete_element_from_archive(&mut archive, delete_list_elem.as_ref())? {
            return Err(wcxhead::E_EABORTED);
        }
    }

    write_archive(archive, packed_file)
}

fn modify_archive_delete_element_from_archive(archive: &mut HrxArchive, delete_list_elem: &str) -> Result<bool, c_int> {
    let delete_list_elem = if delete_list_elem.contains('\\') {
        Cow::from(delete_list_elem.replace('\\', "/"))
    } else {
        Cow::from(delete_list_elem)
    };

    match archive.entries.remove(&delete_list_elem[..]) {
        Some(entry) => {
            let entry_data_len = match entry.data {
                HrxEntryData::File { body } => body.map(|s| s.len()).unwrap_or(0),
                HrxEntryData::Directory => 0,
            };

            Ok(data_processed(entry_data_len))
        }
        None => Err(wcxhead::E_NO_FILES),
    }
}


pub fn is_valid_archive<Fn: AsRef<Path>>(file_name: Fn) -> bool {
    is_valid_archive_impl(file_name.as_ref())
}

fn is_valid_archive_impl(file_name: &Path) -> bool {
    load_archive(&file_name).is_ok()
}


fn read_file_string(path: &Path) -> Result<String, c_int> {
    let mut file = File::open(path).map_err(|_| wcxhead::E_EOPEN)?;

    let mut bytes = Vec::with_capacity(file.metadata().map(|m| m.len() as usize + 1 /* stolen from std::fs::read() */).unwrap_or(0));
    file.read_to_end(&mut bytes).map_err(|_| wcxhead::E_EREAD)?;

    String::from_utf8(bytes).map_err(|_| wcxhead::E_UNKNOWN_FORMAT)
}

fn load_archive(path: &Path) -> Result<HrxArchive, c_int> {
    read_file_string(path)?.parse().map_err(|_| wcxhead::E_BAD_ARCHIVE)
}

fn write_archive(archive: HrxArchive, packed_file: PathBuf) -> Result<(), c_int> {
    let mut out_f = File::create(packed_file).map_err(|_| wcxhead::E_ECREATE)?;

    // Assume boundary was verified, so the only error can be I/O
    archive.serialise(&mut out_f).map_err(|_| wcxhead::E_EWRITE)?;

    Ok(())
}

fn data_processed(len: usize) -> bool {
    let len = len.try_into().unwrap_or(INT_MAX);

    if let Some(cbk) = unsafe { state::GLOBAL_PROCESS_DATA_CALLBACK_W } {
        cbk(ptr::null_mut(), len) == 0
    } else if let Some(cbk) = unsafe { state::GLOBAL_PROCESS_DATA_CALLBACK } {
        cbk(ptr::null_mut(), len) == 0
    } else {
        false
    }
}
