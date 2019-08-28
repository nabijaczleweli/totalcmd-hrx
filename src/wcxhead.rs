#![allow(nonstandard_style)]


//! Contents of file wcxhead.h
//! It contains definitions of error codes, flags and callbacks
//!
//! # Error Codes
//!
//! Use the following values when you want to inform Totalcmd that an error ocurred.
//!
//! | Constant         | Value | Description                                          |
//! | --------         | ----- | -----------                                          |
//! |                  | 0     | Success                                              |
//! | E_END_ARCHIVE    | 10    | No more files in archive                             |
//! | E_NO_MEMORY      | 11    | Not enough memory                                    |
//! | E_BAD_DATA       | 12    | CRC error in the data of the currently unpacked file |
//! | E_BAD_ARCHIVE    | 13    | The archive as a whole is bad, e.g. damaged headers  |
//! | E_UNKNOWN_FORMAT | 14    | Archive format unknown                               |
//! | E_EOPEN          | 15    | Cannot open existing file                            |
//! | E_ECREATE        | 16    | Cannot create file                                   |
//! | E_ECLOSE         | 17    | Error closing file                                   |
//! | E_EREAD          | 18    | Error reading from file                              |
//! | E_EWRITE         | 19    | Error writing to file                                |
//! | E_SMALL_BUF      | 20    | Buffer too small                                     |
//! | E_EABORTED       | 21    | Function aborted by user                             |
//! | E_NO_FILES       | 22    | No files found                                       |
//! | E_TOO_MANY_FILES | 23    | Too many files to pack                               |
//! | E_NOT_SUPPORTED  | 24    | Function not supported                               |
//!
//! # Unicode Support
//!
//! With Total Commander 7.5 (packer plugin interface 2.20), Unicode support has been added to all plugin types. In principle,
//! you need to implement the same functions as for ANSI, with two differences: The function name is changed from FunctionName
//! to FunctionNameW, and Ansi strings are changed to wide char names.
//!
//! Total Commander will call the Unicode functions on all NT-based systems (Windows NT, 2000, XP) if they are present. If not,
//! or on Windows 9x/ME, Total Commander will call the Ansi functions.
//!
//! The following functions of the packer plugin interface support Unicode:
//!
//! OpenArchiveW<br />
//! ReadHeaderExW<br />
//! ProcessFileW<br />
//! SetChangeVolProcW<br />
//! SetProcessDataProcW<br />
//! PackFilesW<br />
//! DeleteFilesW<br />
//! StartMemPackW<br />
//! CanYouHandleThisFileW<br />
//!
//! The following functions do not exist in a Unicode form and must be implemented as Ansi:
//!
//! ReadHeader - use ReadHeaderEx<br />
//! CloseArchive<br />
//! GetPackerCaps<br />
//! ConfigurePacker<br />
//! PackToMem<br />
//! DoneMemPack<br />
//! PackSetDefaultParams<br />
//! ReadHeaderEx
//!
//! What's the easiest way to support Unicode in an existing plugin?
//!
//! 1. Get my sample plugin fsplugin (file system plugins section) even if you write a different type of plugin!
//! 2. Add the files cunicode.h and cunicode.cpp to your project. They contain various functions to make Unicode support easier.
//! 3. Convert your existing functions to Unicode and rename them to FunctionNameW.
//!    For all file functions like CreateFile, do not call their Unicode counterpart CreateFileW directly.
//!    Instead, call the functions from cunicode.cpp like CreateFileT.
//!    These functions automatically call the right Unicode or Ansi function,
//!    and even support file name lengths >259 characters!
//!
//! 4. For each converted function like FunctionNameW, recreate a function FunctionName which you call this way:
//!
//! ```c
//! int __stdcall FunctionName(char* SomeString1,char* SomeString2)
//! {
//!
//! WCHAR SomeString1W[wdirtypemax],SomeString2W[wdirtypemax];
//!     return FunctionNameW(awfilenamecopy(SomeString1W,SomeString1),awfilenamecopy(SomeString2W,SomeString2));
//! }
//! ```
//!
//! The Macro awfilenamecopy will convert the Ansi string SomeString1 to Unicode and store it inSomeString1W. This variable
//! must not be a pointer, because awfilenamecopy uses "countof" to get the target length.
//!
//! # 64-bit support
//!
//! With Total Commander 8, Total Commander is now also available in 64-bit. Since plugins are simple dlls, and 64-bit programs
//! can only load 64-bit dlls, a plugin needs to be re-compiled with a 64-bit compiler to work with 64-bit Total Commander.
//!
//! **IMPORTANT**: A 64-bit plugin must have the same name as the 32-bit plugin and be in the same directory, but '64' must be
//! appended to the extension. Example: filesystem.wcx -> filesystem.wcx64. 64-bit-only plugins must also end with '64'.
//!
//! Since all 64-bit Windows versions support Unicode, it's sufficient to write a 64-bit Unicode-only plugin. However, if your
//! existing 32-bit plugin only supports ANSI functions, you can port it without modifications to 64 bit. Total Commander
//! 64-bit also supports the ANSI functions if it cannot find the Unicode functions. 64-bit Unicode plugins do not have an
//! extension starting with 'u', they have the normal 'wcx64' extension.
//!
//! ### Some porting notes:
//!
//! All integer parameters in plugin functions remain 32-bit (e.g. in C: int, long, DWORD; Delphi: integer, dword), only
//! pointers and handles are 64-bit wide now. Recompiling a program or dll with a 64-bit compiler usually takes care of this
//! automatically without needing many changes. Problems can arise when converting pointers to integer or vice versa. Make sure
//! to use 64-bit integer variables (e.g. size_t in C, ptrint or ptruint in Lazarus) for such operations.
//!
//! ### What's the easiest way to convert an existing plugin to 64-bit?
//!
//! #### 1. If the plugin was written in C or C++:
//!
//! If you have Visual Studio Professional 2005 or later, a 64-bit compiler is already included. If you use the free Express
//! version or Visual Studio 2003, you need to install the Windows Software Development Kit (SDK) in addition to Visual Studio
//! Express.
//!
//! Here is how to add 64-bit support to an existing plugin:
//! 1. Check the toolbars in Visual Studio: There are comboboxes for the compile type (Debug or Release), and one which shows
//! "Win32".
//!
//! 2. Open the one showing "Win32", and click on the "Configuration manager".
//! 3. Click on "New" in the Active Solution Platform list (right side). A new dialog box "New Solution Platform" will be shown.
//! 4. In the upper field, choose "x64"
//! 5. In the lower field, "copy from", choose "Win32"
//! 6. Make sure the checkbox "Create new project platform" is on
//! 7. Click OK<br />
//! See also: http://msdn.microsoft.com/en-us/library/9yb4317s.aspx
//! 8. Now you can switch in the same project between Win32 and x64. Switch to x64.
//! 9. Open the settings of your project.
//! 10. You should set the following options:<br />
//!     C++ - Code generation - runtime library: Multi-threaded-Debug (/Mtd) <- not multithreaded debug dll !<br />
//!     Linker - General - output file: wcx/pluginname.wcx64
//!
//! ##### Download links:
//! 1. Visual Studio Express C++ edition:<br />
//! http://www.microsoft.com/visualstudio/en-us/products/2010-editions/visual-cpp-express
//! 2. Windows Software Development Kit (SDK):<br />
//! http://msdn.microsoft.com/en-us/windows/bb980924.aspx
//!
//! #### 2. If the plugin was written in Delphi or Free Pascal:
//!
//! There is a 64-bit Lazarus/Free Pascal available, which can be used to create 64-bit dlls. Total Commander itself was
//! compiled with Lazarus as a 64-bit application. There are menu items in the "Tools" menu to convert Delphi projects and
//! forms to Lazarus.
//!
//! Lazarus/Free Pascal works a bit differently from Delphi, so some functions may need to be changed. Here are the problems
//! encountered when porting Total Commander:
//!
//! 1. Free pascal is different -> Use {$MODE Delphi} in all *.pas files to handle functions the Delphi way
//! 2. strnew creates a NIL pointer when the passed pchar is 0 bytes long. -> Use your own strnew function.
//! 3. Windows messages below WM_USER are not passed to the windows procedure. -> Use SetWindowLongPtr to subclass the window
//! 4. The calculation  p-buffer  is not working when p is a pwidechar, buffer an array of widechar -> use p-pwidechar(@bufffer)
//! 5. INVALID_HANDLE_VALUE is incorrectly set to 0 instead of -1 in lcltype.pp! -> Put "windows" at end of "uses" command.
//!
//! ##### Download links:
//! You should download and install the 64-bit daily snapshot from<br />
//! http://www.lazarus.freepascal.org/<br />
//! Click on "Daily snapshots", then on e.g. Lazarus + fpc 2.4.4     **win64**<br />
//! The final releases are very outdated, so the snapshots usually work much better.


