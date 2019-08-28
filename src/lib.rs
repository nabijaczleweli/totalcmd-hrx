#![allow(nonstandard_style)]

extern crate linked_hash_map;
extern crate num_traits;
extern crate winapi;
extern crate libc;
extern crate hrx;

mod pack;
mod state;

pub mod util;
pub mod wcxhead;

use wcxhead::{tOpenArchiveDataW, tOpenArchiveData, tProcessDataProcW, tProcessDataProc, tChangeVolProcW, tChangeVolProc, tHeaderDataExW, tHeaderDataEx,
              tHeaderData, BACKGROUND_UNPACK, BACKGROUND_PACK, E_NOT_SUPPORTED, E_END_ARCHIVE, PK_EXTRACT, PK_SKIP, PK_TEST};
use libc::{c_char, c_uint, c_int, strncpy, wcslen, INT_MAX};
use self::util::{CListIter, system_time_to_totalcmd_time};
use std::os::windows::ffi::{OsStringExt, OsStrExt};
use winapi::shared::minwindef::{FALSE, BOOL};
use winapi::shared::ntdef::{HANDLE, WCHAR};
use std::ffi::{OsString, OsStr, CStr};
use std::convert::TryInto;
use hrx::HrxEntryData;
use std::{slice, ptr};
use std::path::Path;

pub use self::pack::{modify_archive, pack_archive};
pub use self::state::{ArchiveState, GLOBAL_PROCESS_DATA_CALLBACK_W, GLOBAL_PROCESS_DATA_CALLBACK};


/// OpenArchive should perform all necessary operations when an archive is to be opened.
///
/// ```c
/// HANDLE __stdcall OpenArchive (tOpenArchiveData *ArchiveData);
/// ```
///
/// # Description
///
/// OpenArchive should return a unique handle representing the archive. The handle should remain valid until
/// [CloseArchive](fn.CloseArchive.html) is called.
/// If an [error](wcxhead/#error-codes) occurs, you should return zero, and specify the [error](wcxhead/#error-codes) by
/// setting OpenResult member of ArchiveData.
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
/// being called again. If an error occurs, ReadHeader should return one of the [error values](wcxhead/#error-codes)
/// or 0 for no error.
///
/// `hArcData` contains the handle returned by [`OpenArchive`](fn.OpenArchive.html). The programmer is encouraged to store
/// other information in the location that can be accessed via this handle. For example, you may want to store the position in
/// the archive when returning files information in ReadHeader.
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
/// ReaderHeaderEx from being called again. If an error occurs, ReadHeaderEx should return one of the
/// [error values](wcxhead/#error-codes) or 0 for no error.
///
/// `hArcData` contains the handle returned by [`OpenArchive`](fn.OpenArchive.html). The programmer is encouraged to store
/// other information in the
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


/// ProcessFile should unpack the specified file or test the integrity of the archive.
///
/// ```c
/// int __stdcall ProcessFile (HANDLE hArcData, int Operation, char *DestPath, char *DestName);
/// ```
///
/// # Description
///
/// ProcessFile should return zero on success, or one of the [error values](wcxhead/#error-codes) otherwise.
///
/// `hArcData` contains the handle previously returned by you in [`OpenArchive`](fn.OpenArchive.html). Using this, you should
/// be able to find out
/// information (such as the archive filename) that you need for extracting files from the archive.
///
/// Unlike [`PackFiles`](fn.PackFiles.html), ProcessFile is passed only one filename. Either `DestName` contains the full path
/// and file name and
/// `DestPath` is NULL, or `DestName` contains only the file name and `DestPath` the file path. This is done for compatibility
/// with unrar.dll.
///
/// When Total Commander first opens an archive, it scans all file names with OpenMode==PK_OM_LIST, so ReadHeader() is called
/// in a loop with calling ProcessFile(...,PK_SKIP,...). When the user has selected some files and started to decompress them,
/// Total Commander again calls ReadHeader() in a loop. For each file which is to be extracted, Total Commander calls
/// ProcessFile() with Operation==PK_EXTRACT immediately after the ReadHeader() call for this file. If the file needs to be
/// skipped, it calls it with Operation==PK_SKIP.
///
/// Each time `DestName` is set to contain the filename to be extracted, tested, or skipped. To find out what operation out of
/// these last three you should apply to the current file within the archive, `Operation` is set to one of the following:
///
/// Constant   | Value | Description
/// --------   | ----- | -----------
/// PK_SKIP    | 0     | Skip this file
/// PK_TEST    | 1     | Test file integrity
/// PK_EXTRACT | 2     | Extract to disk
#[no_mangle]
pub unsafe extern "stdcall" fn ProcessFile(hArcData: HANDLE, Operation: c_int, DestPath: *mut c_char, DestName: *mut c_char) -> c_int {
    let state = &*(hArcData as *mut ArchiveState);

    ProcessFileImpl(state,
                    Operation,
                    &CStr::from_ptr(DestPath).to_string_lossy()[..],
                    &CStr::from_ptr(DestName).to_string_lossy()[..])
}

