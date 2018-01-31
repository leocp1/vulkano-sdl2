extern crate libc;
extern crate sdl2;
extern crate sdl2_vulkan_sys;
extern crate vk_sys;
extern crate vulkano;

use std::error;
use std::fmt;

use vulkano::VulkanObject;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::RawInstanceExtensions;
use vulkano::swapchain::Surface;
use vulkano::swapchain::SurfaceCreationError;
use sdl2::video::Window;
use std::sync::Arc;

#[inline]
fn check_error(rv: sdl2::sys::SDL_bool) -> Result<(), String> {
    if rv == sdl2::sys::SDL_bool::SDL_TRUE {
        Ok(())
    } else {
        Err(sdl2::get_error())
    }
}

pub trait VkSurfaceBuild {
    fn build_vk_surface(&self, instance: Arc<Instance>) -> Result<Arc<Surface>, CreationError>;
    fn drawable_size(&self) -> [u32; 2];
    fn required_vk_instance_extensions(&self) -> Result<InstanceExtensions, String>;
}


impl VkSurfaceBuild for Window {
    fn drawable_size(&self) -> [u32; 2] {
        let mut w: libc::c_int = 0;
        let mut h: libc::c_int = 0;
        unsafe { sdl2_vulkan_sys::getDrawableSize(self.raw(), &mut w, &mut h) };
        [w as u32, h as u32]
    }

    fn required_vk_instance_extensions(&self) -> Result<InstanceExtensions, String> {
        unsafe {
            let mut count: libc::c_uint = std::mem::uninitialized();
            check_error(sdl2_vulkan_sys::getInstanceExtensions(
                self.raw(),
                &mut count,
                std::ptr::null_mut(),
            ))?;
            let mut ext_tmp: Vec<*const libc::c_char> = Vec::with_capacity(count as usize);
            check_error(sdl2_vulkan_sys::getInstanceExtensions(
                self.raw(),
                &mut count,
                ext_tmp.as_mut_ptr(),
            ))?;
            ext_tmp.set_len(count as usize);
            let ext_tmp = ext_tmp
                .iter()
                .map(|&x| std::ffi::CStr::from_ptr(x).to_owned())
                .collect::<Vec<_>>();
            let ext_tmp = RawInstanceExtensions::new(ext_tmp);
            Ok(InstanceExtensions::from(&ext_tmp))
        }
    }

    fn build_vk_surface(&self, instance: Arc<Instance>) -> Result<Arc<Surface>, CreationError> {
        if !instance.loaded_extensions().khr_surface {
            let err = SurfaceCreationError::MissingExtension {
                name: "VK_KHR_surface",
            };
            return Err(CreationError::from(err));
        }
        unsafe {
            let mut raw: vk_sys::SurfaceKHR = std::mem::uninitialized();
            check_error(sdl2_vulkan_sys::createSurface(
                self.raw(),
                instance.internal_object(),
                &mut raw,
            ))?;
            Ok(Arc::new(Surface::from_raw_surface(instance, raw)))
        }
    }
}

/// Error that can happen when creating a surface.
#[derive(Debug)]
pub enum CreationError {
    /// Error from vulkan.
    SurfaceCreationError(SurfaceCreationError),
    /// Error from SDL2.
    SDLError(String),
}

impl error::Error for CreationError {
    #[inline]
    fn description(&self) -> &str {
        match *self {
            CreationError::SurfaceCreationError(_) => "vulkan error while creating the surface",
            CreationError::SDLError(ref err) => err,
        }
    }

    #[inline]
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CreationError::SurfaceCreationError(ref err) => Some(err),
            CreationError::SDLError(_) => None,
        }
    }
}

impl fmt::Display for CreationError {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", error::Error::description(self))
    }
}

impl From<SurfaceCreationError> for CreationError {
    #[inline]
    fn from(err: SurfaceCreationError) -> CreationError {
        CreationError::SurfaceCreationError(err)
    }
}

impl From<String> for CreationError {
    #[inline]
    fn from(err: String) -> CreationError {
        CreationError::SDLError(err)
    }
}
