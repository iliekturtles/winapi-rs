// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! DSound procedure declarations, constant definitions and macros
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DSCAPS {
    pub dwSize: ::DWORD,
    pub dwFlags: ::DWORD,
    pub dwMinSecondarySampleRate: ::DWORD,
    pub dwMaxSecondarySampleRate: ::DWORD,
    pub dwPrimaryBuffers: ::DWORD,
    pub dwMaxHwMixingAllBuffers: ::DWORD,
    pub dwMaxHwMixingStaticBuffers: ::DWORD,
    pub dwMaxHwMixingStreamingBuffers: ::DWORD,
    pub dwFreeHwMixingAllBuffers: ::DWORD,
    pub dwFreeHwMixingStaticBuffers: ::DWORD,
    pub dwFreeHwMixingStreamingBuffers: ::DWORD,
    pub dwMaxHw3DAllBuffers: ::DWORD,
    pub dwMaxHw3DStaticBuffers: ::DWORD,
    pub dwMaxHw3DStreamingBuffers: ::DWORD,
    pub dwFreeHw3DAllBuffers: ::DWORD,
    pub dwFreeHw3DStaticBuffers: ::DWORD,
    pub dwFreeHw3DStreamingBuffers: ::DWORD,
    pub dwTotalHwMemBytes: ::DWORD,
    pub dwFreeHwMemBytes: ::DWORD,
    pub dwMaxContigFreeHwMemBytes: ::DWORD,
    pub dwUnlockTransferRateHwBuffers: ::DWORD,
    pub dwPlayCpuOverheadSwBuffers: ::DWORD,
    pub dwReserved1: ::DWORD,
    pub dwReserved2: ::DWORD,
}
pub type LPDSCAPS = *mut DSCAPS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DSBCAPS {
    pub dwSize: ::DWORD,
    pub dwFlags: ::DWORD,
    pub dwBufferBytes: ::DWORD,
    pub dwUnlockTransferRate: ::DWORD,
    pub dwPlayCpuOverhead: ::DWORD,
}
pub type LPDSBCAPS = *mut DSBCAPS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DSEFFECTDESC {
    pub dwSize: ::DWORD,
    pub dwFlags: ::DWORD,
    pub guidDSFXClass: ::GUID,
    pub dwReserved1: ::DWORD_PTR,
    pub dwReserved2: ::DWORD_PTR,
}
pub type LPDSEFFECTDESC = *mut DSEFFECTDESC;
pub type LPCDSEFFECTDESC = *const DSEFFECTDESC;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DSBUFFERDESC {
    pub dwSize: ::DWORD,
    pub dwFlags: ::DWORD,
    pub dwBufferBytes: ::DWORD,
    pub dwReserved: ::DWORD,
    pub lpwfxFormat: ::LPWAVEFORMATEX,
    pub guid3DAlgorithm: ::GUID,
}
pub type LPDSBUFFERDESC = *mut DSBUFFERDESC;
pub type LPCDSBUFFERDESC = *const DSBUFFERDESC;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DSCBUFFERDESC {
    pub dwSize: ::DWORD,
    pub dwFlags: ::DWORD,
    pub dwBufferBytes: ::DWORD,
    pub dwReserved: ::DWORD,
    pub lpwfxFormat: ::LPWAVEFORMATEX,
    pub dwFXCount: ::DWORD,
    pub lpDSCFXDesc: ::LPDSCEFFECTDESC,
}
pub type LPDSCBUFFERDESC = *mut DSCBUFFERDESC;
pub type LPCDSCBUFFERDESC = *const DSCBUFFERDESC;