#[no_mangle]
pub unsafe extern "stdcall" fn ProcessFileW(hArcData: HANDLE, Operation: c_int, DestPath: *mut WCHAR, DestName: *mut WCHAR) -> c_int {
    let state = &*(hArcData as *mut ArchiveState);

    ProcessFileImpl(state,
                    Operation,
                    OsString::from_wide(slice::from_raw_parts(DestPath, wcslen(DestPath))),
                    OsString::from_wide(slice::from_raw_parts(DestName, wcslen(DestName))))
}

fn ProcessFileImpl<Pd: AsRef<Path>, Pn: AsRef<Path>>(state: &ArchiveState, Operation: c_int, dest_path: Pd, dest_name: Pn) -> c_int {
    ProcessFileImpl_impl(state, Operation, dest_path.as_ref(), dest_name.as_ref())
}

fn ProcessFileImpl_impl(state: &ArchiveState, Operation: c_int, dest_path: &Path, dest_name: &Path) -> c_int {
    match Operation {
        PK_SKIP => 0,
        PK_TEST => 0,
        PK_EXTRACT => {
            match state.extract_current_entry(dest_path, dest_name) {
                Ok(()) => 0,
                Err(err) => err,
            }
        }
        _ => E_NOT_SUPPORTED,
    }
}


/// CloseArchive should perform all necessary operations when an archive is about to be closed.
///
/// ```c
/// int __stdcall CloseArchive (HANDLE hArcData);
/// ```
///
/// # Description
///
/// CloseArchive should return zero on success, or one of the [error values](wcxhead/#error-codes) otherwise. It should free
/// all the resources
/// associated with the open archive.
///
/// The parameter `hArcData` refers to the value returned by a programmer within a previous call to
/// [`OpenArchive`](fn.OpenArchive.html).
#[no_mangle]
pub unsafe extern "stdcall" fn CloseArchive(hArcData: HANDLE) -> c_int {
    Box::from_raw(hArcData as *mut ArchiveState);

    0
}


/// This function allows you to notify user about changing a volume when packing files.
///
/// ```c
/// void __stdcall SetChangeVolProc (HANDLE hArcData, tChangeVolProc pChangeVolProc1);
/// ```
///
/// # Description
///
/// `pChangeVolProc1` contains a pointer to a function that you may want to call when notifying user to change volume (e.g.
/// insterting another diskette). You need to store the value at some place if you want to use it; you can use `hArcData` that
/// you have returned by [`OpenArchive`](fn.OpenArchive.html) to identify that place.
#[no_mangle]
pub extern "stdcall" fn SetChangeVolProc(_: HANDLE, _: tChangeVolProc) {}

#[no_mangle]
pub extern "stdcall" fn SetChangeVolProcW(_: HANDLE, _: tChangeVolProcW) {}


/// This function allows you to notify user about the progress when you un/pack files.
///
/// ```c
/// void __stdcall SetProcessDataProc (HANDLE hArcData, tProcessDataProc pProcessDataProc);
/// ```
///
/// # Description
///
/// `pProcessDataProc` contains a pointer to a function that you may want to call when notifying user about the progress being
/// made when you pack or extract files from an archive. You need to store the value at some place if you want to use it; you
/// can use `hArcData` that you have returned by [`OpenArchive`](fn.OpenArchive.html) to identify that place.
#[no_mangle]
pub unsafe extern "stdcall" fn SetProcessDataProc(hArcData: HANDLE, pProcessDataProc: tProcessDataProc) {
    if hArcData.is_null() || (hArcData as usize).overflowing_add(1).1 {
        GLOBAL_PROCESS_DATA_CALLBACK = Some(pProcessDataProc);
    } else {
        let state = &mut *(hArcData as *mut ArchiveState);

        state.process_data_callback = Some(pProcessDataProc);
    }
}

#[no_mangle]
pub unsafe extern "stdcall" fn SetProcessDataProcW(hArcData: HANDLE, pProcessDataProc: tProcessDataProcW) {
    if hArcData.is_null() || (hArcData as usize).overflowing_add(1).1 {
        GLOBAL_PROCESS_DATA_CALLBACK_W = Some(pProcessDataProc);
    } else {
        let state = &mut *(hArcData as *mut ArchiveState);

        state.process_data_callback_w = Some(pProcessDataProc);
    }
}


