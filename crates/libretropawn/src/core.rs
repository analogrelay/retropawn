use std::ffi::OsStr;

use libloading::Library;
use libretro_sys::CoreAPI;
use thiserror::Error;

/// Represents an error loading or interacting with a Libretro core.
#[derive(Error, Debug)]
pub enum Error {
    #[error("error loading symbol from library")]
    LibError(#[from] libloading::Error),
}

/// Represents a loaded Libretro core.
pub struct Core {
    library: Library,
    core_api: CoreAPI,
}

impl Core {
    pub fn get_api_version(&self) -> u32 {
        unsafe {
            (self.core_api.retro_api_version)()
        }
    }

    pub fn init(&self) {
        unsafe {
            (self.core_api.retro_init)();
        }
    }

    pub fn load(path: impl AsRef<OsStr>) -> Result<Core, Error> {
        unsafe {
            let lib = Library::new(path)?;
            Core::from_library(lib)
        }
    }

    pub fn from_library(dylib: Library) -> Result<Core, Error> {
        unsafe {
            let core_api = CoreAPI {
                retro_set_environment: *(dylib.get(b"retro_set_environment")?),
                retro_set_video_refresh: *(dylib.get(b"retro_set_video_refresh")?),
                retro_set_audio_sample: *(dylib.get(b"retro_set_audio_sample")?),
                retro_set_audio_sample_batch: *(dylib.get(b"retro_set_audio_sample_batch")?),
                retro_set_input_poll: *(dylib.get(b"retro_set_input_poll")?),
                retro_set_input_state: *(dylib.get(b"retro_set_input_state")?),

                retro_init: *(dylib.get(b"retro_init")?),
                retro_deinit: *(dylib.get(b"retro_deinit")?),

                retro_api_version: *(dylib.get(b"retro_api_version")?),

                retro_get_system_info: *(dylib.get(b"retro_get_system_info")?),
                retro_get_system_av_info: *(dylib.get(b"retro_get_system_av_info")?),
                retro_set_controller_port_device: *(dylib.get(b"retro_set_controller_port_device")?),

                retro_reset: *(dylib.get(b"retro_reset")?),
                retro_run: *(dylib.get(b"retro_run")?),

                retro_serialize_size: *(dylib.get(b"retro_serialize_size")?),
                retro_serialize: *(dylib.get(b"retro_serialize")?),
                retro_unserialize: *(dylib.get(b"retro_unserialize")?),

                retro_cheat_reset: *(dylib.get(b"retro_cheat_reset")?),
                retro_cheat_set: *(dylib.get(b"retro_cheat_set")?),

                retro_load_game: *(dylib.get(b"retro_load_game")?),
                retro_load_game_special: *(dylib.get(b"retro_load_game_special")?),
                retro_unload_game: *(dylib.get(b"retro_unload_game")?),

                retro_get_region: *(dylib.get(b"retro_get_region")?),
                retro_get_memory_data: *(dylib.get(b"retro_get_memory_data")?),
                retro_get_memory_size: *(dylib.get(b"retro_get_memory_size")?),
            };
            
            Ok(Core {
                library: dylib,
                core_api: core_api,
            })
        }
    }
}