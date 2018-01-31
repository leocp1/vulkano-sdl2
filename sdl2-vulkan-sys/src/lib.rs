extern crate libc;
extern crate sdl2;
extern crate vk_sys;

use libc::c_char;
use libc::c_int;
use libc::c_uint;
use libc::c_void;
use vk_sys::Instance;
use vk_sys::SurfaceKHR;
use sdl2::sys::SDL_bool;
use sdl2::sys::SDL_Window;

#[link(name = "SDL2")]
extern "C" {
    #[link_name = "SDL_Vulkan_LoadLibrary"]
    pub fn loadLibrary(path: *const c_char) -> c_int;
    #[link_name = "SDL_Vulkan_GetVkGetInstanceProcAddr"]
    pub fn getVkGetInstanceProcAddr() -> *mut c_void;
    #[link_name = "SDL_Vulkan_UnloadLibrary"]
    pub fn unloadLibrary();
    #[link_name = "SDL_Vulkan_GetInstanceExtensions"]
    pub fn getInstanceExtensions(
        window: *mut SDL_Window,
        pCount: *mut c_uint,
        pNames: *mut *const c_char,
    ) -> SDL_bool;
    #[link_name = "SDL_Vulkan_CreateSurface"]
    pub fn createSurface(
        window: *mut SDL_Window,
        instance: Instance,
        surface: *mut SurfaceKHR,
    ) -> SDL_bool;
    #[link_name = "SDL_Vulkan_GetDrawableSize"]
    pub fn getDrawableSize(window: *mut SDL_Window, w: *mut c_int, h: *mut c_int);
}