use winapi::shared::minwindef::{MAX_PATH, DWORD};
use libc::{c_char, c_uint, c_int};
use winapi::shared::ntdef::WCHAR;


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

/// tHeaderData is a structure used in `ReadHeader`.
///
/// ```c
/// typedef struct {
///
///     char ArcName[260];
///     char FileName[260];
///     int Flags;
///     int PackSize;
///     int UnpSize;
///     int HostOS;
///     int FileCRC;
///     int FileTime;
///     int UnpVer;
///     int Method;
///     int FileAttr;
///     char* CmtBuf;
///     int CmtBufSize;
///     int CmtSize;
///     int CmtState;
///   } tHeaderData;
/// ```
///
/// # Description
///
/// `ArcName`, `FileName`, `PackSize`, `UnpSize` contain the name of the archive, the name of the file within the archive, size
/// of the file when packed, and the size of the file when extracted, respectively.
///
/// `HostOS` is there for compatibility with unrar.dll only, and should be set to zero.
///
/// `FileCRC` is the 32-bit CRC (cyclic redundancy check) checksum of the file. If not available, set to zero.
///
/// The `Cmt*` values can be used to transfer file comment information. They are currently not used in Total Commander, so they
/// may be set to zero.
///
/// `FileAttr` can be set to any combination of the following values:
///
/// | Value | Description    |
/// | ----- | -----------    |
/// | 0x1   | Read-only file |
/// | 0x2   | Hidden file    |
/// | 0x4   | System file    |
/// | 0x8   | Volume ID file |
/// | 0x10  | Directory      |
/// | 0x20  | Archive file   |
/// | 0x3F  | Any file       |
///
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