RIDL!(
interface IDirectSound(IDirectSoundVtbl): IUnknown(IUnknownVtbl)
{
    fn CreateSoundBuffer(
        &mut self, pcDSBufferDesc: ::LPCDSBUFFERDESC, ppDSBuffer: *mut ::LPDIRECTSOUNDBUFFER,
        pUnkOuter: ::LPUNKNOWN
    ) -> ::HRESULT,
    fn GetCaps(&mut self, pDSCaps: ::LPDSCAPS) -> ::HRESULT,
    fn DuplicateSoundBuffer(
        &mut self, pDSBufferOriginal: LPDIRECTSOUNDBUFFER,
        ppDSBufferDuplicate: *mut ::LPDIRECTSOUNDBUFFER
    ) -> ::HRESULT,
    fn SetCooperativeLevel(&mut self, hWnd: ::HWND, dwLevel: ::DWORD) -> ::HRESULT,
    fn Compact(&mut self) -> ::HRESULT,
    fn GetSpeakerConfig(&mut self, pdwSpeakerConfig: ::LPDWORD) -> ::HRESULT,
    fn SetSpeakerConfig(&mut self, dwSpeakerConfig: ::DWORD) -> ::HRESULT,
    fn Initialize(&mut self, pcGuidDevice: ::LPCGUID) -> ::HRESULT
}
);
pub type LPDIRECTSOUND = *mut IDirectSound;

RIDL!(
interface IDirectSound8(IDirectSound8Vtbl): IDirectSound(IDirectSoundVtbl)
{
    fn VerifyCertification(&mut self, ldwCertified: ::LPDWORD) -> ::HRESULT
}
);
pub type LPDIRECTSOUND8 = *mut IDirectSound8;

RIDL!(
interface IDirectSoundBuffer(IDirectSoundBufferVtbl): IUnknown(IUnknownVtbl) {
    fn GetCaps(&mut self, pDSBufferCaps: ::LPDSBCAPS) -> ::HRESULT,
    fn GetCurrentPosition(
        &mut self, pdwCurrentPlayCursor: ::LPDWORD, pdwCurrentWriteCursor: ::LPDWORD
    ) -> ::HRESULT,
    fn GetFormat(
        &mut self, pwfxFormat: ::LPWAVEFORMATEX, dwSizeAllocated: ::DWORD,
        pdwSizeWritten: ::LPDWORD
    ) -> ::HRESULT,
    fn GetVolume(&mut self, plVolume: ::LPLONG) -> ::HRESULT,
    fn GetPan(&mut self, plPan: ::LPLONG) -> ::HRESULT,
    fn GetFrequency(&mut self, pdwFrequency: ::LPDWORD) -> ::HRESULT,
    fn GetStatus(&mut self, pdwStatus: ::LPDWORD) -> ::HRESULT,
    fn Initialize(
        &mut self, pDirectSound: ::LPDIRECTSOUND, pcDSBufferDesc: ::LPCDSBUFFERDESC
    ) -> ::HRESULT,
    fn Lock(
        &mut self, dwOffset: ::DWORD, dwBytes: ::DWORD, ppvAudioPtr1: *mut ::LPVOID,
        pdwAudioBytes1: ::LPDWORD, ppvAudioPtr2: *mut ::LPVOID, pdwAudioBytes2: ::LPDWORD,
        dwFlags: ::DWORD
    ) -> ::HRESULT,
    fn Play(&mut self, dwReserved1: ::DWORD, dwPriority: ::DWORD, dwFlags: ::DWORD) -> ::HRESULT,
    fn SetCurrentPosition(&mut self, dwNewPosition: ::DWORD) -> ::HRESULT,
    fn SetFormat(&mut self, pcfxFormat: ::LPCWAVEFORMATEX) -> ::HRESULT,
    fn SetVolume(&mut self, lVolume: ::LONG) -> ::HRESULT,
    fn SetPan(&mut self, lPan: ::LONG) -> ::HRESULT,
    fn SetFrequency(&mut self, dwFrequency: ::DWORD) -> ::HRESULT,
    fn Stop(&mut self) -> ::HRESULT,
    fn Unlock(
        &mut self, pvAudioPtr1: ::LPVOID, dwAudioBytes1: ::DWORD, pvAudioPtr2: ::LPVOID,
        dwAudioBytes2: ::DWORD
    ) -> ::HRESULT,
    fn Restore(&mut self) -> ::HRESULT
}
);
pub type LPDIRECTSOUNDBUFFER = *mut IDirectSoundBuffer;

