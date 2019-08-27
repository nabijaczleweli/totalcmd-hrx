#![crate_type = "dylib"]

extern crate winapi;
extern crate libc;
extern crate hrx;

pub mod wcxhead;

/*
/// OpenArchive should perform all necessary operations
/// when an archive is to be opened
    extern "stdcall" fn OpenArchive (
        ArchiveData : *mut tOpenArchiveData
        ) -> HANDLE {}

/// WinCmd calls ReadHeaderEx to find out what files are in the archive
/// It is called if the supported archive type may contain files >2 GB.
    extern "stdcall" fn ReadHeaderEx (
        hArcData: HANDLE ,
        HeaderDataEx : tHeaderDataEx *
        ) -> int {}

/// WinCmd calls ReadHeader to find out what files are in the archive
    extern "stdcall" fn ReadHeader (
        HANDLE hArcData,
        tHeaderData *HeaderData
        ) -> int {}

/// ProcessFile should unpack the specified file
/// or test the integrity of the archive
    extern "stdcall" fn ProcessFile (
        HANDLE hArcData,
        int Operation,
        char *DestPath,
        char *DestName
        ) -> int {}

/// CloseArchive should perform all necessary operations
/// when an archive is about to be closed.
    extern "stdcall" fn CloseArchive (
        HANDLE hArcData
        ) -> int {}

/// This function allows you to notify user
/// about changing a volume when packing files
    extern "stdcall" fn SetChangeVolProc (
        HANDLE hArcData,
        tChangeVolProc pChangeVolProc1
        ) -> void {}

/// This function allows you to notify user about
/// the progress when you un/pack files
    extern "stdcall" fn SetProcessDataProc (
        HANDLE hArcData,
        tProcessDataProc pProcessDataProc
        ) -> void {}

/// GetPackerCaps tells WinCmd what features your packer plugin supports
    extern "stdcall" fn GetPackerCaps () -> int {}

/// PackFiles specifies what should happen when a user creates,
/// or adds files to the archive.
    extern "stdcall" fn PackFiles (
        char *PackedFile,
        char *SubPath,
        char *SrcPath,
        char *AddList,
        int Flags
        ) -> int {}

/// ConfigurePacker gets called when the user clicks the Configure button
/// from within "Pack files..." dialog box in WinCmd
    extern "stdcall" fn ConfigurePacker (
        HWND Parent,
        HINSTANCE DllInstance
        ) -> void {}

    extern "stdcall" fn PackSetDefaultParams (
        PackDefaultParamStruct* dps
        ) -> void {}

    extern "stdcall" fn CanYouHandleThisFile (
        char*FileName
        ) -> BOOL {}

    extern "stdcall" fn StartMemPack (
        int Options,
        char*FileName
        ) -> HANDLE {}

    extern "stdcall" fn PackToMem (
        HANDLE hMemPack,
        char*BufIn,
        int InLen,
        int*Taken,
        char*BufOut,
        int OutLen,
        int*Written,
        int SeekBy
        ) -> int {}

    extern "stdcall" fn DoneMemPack (
        HANDLE hMemPack
        ) -> int {}

    extern "stdcall" fn GetBackgroundFlags (
        ) -> int {}
*/