/// tHeaderDataEx is a structure used in ReadHeaderEx.
///
/// ```c
/// typedef struct {
///
///     char ArcName[1024];
///     char FileName[1024];
///     int Flags;
///     unsigned int PackSize;
///     unsigned int PackSizeHigh;
///     unsigned int UnpSize;
///     unsigned int UnpSizeHigh;
///     int HostOS;
///     int FileCRC;
///     int FileTime;
///     int UnpVer;
///     int Method;
///     int FileAttr;
///     char* CmtBuf;
///     int CmtBufSize;
///     int CmtSize;
///     int CmtState;
///     char Reserved[1024];
///   } tHeaderDataEx;
/// ```
///
/// # Description
///
/// `ArcName`, `FileName`, `PackSize`, `UnpSize` contain the name of the archive, the name of the file within the archive, size
/// of the file when packed, and the size of the file when extracted, respectively. `PackSizeHigh`, `UnpSizeHigh` contain the
/// upper
/// 32 bit of a 64-bit size number. Set to 0 if the file is smaller than 4 GB.
///
/// `HostOS` is there for compatibility with unrar.dll only, and should be set to zero.
///
/// `FileCRC` is the 32-bit CRC (cyclic redundancy check) checksum of the file. If not available, set to zero.
///
/// The `Cmt*` values can be used to transfer file comment information. They are currently not used in Total Commander, so they
/// may be set to zero.
///
/// `FileAttr` can be set to any combination of the following values:
///
/// | Value | Description    |
/// | ----- | -----------    |
/// | 0x1   | Read-only file |
/// | 0x2   | Hidden file    |
/// | 0x4   | System file    |
/// | 0x8   | Volume ID file |
/// | 0x10  | Directory      |
/// | 0x20  | Archive file   |
/// | 0x3F  | Any file       |
///
/// FileTime contains the date and the time of the file’s last update. Use the following algorithm to set the value:
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
/// `Reserved` may be used in the future for additional data - you MUST set it to 0 for now to avoid problems with future
/// versions of TC.
///
/// Note:
///
/// The Unicode version of this structure uses WCHAR[1024] for ArcName and FileName. "Reserved" is unchanged.
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

