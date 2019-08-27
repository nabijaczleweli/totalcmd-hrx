#![allow(nonstandard_style)]
#![crate_type = "dylib"]

extern crate winapi;
extern crate libc;
extern crate hrx;

pub mod wcxhead;

use wcxhead::{PackDefaultParamStruct, tOpenArchiveDataW, tOpenArchiveData, tProcessDataProcW, tProcessDataProc, tChangeVolProcW, tChangeVolProc, tHeaderDataExW,
              tHeaderDataEx, tPkCryptProcW, tPkCryptProc, tHeaderData};
use winapi::shared::minwindef::{HINSTANCE, FALSE, BOOL};
use winapi::shared::ntdef::HANDLE;
use winapi::shared::ntdef::WCHAR;
use winapi::shared::windef::HWND;
use libc::{c_char, c_int};
use std::ptr;


/// OpenArchive should perform all necessary operations
/// when an archive is to be opened
pub extern "stdcall" fn OpenArchive(ArchiveData: *mut tOpenArchiveData) -> HANDLE {
    ptr::null_mut()
}

pub extern "stdcall" fn OpenArchiveW(ArchiveData: *mut tOpenArchiveDataW) -> HANDLE {
    ptr::null_mut()
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

/// CloseArchive should perform all necessary operations
/// when an archive is about to be closed.
pub extern "stdcall" fn CloseArchive(hArcData: HANDLE) -> c_int {
    0
}


/// This function allows you to notify user
/// about changing a volume when packing files
pub extern "stdcall" fn SetChangeVolProc(hArcData: HANDLE, pChangeVolProc1: tChangeVolProc) {}
pub extern "stdcall" fn SetChangeVolProcW(hArcData: HANDLE, pChangeVolProc1: tChangeVolProcW) {}

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

pub extern "stdcall" fn DeleteFiles(PackedFile: *mut c_char, DeleteList: *mut c_char) -> c_int {}
pub extern "stdcall" fn DeleteFilesW(PackedFile: *mut WCHAR, DeleteList: *mut WCHAR) -> c_int {}

/// GetPackerCaps tells WinCmd what features your packer plugin supports
pub extern "stdcall" fn GetPackerCaps() -> c_int {
    0
}

/// ConfigurePacker gets called when the user clicks the Configure button
/// from within "Pack files..." dialog box in WinCmd
pub extern "stdcall" fn ConfigurePacker(Parent: HWND, DllInstance: HINSTANCE) {}

pub extern "stdcall" fn StartMemPack(Options: c_int, FileName: *mut c_char) -> HANDLE {
    ptr::null_mut()
}
pub extern "stdcall" fn StartMemPackW(Options: c_int, FileName: *mut WCHAR) -> HANDLE {
    ptr::null_mut()
}

pub extern "stdcall" fn PackToMem(hMemPack: HANDLE, BufIn: *mut c_char, InLen: c_int, Taken: *mut c_int, BufOut: *mut c_char, OutLen: c_int,
                                  Written: *mut c_int, SeekBy: c_int)
                                  -> c_int {
    0
}

pub extern "stdcall" fn DoneMemPack(hMemPack: HANDLE) -> c_int {
    0
}

pub extern "stdcall" fn CanYouHandleThisFile(FileName: *mut c_char) -> BOOL {
    FALSE
}
pub extern "stdcall" fn CanYouHandleThisFileW(FileName: *mut WCHAR) -> BOOL {
    FALSE
}

pub extern "stdcall" fn PackSetDefaultParams(dps: *mut PackDefaultParamStruct) {}

pub extern "stdcall" fn PkSetCryptCallback(pPkCryptProc: tPkCryptProc, CryptoNr: c_int, Flags: c_int) {}
pub extern "stdcall" fn PkSetCryptCallbackW(pPkCryptProc: tPkCryptProcW, CryptoNr: c_int, Flags: c_int) {}

pub extern "stdcall" fn GetBackgroundFlags() -> c_int {
    0
}
