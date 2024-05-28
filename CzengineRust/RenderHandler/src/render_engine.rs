use std::{cell::RefCell, rc::Rc};

use vulkanalia::{loader::{LibloadingLoader, LIBRARY}, vk::{self, make_version, ApplicationInfo, HasBuilder, InstanceCreateFlags, InstanceCreateInfo}, window, Entry, Version};
use winit::window::Window;
use anyhow::{anyhow, Result};
use log::*;


const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

pub trait RenderEngine {
    fn buildAppInfo(&mut self);
    fn buildAppExtensions(&mut self);
    fn buildAppFlags(&mut self);
    fn buildAppCreateInfo(&mut self);
}

pub struct VulkanEngine {
    entry: Option<Entry>,
    create_info: RefCell<Option<InstanceCreateInfo>>,
    app_info: RefCell<Option<ApplicationInfo>>,
    app_ext: RefCell<Option<Vec<*const i8>>>,
    app_flags: RefCell<Option<InstanceCreateFlags>>,
    window_ref: Rc<RefCell<Window>>
}

impl VulkanEngine {
    pub unsafe fn new(window: Rc<RefCell<Window>>) -> Self {
        let temp_loader = LibloadingLoader::new(LIBRARY).unwrap();
        VulkanEngine {
            app_info: RefCell::new(None),
            app_ext: RefCell::new(None),
            app_flags: RefCell::new(None),
            window_ref: window,
            entry: Entry::new(temp_loader).map_err(|b| anyhow!("{}", b)).ok(),
            create_info: RefCell::new(None)
        }
    }
    fn checkForMacOs(&mut self) -> InstanceCreateFlags {
        let opt_entry = &self.entry;

        match opt_entry {
            Some(ee) => {
                if ((ee.version().unwrap()) < PORTABILITY_MACOS_VERSION || cfg!(not(target_os = "macos"))) {
                    vk::InstanceCreateFlags::empty()
                } else {
                    self.testExtensions()
                }
            },
            None => vk::InstanceCreateFlags::empty(),
        }
    }
    fn testExtensions(&mut self) -> InstanceCreateFlags {
        let app_extensions: &mut Option<Vec<*const i8>> = &mut *self.app_ext.borrow_mut();
        match (app_extensions) {
            Some(extensions) => {
                extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
                extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
                vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
            },
            None => vk::InstanceCreateFlags::empty(),
        }
    }
}

impl RenderEngine for VulkanEngine {
    fn buildAppInfo(&mut self) {
        let mut window = (*self.window_ref).borrow_mut();
        let title = window.title().clone();
        let app_info_builder = ApplicationInfo::builder()
            .application_name(title.as_bytes())
            .application_version(make_version(1, 0, 0))
            .engine_name(b"No Engine\0")
            .engine_version(make_version(1, 0, 0))
            .api_version(make_version(1, 0, 0));
        self.app_info.replace(Some(app_info_builder.build()));
    }

    fn buildAppExtensions(&mut self) {
        self.app_ext.replace(Some(
            window::get_required_instance_extensions(&*(*self.window_ref).borrow_mut())
                .iter()
                .map(|e| e.as_ptr())
                .collect::<Vec<_>>()
        ));
    }

    fn buildAppFlags(&mut self) {
        let flags = self.checkForMacOs();
        self.app_flags.replace(Some(flags));
    }

    fn buildAppCreateInfo(&mut self) {
        let _app_info: &mut Option<ApplicationInfo> = &mut *self.app_info.borrow_mut();
        let _app_option_ext = &mut self.app_ext.borrow_mut();
        let _app_ext: Option<&Vec<*const i8>> = (&mut *_app_option_ext).as_ref();
        let _create_info = vk::InstanceCreateInfo::builder()
            .application_info(&_app_info.unwrap())
            .enabled_extension_names(&_app_ext.unwrap())
            .build();
        self.create_info.replace(Some(
            vk::InstanceCreateInfo::builder()
                .application_info(&_app_info.unwrap())
                .enabled_extension_names(_app_ext.unwrap().as_ref())
                .build()
        ));
    }
}