/// tOpenArchiveData is used in `OpenArchive`.
///
/// ```c
/// typedef struct {
///
///     char* ArcName;
///     int OpenMode;
///     int OpenResult;
///     char* CmtBuf;
///     int CmtBufSize;
///     int CmtSize;
///     int CmtState;
///   } tOpenArchiveData;
/// ```
///
/// # Description
///
/// `ArcName` contains the name of the archive to open.
///
/// `OpenMode` is set to one of the following values:
///
/// | Constant      | Value | Description                                |
/// | --------      | ----- | -----------                                |
/// | PK_OM_LIST    | 0     | Open file for reading of file names only   |
/// | PK_OM_EXTRACT | 1     | Open file for processing (extract or test) |
///
///
/// `OpenResult` used to return one of the [error values](./#error-codes) if an error occurs.
///
/// The `Cmt*` variables are for the file comment. They are currently not used by Total Commander, so may be set to NULL.
///
/// Notes:
///
/// If the file is opened with OpenMode==PK_OM_LIST, `ProcessFile` will never be called by Total Commander.
///
/// The [Unicode](./#unicode-support) version of this function uses WCHAR* instead of char* for the text fields.
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

/// PackDefaultParamStruct is passed to `PackSetDefaultParams` to inform the plugin about the current plugin interface version
/// and ini file location.
///
/// # Declaration:
///
/// ```c
/// typedef struct {
///
/// int size;
///     DWORD PluginInterfaceVersionLow;
///     DWORD PluginInterfaceVersionHi;
///     char DefaultIniName[MAX_PATH];
/// } PackDefaultParamStruct;
/// ```
///
/// # Description of struct members:
///
/// <table>
/// <tr><td>size</td>
///     <td>The size of the structure, in bytes. Later revisions of the plugin interface may add more structure members, and
///         will adjust this size field accordingly.</td></tr>
///
/// <tr><td>PluginInterfaceVersionLow</td>
///     <td>Low value of plugin interface version. This is the value after the comma, multiplied by 100!
///         Example. For plugin interface version 2.1, the low DWORD is 10 and the high DWORD is 2..</td></tr>
///
/// <tr><td>PluginInterfaceVersionHi</td>
///     <td>High value of plugin interface version.</td></tr>
///
/// <tr><td>DefaultIniName</td>
///     <td>Suggested location+name of the ini file where the plugin could store its data. This is a fully
/// qualified path+file name, and will be in the same directory as the wincmd.ini. It's recommended to store the plugin
/// data in
/// this file or at least in this directory, because the plugin directory or the Windows directory may not be
/// writable!.</td></tr>
/// </table>
#[repr(C)]
pub struct PackDefaultParamStruct {
    pub size: c_int,
    pub PluginInterfaceVersionLow: DWORD,
    pub PluginInterfaceVersionHi: DWORD,
    pub DefaultIniName: [c_char; MAX_PATH],
}

/* Definition of callback functions called by the DLL */

/// tChangeValueProc is a typedef of the function that asks the user to change volume.
///
/// ```c
/// typedef int (__stdcall *tChangeVolProc)(char *ArcName, int Mode);
/// ```
///
/// # Description
///
/// `SetChangeVolProc` has provided you with a pointer to a function with this declaration. When you want the user to be asked
/// about changing volume, call this function with appropriate parameters. The function itself is part of Totalcmd - you only
/// specify the question. Totalcmd then asks the user, and you get the answer as the result of the call to this function. If
/// the user has aborted the operation, the function returns zero.
///
/// `ArcName` specifies the filename of the archive that you are processing, and will receive the name of the next volume.
///
/// Set `Mode` to one of the following values, according to what you want Totalcmd to ask the user:
///
/// | Constant      | Value | Description                                  |
/// | --------      | ----- | -----------                                  |
/// | PK_VOL_ASK    | 0     | Ask user for location of next volume         |
/// | PK_VOL_NOTIFY | 1     | Notify app that next volume will be unpacked |
///
/// Note
///
/// The keyword or constant __stdcall must be set according to the compiler that you will use to make the library. For example,
/// this is STDCALL for cygwin and __stdcall for MSC.
pub type tChangeVolProc = extern "stdcall" fn(ArcName: *mut char, Mode: c_int) -> c_int;
pub type tChangeVolProcW = extern "stdcall" fn(ArcName: *mut WCHAR, Mode: c_int) -> c_int;

