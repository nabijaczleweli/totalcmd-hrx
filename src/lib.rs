#![allow(nonstandard_style)]
#![crate_type = "dylib"]

extern crate winapi;
extern crate libc;
extern crate hrx;

mod state;

pub mod wcxhead;

use wcxhead::{tOpenArchiveDataW, tOpenArchiveData, tProcessDataProcW, tProcessDataProc, tChangeVolProcW, tChangeVolProc, tHeaderDataExW, tHeaderDataEx,
              tHeaderData, BACKGROUND_UNPACK, BACKGROUND_PACK};
use winapi::shared::minwindef::{FALSE, BOOL};
use winapi::shared::ntdef::{HANDLE, WCHAR};
use std::os::windows::ffi::OsStringExt;
use libc::{c_char, c_int, wcslen};
use std::ffi::{OsString, CStr};
use std::{slice, ptr};
use std::path::Path;

pub use state::ArchiveState;


/// OpenArchive should perform all necessary operations when an archive is to be opened
pub unsafe extern "stdcall" fn OpenArchive(ArchiveData: *mut tOpenArchiveData) -> HANDLE {
    let ArchiveData = &mut *ArchiveData;

    OpenArchiveImpl(&CStr::from_ptr(ArchiveData.ArcName).to_string_lossy()[..], &mut ArchiveData.OpenResult)
}

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

/// WinCmd calls ReadHeader to find out what files are in the archive
pub extern "stdcall" fn ReadHeader(hArcData: HANDLE, HeaderData: *mut tHeaderData) -> c_int {
    0
}

/// WinCmd calls ReadHeaderEx to find out what files are in the archive
/// It is called if the supported archive type may contain files >2 GB.
pub extern "stdcall" fn ReadHeaderEx(hArcData: HANDLE, HeaderDataEx: *mut tHeaderDataEx) -> c_int {
    0
}
pub extern "stdcall" fn ReadHeaderExW(hArcData: HANDLE, HeaderDataEx: *mut tHeaderDataExW) -> c_int {
    0
}

/// ProcessFile should unpack the specified file
/// or test the integrity of the archive
pub extern "stdcall" fn ProcessFile(hArcData: HANDLE, Operation: c_int, DestPath: *mut c_char, DestName: *mut c_char) -> c_int {
    0
}
pub extern "stdcall" fn ProcessFileW(hArcData: HANDLE, Operation: c_int, DestPath: *mut WCHAR, DestName: *mut WCHAR) -> c_int {
    0
}

/// CloseArchive should perform all necessary operations when an archive is about to be closed.
pub unsafe extern "stdcall" fn CloseArchive(hArcData: HANDLE) -> c_int {
    Box::from_raw(hArcData as *mut ArchiveState);

    0
}

/// HRX archives are single-volume, safe to ignore
pub extern "stdcall" fn SetChangeVolProc(_: HANDLE, _: tChangeVolProc) {}
pub extern "stdcall" fn SetChangeVolProcW(_: HANDLE, _: tChangeVolProcW) {}

/// This function allows you to notify user about
/// the progress when you un/pack files
pub extern "stdcall" fn SetProcessDataProc(hArcData: HANDLE, pProcessDataProc: tProcessDataProc) {}
pub extern "stdcall" fn SetProcessDataProcW(hArcData: HANDLE, pProcessDataProc: tProcessDataProcW) {}


/// PackFiles specifies what should happen when a user creates,
/// or adds files to the archive.
pub extern "stdcall" fn PackFiles(PackedFile: *mut c_char, SubPath: *mut c_char, SrcPath: *mut c_char, AddList: *mut c_char, Flags: c_int) -> c_int {
    0
}
pub extern "stdcall" fn PackFilesW(PackedFile: *mut WCHAR, SubPath: *mut WCHAR, SrcPath: *mut WCHAR, AddList: *mut WCHAR, Flags: c_int) -> c_int {
    0
}

pub extern "stdcall" fn DeleteFiles(PackedFile: *mut c_char, DeleteList: *mut c_char) -> c_int {
    0
}
pub extern "stdcall" fn DeleteFilesW(PackedFile: *mut WCHAR, DeleteList: *mut WCHAR) -> c_int {
    0
}

/// GetPackerCaps tells WinCmd what features your packer plugin supports
pub extern "stdcall" fn GetPackerCaps() -> c_int {
    0
}

pub extern "stdcall" fn CanYouHandleThisFile(FileName: *mut c_char) -> BOOL {
    FALSE
}
pub extern "stdcall" fn CanYouHandleThisFileW(FileName: *mut WCHAR) -> BOOL {
    FALSE
}

pub extern "stdcall" fn GetBackgroundFlags() -> c_int {
    BACKGROUND_UNPACK | BACKGROUND_PACK
}
