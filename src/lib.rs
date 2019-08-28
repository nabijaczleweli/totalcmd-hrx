#![allow(nonstandard_style)]

extern crate linked_hash_map;
extern crate winapi;
extern crate libc;
extern crate hrx;

mod state;

pub mod util;
pub mod wcxhead;

use wcxhead::{tOpenArchiveDataW, tOpenArchiveData, tProcessDataProcW, tProcessDataProc, tChangeVolProcW, tChangeVolProc, tHeaderDataExW, tHeaderDataEx,
              tHeaderData, BACKGROUND_UNPACK, BACKGROUND_PACK, E_END_ARCHIVE};
use libc::{c_char, c_uint, c_int, strncpy, wcslen, INT_MAX};
use std::os::windows::ffi::{OsStringExt, OsStrExt};
use self::util::system_time_to_totalcmd_time;
use winapi::shared::minwindef::{FALSE, BOOL};
use winapi::shared::ntdef::{HANDLE, WCHAR};
use std::ffi::{OsString, OsStr, CStr};
use std::convert::TryInto;
use hrx::HrxEntryData;
use std::{slice, ptr};
use std::path::Path;

pub use self::state::ArchiveState;


/// OpenArchive should perform all necessary operations when an archive is to be opened.
///
/// ```c
/// HANDLE __stdcall OpenArchive (tOpenArchiveData *ArchiveData);
/// ```
///
/// # Description
///
/// OpenArchive should return a unique handle representing the archive. The handle should remain valid until CloseArchive is
/// called. If an error occurs, you should return zero, and specify the error by setting OpenResult member of ArchiveData.
///
/// You can use the ArchiveData to query information about the archive being open, and store the information in ArchiveData to
/// some location that can be accessed via the handle.
#[no_mangle]
pub unsafe extern "stdcall" fn OpenArchive(ArchiveData: *mut tOpenArchiveData) -> HANDLE {
    let ArchiveData = &mut *ArchiveData;

    OpenArchiveImpl(&CStr::from_ptr(ArchiveData.ArcName).to_string_lossy()[..], &mut ArchiveData.OpenResult)
}

#[no_mangle]
pub unsafe extern "stdcall" fn OpenArchiveW(ArchiveData: *mut tOpenArchiveDataW) -> HANDLE {
    let ArchiveData = &mut *ArchiveData;

    OpenArchiveImpl(OsString::from_wide(slice::from_raw_parts(ArchiveData.ArcName, wcslen(ArchiveData.ArcName))),
                    &mut ArchiveData.OpenResult)
}

fn OpenArchiveImpl<P: AsRef<Path>>(path: P, OpenResult: &mut c_int) -> HANDLE {
    OpenArchiveImpl_impl(path.as_ref(), OpenResult)
}

fn OpenArchiveImpl_impl(path: &Path, OpenResult: &mut c_int) -> HANDLE {
    match ArchiveState::open(path) {
        Ok(arch) => Box::into_raw(Box::new(arch)) as HANDLE,
        Err(err) => {
            *OpenResult = err;
            ptr::null_mut()
        }
    }
}


/// Totalcmd calls ReadHeader to find out what files are in the archive.
///
/// ```c
/// int __stdcall ReadHeader (HANDLE hArcData, tHeaderData *HeaderData);
/// ```
///
/// # Description
///
/// ReadHeader is called as long as it returns zero (as long as the previous call to this function returned zero). Each time it
/// is called, `HeaderData` is supposed to provide Totalcmd with information about the next file contained in the archive. When
/// all files in the archive have been returned, ReadHeader should return E_END_ARCHIVE which will prevent ReaderHeader from
/// being called again. If an error occurs, ReadHeader should return one of the error values or 0 for no error.
///
/// `hArcData` contains the handle returned by `OpenArchive`. The programmer is encouraged to store other information in the
/// location that can be accessed via this handle. For example, you may want to store the position in the archive when
/// returning files information in ReadHeader.
///
/// In short, you are supposed to set at least PackSize, UnpSize, FileTime, and FileName members of tHeaderData. Totalcmd will
/// use this information to display content of the archive when the archive is viewed as a directory.
#[no_mangle]
pub unsafe extern "stdcall" fn ReadHeader(hArcData: HANDLE, HeaderData: *mut tHeaderData) -> c_int {
    let state = &mut *(hArcData as *mut ArchiveState);
    let HeaderData = &mut *HeaderData;

    ReadHeaderImpl(state, |entry_len, file_time, fname, file_attr| {
        HeaderData.PackSize = entry_len.try_into().unwrap_or(INT_MAX);
        HeaderData.UnpSize = HeaderData.PackSize;
        HeaderData.FileTime = file_time;

        strncpy(HeaderData.FileName.as_mut_ptr(),
                fname.as_bytes().as_ptr() as *const c_char,
                HeaderData.FileName.len() - 1);
        HeaderData.FileName[HeaderData.FileName.len() - 1] = 0;

        HeaderData.HostOS = 0;
        HeaderData.FileCRC = 0;
        HeaderData.FileAttr = file_attr;
    })
}

