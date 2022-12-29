use crate::{*, collections::tree::Tree, dev::{*, filesystem::FileSystem}};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

pub enum ResourceType<'a> {
    Device(&'a mut dyn Device),
    FileSystem(&'a mut dyn FileSystem),
    Other
}

pub trait Resource {
    fn unwrap(&mut self) -> ResourceType;
    fn resource_path(&self) -> Vec<String>;
    fn resource_path_string(&self) -> String {
        let mut str = String::new();
        for node in self.resource_path() {
            str += (String::from("/") + node.as_str()).as_str();
        }
        str
    }
}

static mut NAMESPACE: Tree<String, Box<dyn Resource>> = Tree::new(String::new(), None);

pub fn cast_resource<D>(resource: &'static mut Box<dyn Resource>) -> &'static mut D {
    unsafe {
        ((*resource).as_mut() as *mut _ as *mut D).as_mut().unwrap()
    }
}

pub fn namespace() -> &'static mut Tree<String, Box<dyn Resource>> {
    unsafe {
        &mut NAMESPACE
    }
}

pub fn subtree_parts(path: Vec<String>) -> Option<&'static mut Tree<String, Box<dyn Resource>>> {
    namespace().get_node_by_path(path)
}

pub fn subtree(path: String) -> Option<&'static mut Tree<String, Box<dyn Resource>>> {
    namespace().get_node_by_path(split_resource_path(path))
}

pub fn init_namespace() {
    namespace().insert_subtree(String::from("Devices"), None);
    namespace().insert_subtree(String::from("Files"), None);
}

pub fn split_resource_path(path: String) -> Vec<String> {
    path.split("/").filter(|s| !s.is_empty()).map(|s| String::from(s)).collect()
}

pub fn concat_resource_path(path: Vec<String>) -> String {
    path.iter().map(|s| String::from("/") + s.as_str()).collect()
}

pub fn get_resource_parts<D>(path: Vec<String>) -> Option<&'static mut D> {
    get_resource_non_generic_parts(path).map(|d| cast_resource(d))
}

pub fn get_resource<D>(path: String) -> Option<&'static mut D> {
    get_resource_parts(split_resource_path(path))
}

pub fn get_resource_non_generic(path: String) -> Option<&'static mut Box<dyn Resource>> {
    namespace().get_node_by_path(split_resource_path(path)).unwrap().value()
}

pub fn get_resource_non_generic_parts(path: Vec<String>) -> Option<&'static mut Box<dyn Resource>> {
    namespace().get_node_by_path(path).unwrap().value()
}

pub fn register_resource<T: Resource + 'static>(resource: T) -> &'static mut T {
    let path = resource.resource_path();
    namespace().insert_node_by_path(path.clone(), Some(Box::new(resource)));
    get_resource_parts(path).unwrap()
}

pub fn register_resource_path<T: Resource + 'static>(path: Vec<String>, resource: T) -> &'static mut T {
    namespace().insert_node_by_path(path.clone(), Some(Box::new(resource)));
    get_resource_parts(path).unwrap()
}

pub fn get_block_device(path: String) -> Option<&'static mut dyn dev::BlockReadWrite> {
    get_block_device_parts(split_resource_path(path))
}

pub fn get_block_device_parts(path: Vec<String>) -> Option<&'static mut dyn dev::BlockReadWrite> {
    if let Some(drive) = namespace::get_resource_non_generic_parts(path) {
        if let ResourceType::Device(drive) = drive.unwrap() {
            if let DeviceClass::BlockDevice(drive) = Device::unwrap(drive) {
                return Some(drive)
            }
        }
    }
    None
}