/// tProcessDataProc is a typedef of the function that notifies the user about the progress when un/packing files.
///
/// ```c
/// typedef int (__stdcall *tProcessDataProc)(char *FileName, int Size);
/// ```
///
/// # Description
///
/// `SetProcessDataProc` has provided you with a pointer to a function with this declaration. When you want to notify the user
/// about the progress when un/packing files, call this function with appropriate parameters. The function itself is part of
/// Totalcmd - you only specify what Totalcmd should display. In addition, Totalcmd displays the Cancel button that allows the
/// user to abort the un/packing process. If the user has clicked on Cancel, the function returns zero.
///
/// `FileName` can be used to pass a pointer to the currently processed filename (0 terminated string), or NULL if it is not
/// available.
///
/// Set `Size` to the number of bytes processed since the previous call to the function. For plugins which unpack in
/// CloseArchive: Set size to negative percent value (-1..-100) to directly set first percent bar, -1000..-1100 for second
/// percent bar (-1000=0%).
///
/// # Note
///
/// The keyword or constant __stdcall must be set according to the compiler that you will use to make the library. For example,
/// this is STDCALL for cygwin and __stdcall for MSC.
pub type tProcessDataProc = extern "stdcall" fn(FileName: *mut char, Size: c_int) -> c_int;
pub type tProcessDataProcW = extern "stdcall" fn(FileName: *mut WCHAR, Size: c_int) -> c_int;

/// PkCryptProc is a callback function, which the plugin can call to store passwords in the secure password store, read them
/// back, or copy them to a new connection.
///
/// # Declaration:
///
/// ```c
/// int __stdcall PkCryptProc(int CryptoNumber,int mode,
///          char* ArchiveName,char* Password,int maxlen);
/// ```
///
/// # Description of parameters:
///
/// <table>
/// <tr><td>CryptoNumber</td>
///     <td>Here the plugin needs to pass the crypto number received through the `PkSetCryptCallback()` function.</td></tr>
///
/// <tr><td>mode</td>
///     <td>The mode of operation:<br />
///     PK_CRYPT_SAVE_PASSWORD: Save password to password store<br />
///     PK_CRYPT_LOAD_PASSWORD: Load password from password store<br />
///     PK_CRYPT_LOAD_PASSWORD_NO_UI: Load password only if master password has already been entered<br />
///     PK_CRYPT_COPY_PASSWORD: Copy password to new connection. Here the second string parameter "Password" is not a password,
///     but the name of the target archive name<br />
///     PK_CRYPT_MOVE_PASSWORD: As above, but delete the source password<br />
///     PK_CRYPT_DELETE_PASSWORD: Delete the password of the given archive name</td></tr>
///
/// <tr><td>ArchiveName</td>
///     <td>Name of the archive for this operation. The plugin can give any name here which can be stored in Windows ini
///         files. The plugin should encode names which cannot be stored in ini files, or give a reference code or so instead
///         of the file name.</td></tr>
///
/// <tr><td>Password</td>
///     <td>Operation-specific, usually the password to be stored/retrieved, or the target name when copying/moving a
///         connection</td></tr>
///
/// <tr><td>maxlen</td>
///     <td>Maximum length, in characters, the password buffer can store when calling one of the load functions</td></tr>
/// </table>
///
/// # Return value:
///
/// Total Commander returns one of these values:
///
/// <table>
/// <tr><td>FS_FILE_OK</td><td>Success</td></tr>
/// <tr><td>E_ECREATE</td> <td>Encrypt/Decrypt failed</td></tr>
/// <tr><td>E_EWRITE</td>  <td>Could not write password to password store</td></tr>
/// <tr><td>E_EREAD</td>   <td>Password not found in password store</td></tr>
/// <tr><td>E_NO_FILES</td><td>No master password entered yet</td></tr>
/// </table>
///
/// # Note:
///
/// When showing the details of an existing archive, you should call PK_CRYPT_LOAD_PASSWORD_NO_UI first. In case of error
/// E_NO_FILES, show a button "Edit password". Only call PK_CRYPT_LOAD_PASSWORD when the user clicks that button, or tries to
/// decrypt the archive. This way the user doesn't have to enter the master password if he just wanted to make some other
/// changes to the archive settings.
pub type tPkCryptProc = extern "stdcall" fn(CryptoNr: c_int, Mode: c_int, ArchiveName: *mut char, Password: *mut char, maxlen: c_int) -> c_int;
pub type tPkCryptProcW = extern "stdcall" fn(CryptoNr: c_int, Mode: c_int, ArchiveName: *mut WCHAR, Password: *mut WCHAR, maxlen: c_int) -> c_int;