/// Totalcmd calls ReadHeaderEx to find out what files are in the archive. This function is always called instead of ReadHeader
/// if it is present. It only needs to be implemented if the supported archive type may contain files >2 GB. You should
/// implement both ReadHeader and ReadHeaderEx in this case, for compatibility with older versions of Total Commander.
///
/// ```c
/// int __stdcall ReadHeaderEx (HANDLE hArcData, tHeaderDataEx *HeaderDataEx);
/// ```
///
/// # Description
///
/// ReadHeaderEx is called as long as it returns zero (as long as the previous call to this function returned zero). Each time
/// it is called, `HeaderDataEx` is supposed to provide Totalcmd with information about the next file contained in the archive.
/// When all files in the archive have been returned, ReadHeaderEx should return E_END_ARCHIVE which will prevent
/// ReaderHeaderEx from being called again. If an error occurs, ReadHeaderEx should return one of the error values or 0 for no
/// error.
///
/// `hArcData` contains the handle returned by OpenArchive. The programmer is encouraged to store other information in the
/// location that can be accessed via this handle. For example, you may want to store the position in the archive when
/// returning files information in ReadHeaderEx.
///
/// In short, you are supposed to set at least PackSize, PackSizeHigh, UnpSize, UnpSizeHigh, FileTime, and FileName members of
/// tHeaderDataEx. Totalcmd will use this information to display content of the archive when the archive is viewed as a
/// directory.
#[no_mangle]
pub unsafe extern "stdcall" fn ReadHeaderEx(hArcData: HANDLE, HeaderDataEx: *mut tHeaderDataEx) -> c_int {
    let state = &mut *(hArcData as *mut ArchiveState);
    let HeaderDataEx = &mut *HeaderDataEx;

    ReadHeaderImpl(state, |entry_len, file_time, fname, file_attr| {
        HeaderDataEx.PackSize = (entry_len & 0xFFFFFF) as c_uint;
        HeaderDataEx.PackSizeHigh = (entry_len.checked_shr(32).unwrap_or(0) & 0xFFFFFF) as c_uint;

        HeaderDataEx.UnpSize = HeaderDataEx.PackSize;
        HeaderDataEx.UnpSizeHigh = HeaderDataEx.PackSizeHigh;

        HeaderDataEx.FileTime = file_time;

        strncpy(HeaderDataEx.FileName.as_mut_ptr(),
                fname.as_bytes().as_ptr() as *const c_char,
                HeaderDataEx.FileName.len() - 1);
        HeaderDataEx.FileName[HeaderDataEx.FileName.len() - 1] = 0;

        HeaderDataEx.HostOS = 0;
        HeaderDataEx.FileCRC = 0;
        HeaderDataEx.FileAttr = file_attr;

        ptr::write_bytes(HeaderDataEx.Reserved.as_mut_ptr(), 0, HeaderDataEx.Reserved.len());
    })
}

#[no_mangle]
pub unsafe extern "stdcall" fn ReadHeaderExW(hArcData: HANDLE, HeaderDataEx: *mut tHeaderDataExW) -> c_int {
    let state = &mut *(hArcData as *mut ArchiveState);
    let HeaderDataEx = &mut *HeaderDataEx;

    ReadHeaderImpl(state, |entry_len, file_time, fname, file_attr| {
        HeaderDataEx.PackSize = (entry_len & 0xFFFFFF) as c_uint;
        HeaderDataEx.PackSizeHigh = (entry_len.checked_shr(32).unwrap_or(0) & 0xFFFFFF) as c_uint;

        HeaderDataEx.UnpSize = HeaderDataEx.PackSize;
        HeaderDataEx.UnpSizeHigh = HeaderDataEx.PackSizeHigh;

        HeaderDataEx.FileTime = file_time;

        let last_idx = HeaderDataEx.FileName.len() - 1;
        let mut written_idx = 0;
        for (i, (out, enc)) in HeaderDataEx.FileName.iter_mut().take(last_idx).zip(OsStr::new(fname).encode_wide()).enumerate() {
            *out = enc;
            written_idx = i;
        }
        ptr::write_bytes(HeaderDataEx.FileName.as_mut_ptr().offset(written_idx as isize + 1),
                         0,
                         last_idx - written_idx + 1);


        HeaderDataEx.HostOS = 0;
        HeaderDataEx.FileCRC = 0;
        HeaderDataEx.FileAttr = file_attr;

        ptr::write_bytes(HeaderDataEx.Reserved.as_mut_ptr(), 0, HeaderDataEx.Reserved.len());
    })
}

