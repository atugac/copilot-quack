#![allow(non_snake_case)]

mod quack;

use crate::quack::quack;
use std::ffi::c_void;
use windows::ApplicationModel::Resources::ResourceLoader;
use windows::core::implement;
use windows::Win32::Foundation::{E_NOTIMPL, HMODULE};
use windows::Win32::System::Com::IClassFactory;
use windows::Win32::System::Com::{IBindCtx, IClassFactory_Impl};
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::SystemServices::SFGAO_FOLDER;
use windows::Win32::UI::Shell::{
    IEnumExplorerCommand, IExplorerCommand, IShellItemArray,
    IExplorerCommand_Impl, SHStrDupW
};
use windows::core::{h, Error, IUnknown, Interface, Ref, BOOL, GUID, HRESULT, HSTRING, PWSTR};

#[implement(IExplorerCommand)]
pub struct QuackCommand;

impl IExplorerCommand_Impl for QuackCommand_Impl {
    fn GetTitle(&self, psiitemarray: Ref<IShellItemArray>) -> windows_core::Result<PWSTR> {
        let id = if let Some(items) = psiitemarray.as_ref() {
            let count = unsafe { items.GetCount()? };
            match count {
                0 => h!("QuackMisc"), // Shouldn't be reachable
                1 => unsafe {
                    let item = items.GetItemAt(0)?;
                    let attr = item.GetAttributes(SFGAO_FOLDER)?;
                    if (attr & SFGAO_FOLDER).0 != 0 {
                        h!("QuackDir")
                    } else {
                        h!("QuackFile")
                    }
                },
                _ => h!("QuackMulti"),
            }
        } else {
            h!("QuackHere")
        };
        let loader = ResourceLoader::GetForViewIndependentUse()?;
        let value = loader.GetString(id)?;
        unsafe {
            SHStrDupW(&value)
        }
    }

    fn GetIcon(&self, _psiitemarray: Ref<IShellItemArray>) -> windows_core::Result<PWSTR> {
        unsafe {
            let hmod = get_module_instance_handle();
            let mut path = [0u16; 260];
            let len = GetModuleFileNameW(Some(hmod), &mut path);
            let path_str = String::from_utf16_lossy(&path[..len as usize]);
            let icon_path = format!("{},-101", path_str);
            SHStrDupW(&HSTRING::from(icon_path))
        }
    }

    fn GetToolTip(&self, _psiitemarray: Ref<IShellItemArray>) -> windows_core::Result<PWSTR> {
        Err(Error::from(E_NOTIMPL))
    }

    fn GetCanonicalName(&self) -> windows_core::Result<GUID> {
        let guid = GUID::from_u128(0xA5BE5705_FFBD_4023_9BA1_965769552B93);
        Ok(guid)
    }

    fn GetState(
        &self, _psiitemarray: Ref<IShellItemArray>, _foktobeslow: BOOL
    ) -> windows_core::Result<u32> {
        Ok(0) // ECS_ENABLED
    }

    fn Invoke(
        &self, _psiitemarray: Ref<IShellItemArray>, _pbc: Ref<IBindCtx>
    ) -> windows_core::Result<()> {
        unsafe {
            quack();
        }
        Ok(())
    }

    fn GetFlags(&self) -> windows_core::Result<u32> {
        Ok(0) // ECF_DEFAULT
    }

    fn EnumSubCommands(&self) -> windows_core::Result<IEnumExplorerCommand> {
        Err(Error::from(E_NOTIMPL))
    }
}

#[implement(IClassFactory)]
struct QuackClassFactory;

impl IClassFactory_Impl for QuackClassFactory_Impl {
    fn CreateInstance(
        &self,
        _punkouter: Ref<IUnknown>, riid: *const GUID, ppvobject: *mut *mut c_void
    ) -> windows_core::Result<()> {
        let command: IExplorerCommand = QuackCommand.into();
        unsafe { command.query(riid, ppvobject).ok() }
    }

    fn LockServer(&self, _flock: BOOL) -> windows_core::Result<()> {
        Ok(())
    }
}

unsafe extern "system" {
    static __ImageBase: u8;
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn get_module_instance_handle() -> HMODULE {
    HMODULE(&__ImageBase as *const _ as *mut c_void)
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllGetClassObject(
    _rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let factory: IClassFactory = QuackClassFactory.into();
    unsafe {
        factory.query(riid, ppv)
    }
}