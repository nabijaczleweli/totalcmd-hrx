#![allow(nonstandard_style)]


//! Contents of file wcxhead.h
//! It contains definitions of error codes, flags and callbacks


use winapi::shared::minwindef::{MAX_PATH, DWORD};
use libc::{c_char, c_uint, c_int};
use winapi::shared::ntdef::WCHAR;


/* Error codes returned to calling application */
/// No more files in archive
pub const E_END_ARCHIVE: c_int = 10;
/// Not enough memory
pub const E_NO_MEMORY: c_int = 11;
/// CRC error in the data of the currently unpacked file
pub const E_BAD_DATA: c_int = 12;
/// The archive as a whole is bad, e.g. damaged headers
pub const E_BAD_ARCHIVE: c_int = 13;
/// Archive format unknown
pub const E_UNKNOWN_FORMAT: c_int = 14;
/// Cannot open existing file
pub const E_EOPEN: c_int = 15;
/// Cannot create file
pub const E_ECREATE: c_int = 16;
/// Error closing file
pub const E_ECLOSE: c_int = 17;
/// Error reading from file
pub const E_EREAD: c_int = 18;
/// Error writing to file
pub const E_EWRITE: c_int = 19;
/// Buffer too small
pub const E_SMALL_BUF: c_int = 20;
/// Function aborted by user
pub const E_EABORTED: c_int = 21;
/// No files found
pub const E_NO_FILES: c_int = 22;
/// Too many files to pack
pub const E_TOO_MANY_FILES: c_int = 23;
/// Function not supported
pub const E_NOT_SUPPORTED: c_int = 24;

/* flags for unpacking */
pub const PK_OM_LIST: c_int = 0;
pub const PK_OM_EXTRACT: c_int = 1;

/* flags for ProcessFile */
/// Skip this file
pub const PK_SKIP: c_int = 0;
/// Test file integrity
pub const PK_TEST: c_int = 1;
/// Extract to disk
pub const PK_EXTRACT: c_int = 2;

/* Flags passed through ChangeVolProc */
/// Ask user for location of next volume
pub const PK_VOL_ASK: c_int = 0;
/// Notify app that next volume will be unpacked
pub const PK_VOL_NOTIFY: c_int = 1;

/* Flags for packing */

/* For PackFiles */
/// Delete original after packing
pub const PK_PACK_MOVE_FILES: c_int = 1;
/// Save path names of files
pub const PK_PACK_SAVE_PATHS: c_int = 2;
/// Ask user for password, then encrypt
pub const PK_PACK_ENCRYPT: c_int = 4;

/* Returned by GetPackCaps */
/// Can create new archives
pub const PK_CAPS_NEW: c_int = 1;
/// Can modify exisiting archives
pub const PK_CAPS_MODIFY: c_int = 2;
/// Archive can contain multiple files
pub const PK_CAPS_MULTIPLE: c_int = 4;
/// Can delete files
pub const PK_CAPS_DELETE: c_int = 8;
/// Has options dialog
pub const PK_CAPS_OPTIONS: c_int = 16;
/// Supports packing in memory
pub const PK_CAPS_MEMPACK: c_int = 32;
/// Detect archive type by content
pub const PK_CAPS_BY_CONTENT: c_int = 64;
/// Allow searching for text in archives created with this plugin
pub const PK_CAPS_SEARCHTEXT: c_int = 128;
/// Show as normal files (hide packer icon), open with Ctrl+PgDn, not Enter
pub const PK_CAPS_HIDE: c_int = 256;
/// Plugin supports PK_PACK_ENCRYPT option
pub const PK_CAPS_ENCRYPT: c_int = 512;

/* Which operations are thread-safe? */
pub const BACKGROUND_UNPACK: c_int = 1;
pub const BACKGROUND_PACK: c_int = 2;
pub const BACKGROUND_MEMPACK: c_int = 4;

/* Flags for packing in memory */
/// Return archive headers with packed data
pub const MEM_OPTIONS_WANTHEADERS: c_int = 1;

/* Errors returned by PackToMem */
/// Function call finished OK, but there is more data
pub const MEMPACK_OK: c_int = 0;
/// Function call finished OK, there is no more data
pub const MEMPACK_DONE: c_int = 1;