fn ReadHeaderImpl<F: FnOnce(usize, c_int, &str, c_int)>(state: &'static mut ArchiveState, callback: F) -> c_int {
    let mod_time = state.mod_time;

    match state.next_entry() {
        Some((fname, entry)) => {
            let (attr, entry_body) = match &entry.data {
                HrxEntryData::File { body } => (0x00, body.as_ref().map(|s| &s[..]).unwrap_or("")),
                HrxEntryData::Directory => (0x10, ""),
            };

            callback(entry_body.len(), system_time_to_totalcmd_time(&mod_time), fname.as_ref(), attr);

            0
        }
        None => E_END_ARCHIVE,
    }
}


/// ProcessFile should unpack the specified file
/// or test the integrity of the archive
#[no_mangle]
pub extern "stdcall" fn ProcessFile(hArcData: HANDLE, Operation: c_int, DestPath: *mut c_char, DestName: *mut c_char) -> c_int {
    0
}
#[no_mangle]
pub extern "stdcall" fn ProcessFileW(hArcData: HANDLE, Operation: c_int, DestPath: *mut WCHAR, DestName: *mut WCHAR) -> c_int {
    0
}

/// CloseArchive should perform all necessary operations when an archive is about to be closed.
#[no_mangle]
pub unsafe extern "stdcall" fn CloseArchive(hArcData: HANDLE) -> c_int {
    Box::from_raw(hArcData as *mut ArchiveState);

    0
}

/// HRX archives are single-volume, safe to ignore
#[no_mangle]
pub extern "stdcall" fn SetChangeVolProc(_: HANDLE, _: tChangeVolProc) {}
#[no_mangle]
pub extern "stdcall" fn SetChangeVolProcW(_: HANDLE, _: tChangeVolProcW) {}

/// This function allows you to notify user about
/// the progress when you un/pack files
#[no_mangle]
pub extern "stdcall" fn SetProcessDataProc(hArcData: HANDLE, pProcessDataProc: tProcessDataProc) {}
#[no_mangle]
pub extern "stdcall" fn SetProcessDataProcW(hArcData: HANDLE, pProcessDataProc: tProcessDataProcW) {}


/// PackFiles specifies what should happen when a user creates,
/// or adds files to the archive.
#[no_mangle]
pub extern "stdcall" fn PackFiles(PackedFile: *mut c_char, SubPath: *mut c_char, SrcPath: *mut c_char, AddList: *mut c_char, Flags: c_int) -> c_int {
    0
}
#[no_mangle]
pub extern "stdcall" fn PackFilesW(PackedFile: *mut WCHAR, SubPath: *mut WCHAR, SrcPath: *mut WCHAR, AddList: *mut WCHAR, Flags: c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "stdcall" fn DeleteFiles(PackedFile: *mut c_char, DeleteList: *mut c_char) -> c_int {
    0
}
#[no_mangle]
pub extern "stdcall" fn DeleteFilesW(PackedFile: *mut WCHAR, DeleteList: *mut WCHAR) -> c_int {
    0
}

/// GetPackerCaps tells WinCmd what features your packer plugin supports
#[no_mangle]
pub extern "stdcall" fn GetPackerCaps() -> c_int {
    0
}

#[no_mangle]
pub extern "stdcall" fn CanYouHandleThisFile(FileName: *mut c_char) -> BOOL {
    FALSE
}
#[no_mangle]
pub extern "stdcall" fn CanYouHandleThisFileW(FileName: *mut WCHAR) -> BOOL {
    FALSE
}

#[no_mangle]
pub extern "stdcall" fn GetBackgroundFlags() -> c_int {
    BACKGROUND_UNPACK | BACKGROUND_PACK
}