RIDL!(
interface IDirectSoundBuffer8(IDirectSoundBuffer8Vtbl): IDirectSoundBuffer(IDirectSoundBufferVtbl) {
    fn SetFX(
        &mut self, dwEffectsCount: ::DWORD, pDSFXDesc: ::LPDSEFFECTDESC, pdwResultCodes: ::LPDWORD
    ) -> ::HRESULT,
    fn AcquireResources(
        &mut self, dwFlags: ::DWORD, dwEffectsCount: ::DWORD, pdwResultCodes: ::LPDWORD
    ) -> ::HRESULT,
    fn GetObjectInpath(
        &mut self, rguidObject: ::REFGUID, dwIndex: ::DWORD, rguidInterface: ::REFGUID,
        ppObject: *mut ::LPVOID
    ) -> ::HRESULT
}
);
pub type LPDIRECTSOUNDBUFFER8 = *mut IDirectSoundBuffer8;

RIDL!(
interface IDirectSoundCapture(IDirectSoundCaptureVtbl): IUnknown(IUnknownVtbl) {
    fn CreateCaptureBuffer(
        &mut self, pcDSCBufferDesc: ::LPCDSCBUFFERDESC,
        ppDSCBuffer: *mut ::LPDIRECTSOUNDCAPTUREBUFFER, pUnkOuter: LPUNKNOWN
    ) -> ::HRESULT,
    fn GetCaps(&mut self, pDSCCaps: ::LPDSCCAPS) -> ::HRESULT,
    fn Initialize(&mut self, pcGuidDevice: ::LPCGUID) -> ::HRESULT
}
);
pub type LPDIRECTSOUNDCAPTURE = *mut IDirectSoundCapture;

pub type LPDSENUMCALLBACKA = Option<unsafe extern "system" fn(
    ::LPGUID, ::LPCSTR, ::LPCSTR, ::LPVOID
) -> ::BOOL>;
pub type LPDSENUMCALLBACKW = Option<unsafe extern "system" fn(
    ::LPGUID, ::LPCWSTR, ::LPCWSTR, ::LPVOID
) -> ::BOOL>;

pub const DS_OK: ::HRESULT = ::S_OK;
pub const DSERR_GENERIC: ::HRESULT = ::E_FAIL;

pub const DSSCL_NORMAL: ::DWORD = 0x00000001;
pub const DSSCL_PRIORITY: ::DWORD = 0x00000002;
pub const DSSCL_EXCLUSIVE: ::DWORD = 0x00000003;
pub const DSSCL_WRITEPRIMARY: ::DWORD = 0x00000004;

pub const DSBCAPS_PRIMARYBUFFER: ::DWORD = 0x00000001;
pub const DSBCAPS_STATIC: ::DWORD = 0x00000002;
pub const DSBCAPS_LOCHARDWARE: ::DWORD = 0x00000004;
pub const DSBCAPS_LOCSOFTWARE: ::DWORD = 0x00000008;
pub const DSBCAPS_CTRL3D: ::DWORD = 0x00000010;
pub const DSBCAPS_CTRLFREQUENCY: ::DWORD = 0x00000020;
pub const DSBCAPS_CTRLPAN: ::DWORD = 0x00000040;
pub const DSBCAPS_CTRLVOLUME: ::DWORD = 0x00000080;
pub const DSBCAPS_CTRLPOSITIONNOTIFY: ::DWORD = 0x00000100;
pub const DSBCAPS_CTRLFX: ::DWORD = 0x00000200;
pub const DSBCAPS_STICKYFOCUS: ::DWORD = 0x00004000;
pub const DSBCAPS_GLOBALFOCUS: ::DWORD = 0x00008000;
pub const DSBCAPS_GETCURRENTPOSITION2: ::DWORD = 0x00010000;
pub const DSBCAPS_MUTE3DATMAXDISTANCE: ::DWORD = 0x00020000;
pub const DSBCAPS_LOCDEFER: ::DWORD = 0x00040000;
pub const DSBCAPS_TRUEPLAYPOSITION: ::DWORD = 0x00080000;

pub const DSBPLAY_LOOPING: ::DWORD = 0x00000001;
pub const DSBPLAY_LOCHARDWARE: ::DWORD = 0x00000002;
pub const DSBPLAY_LOCSOFTWARE: ::DWORD = 0x00000004;
pub const DSBPLAY_TERMINATEBY_TIME: ::DWORD = 0x00000008;
pub const DSBPLAY_TERMINATEBY_DISTANCE: ::DWORD = 0x000000010;
pub const DSBPLAY_TERMINATEBY_PRIORITY: ::DWORD = 0x000000020;