/// PackFiles specifies what should happen when a user creates, or adds files to the archive.
///
/// ```c
/// int __stdcall PackFiles (char *PackedFile, char *SubPath, char *SrcPath, char *AddList, int Flags);
/// ```
///
/// # Description
///
/// PackFiles should return zero on success, or one of the [error values](wcxhead/#error-codes) otherwise.
///
/// `PackedFile` refers to the archive that is to be created or modified. The string contains the full path.
///
/// `SubPath` is either NULL, when the files should be packed with the paths given with the file names, or not NULL when they
/// should be placed below the given subdirectory within the archive. Example:
///
/// ```plaintext
/// SubPath="subdirectory"
/// Name in AddList="subdir2\filename.ext"
/// -> File should be packed as "subdirectory\subdir2\filename.ext"
/// ```
///
/// `SrcPath` contains path to the files in `AddList`. `SrcPath` and `AddList` together specify files that are to be packed into
/// `PackedFile`. Each string in `AddList` is zero-delimited (ends in zero), and the `AddList` string ends with an extra zero
/// byte, i.e. there are two zero bytes at the end of `AddList`.
///
/// `Flags` can contain a combination of the following values reflecting the user choice from within Totalcmd:
///
/// | Constant           | Value | Description                                                 |
/// | --------           | ----- | -----------                                                 |
/// | PK_PACK_MOVE_FILES | 1     | Delete original after packing                               |
/// | PK_PACK_SAVE_PATHS | 2     | Save path names of files                                    |
/// | PK_PACK_ENCRYPT    | 4     | Ask user for password, then encrypt file with that password |
#[no_mangle]
pub unsafe extern "stdcall" fn PackFiles(PackedFile: *mut c_char, SubPath: *mut c_char, SrcPath: *mut c_char, AddList: *mut c_char, Flags: c_int) -> c_int {
    match pack_archive(CStr::from_ptr(PackedFile).to_string_lossy().into_owned(),
                       if SubPath.is_null() {
                           None
                       } else {
                           Some(CStr::from_ptr(SubPath).to_string_lossy())
                       },
                       &CStr::from_ptr(SrcPath).to_string_lossy()[..],
                       CListIter(AddList)
                           .map(|s| CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(s.as_ptr() as *const u8, s.len() + 1)))
                           .map(|s| s.to_string_lossy()),
                       Flags) {
        Ok(()) => 0,
        Err(err) => err,
    }
}

#[no_mangle]
pub unsafe extern "stdcall" fn PackFilesW(PackedFile: *mut WCHAR, SubPath: *mut WCHAR, SrcPath: *mut WCHAR, AddList: *mut WCHAR, Flags: c_int) -> c_int {
    match pack_archive(OsString::from_wide(slice::from_raw_parts(PackedFile, wcslen(PackedFile))),
                       if SubPath.is_null() {
                           None
                       } else {
                           Some(OsString::from_wide(slice::from_raw_parts(SubPath, wcslen(SubPath)))
                               .into_string()
                               .unwrap_or_else(|s| s.to_string_lossy().into()))
                       },
                       OsString::from_wide(slice::from_raw_parts(SrcPath, wcslen(SrcPath))),
                       CListIter(AddList).map(OsString::from_wide).map(|s| s.into_string().unwrap_or_else(|s| s.to_string_lossy().into())),
                       Flags) {
        Ok(()) => 0,
        Err(err) => err,
    }
}


/// DeleteFiles should delete the specified files from the archive
///
/// ```c
/// int __stdcall DeleteFiles (char *PackedFile, char *DeleteList);
/// ```
///
/// # Description
///
/// DeleteFiles should return zero on success, or one of the [error values](wcxhead/#error-codes) otherwise.
///
/// `PackedFile` contains full path and name of the the archive.
///
/// `DeleteList` contains the list of files that should be deleted from the archive. The format of this string is the same as
/// `AddList` within [PackFiles](fn.PackFiles.html).
#[no_mangle]
pub extern "stdcall" fn DeleteFiles(PackedFile: *mut c_char, DeleteList: *mut c_char) -> c_int {
    match modify_archive(CStr::from_ptr(PackedFile).to_string_lossy().into_owned(),
                         CListIter(DeleteList)
                             .map(|s| CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(s.as_ptr() as *const u8, s.len() + 1)))
                             .map(|s| s.to_string_lossy())) {
        Ok(()) => 0,
        Err(err) => err,
    }
}

#[no_mangle]
pub extern "stdcall" fn DeleteFilesW(PackedFile: *mut WCHAR, DeleteList: *mut WCHAR) -> c_int {
    match modify_archive(OsString::from_wide(slice::from_raw_parts(PackedFile, wcslen(PackedFile))),
                         CListIter(DeleteList).map(OsString::from_wide).map(|s| s.into_string().unwrap_or_else(|s| s.to_string_lossy().into()))) {
        Ok(()) => 0,
        Err(err) => err,
    }
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