pub const PK_CRYPT_SAVE_PASSWORD: c_int = 1;
pub const PK_CRYPT_LOAD_PASSWORD: c_int = 2;
/// Load password only if master password has already been entered!
pub const PK_CRYPT_LOAD_PASSWORD_NO_UI: c_int = 3;
/// Copy encrypted password to new archive name
pub const PK_CRYPT_COPY_PASSWORD: c_int = 4;
/// Move password when renaming an archive
pub const PK_CRYPT_MOVE_PASSWORD: c_int = 5;
/// Delete password
pub const PK_CRYPT_DELETE_PASSWORD: c_int = 6;

/// The user already has a master password defined
pub const PK_CRYPTOPT_MASTERPASS_SET: c_int = 1;

#[repr(C)]
pub struct tHeaderData {
    pub ArcName: [c_char; 260],
    pub FileName: [c_char; 260],
    pub Flags: c_int,
    pub PackSize: c_int,
    pub UnpSize: c_int,
    pub HostOS: c_int,
    pub FileCRC: c_int,
    pub FileTime: c_int,
    pub UnpVer: c_int,
    pub Method: c_int,
    pub FileAttr: c_int,
    pub CmtBuf: *mut c_char,
    pub CmtBufSize: c_int,
    pub CmtSize: c_int,
    pub CmtState: c_int,
}

#[repr(C)]
pub struct tHeaderDataEx {
    pub ArcName: [c_char; 1024],
    pub FileName: [c_char; 1024],
    pub Flags: c_int,
    pub PackSize: c_uint,
    pub PackSizeHigh: c_uint,
    pub UnpSize: c_uint,
    pub UnpSizeHigh: c_uint,
    pub HostOS: c_int,
    pub FileCRC: c_int,
    pub FileTime: c_int,
    pub UnpVer: c_int,
    pub Method: c_int,
    pub FileAttr: c_int,
    pub CmtBuf: *mut c_char,
    pub CmtBufSize: c_int,
    pub CmtSize: c_int,
    pub CmtState: c_int,
    pub Reserved: [c_char; 1024],
}

#[repr(C)]
pub struct tHeaderDataExW {
    pub ArcName: [WCHAR; 1024],
    pub FileName: [WCHAR; 1024],
    pub Flags: c_int,
    pub PackSize: c_uint,
    pub PackSizeHigh: c_uint,
    pub UnpSize: c_uint,
    pub UnpSizeHigh: c_uint,
    pub HostOS: c_int,
    pub FileCRC: c_int,
    pub FileTime: c_int,
    pub UnpVer: c_int,
    pub Method: c_int,
    pub FileAttr: c_int,
    pub CmtBuf: *mut c_char,
    pub CmtBufSize: c_int,
    pub CmtSize: c_int,
    pub CmtState: c_int,
    pub Reserved: [c_char; 1024],
}

#[repr(C)]
pub struct tOpenArchiveData {
    pub ArcName: *mut c_char,
    pub OpenMode: c_int,
    pub OpenResult: c_int,
    pub CmtBuf: *mut c_char,
    pub CmtBufSize: c_int,
    pub CmtSize: c_int,
    pub CmtState: c_int,
}

#[repr(C)]
pub struct tOpenArchiveDataW {
    pub ArcName: *mut WCHAR,
    pub OpenMode: c_int,
    pub OpenResult: c_int,
    pub CmtBuf: *mut WCHAR,
    pub CmtBufSize: c_int,
    pub CmtSize: c_int,
    pub CmtState: c_int,
}

#[repr(C)]
pub struct PackDefaultParamStruct {
    pub size: c_int,
    pub PluginInterfaceVersionLow: DWORD,
    pub PluginInterfaceVersionHi: DWORD,
    pub DefaultIniName: [c_char; MAX_PATH],
}

/* Definition of callback functions called by the DLL */

/// Ask to swap disk for multi-volume archive
pub type tChangeVolProc = extern "stdcall" fn(ArcName: *mut char, Mode: c_int) -> c_int;
pub type tChangeVolProcW = extern "stdcall" fn(ArcName: *mut WCHAR, Mode: c_int) -> c_int;

/* Notify that data is processed - used for progress dialog */
pub type tProcessDataProc = extern "stdcall" fn(FileName: *mut char, Size: c_int) -> c_int;
pub type tProcessDataProcW = extern "stdcall" fn(FileName: *mut WCHAR, Size: c_int) -> c_int;
pub type tPkCryptProc = extern "stdcall" fn(CryptoNr: c_int, Mode: c_int, ArchiveName: *mut char, Password: *mut char, maxlen: c_int) -> c_int;
pub type tPkCryptProcW = extern "stdcall" fn(CryptoNr: c_int, Mode: c_int, ArchiveName: *mut WCHAR, Password: *mut WCHAR, maxlen: c_int) -> c_int;
