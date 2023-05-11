use std::ffi::c_void;

use autd3::{
    core::{error::AUTDInternalError, link::Link},
    prelude::*,
};

pub struct DynamicLink {
    link_ptr: Box<dyn Link<DynamicTransducer>>,
}

impl DynamicLink {
    pub fn new(link_ptr: Box<dyn Link<DynamicTransducer>>) -> Self {
        Self { link_ptr }
    }
}

impl Link<DynamicTransducer> for DynamicLink {
    fn open(&mut self, geometry: &Geometry<DynamicTransducer>) -> Result<(), AUTDInternalError> {
        self.link_ptr.open(geometry)
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.link_ptr.close()
    }

    fn send(&mut self, tx: &autd3::core::TxDatagram) -> Result<bool, AUTDInternalError> {
        self.link_ptr.send(tx)
    }

    fn receive(&mut self, rx: &mut autd3::core::RxDatagram) -> Result<bool, AUTDInternalError> {
        self.link_ptr.receive(rx)
    }

    fn is_open(&self) -> bool {
        self.link_ptr.is_open()
    }

    fn timeout(&self) -> std::time::Duration {
        self.link_ptr.timeout()
    }
}

#[no_mangle]
pub extern "C" fn AUTDCreateGeometryBuilder(out: *mut *mut c_void) {
    unsafe {
        let builder = Box::new(Geometry::builder());
        let builder = Box::into_raw(builder);
        *out = builder as *mut c_void;
    }
}

#[no_mangle]
pub extern "C" fn AUTDAddDevice(
    builder: *mut c_void,
    x: float,
    y: float,
    z: float,
    rz1: float,
    ry: float,
    rz2: float,
) {
    unsafe {
        (*(builder as *mut GeometryBuilder)).add_device(AUTD3::new(
            Vector3::new(x, y, z),
            Vector3::new(rz1, ry, rz2),
        ));
    }
}

#[no_mangle]
pub extern "C" fn AUTDBuildGeometry(out: *mut *mut c_void, builder: *const c_void) {
    unsafe {
        let builder = Box::from_raw(builder as *mut GeometryBuilder);
        let geometry = Box::new(builder.build().unwrap());
        let geometry = Box::into_raw(geometry);
        *out = geometry as *mut c_void;
    }
}

#[no_mangle]
pub extern "C" fn AUTDOpenController(
    out: *mut *mut c_void,
    geometry: *const c_void,
    link: *const c_void,
) -> bool {
    unsafe {
        let link: Box<Box<dyn Link<DynamicTransducer>>> = Box::from_raw(link as *mut _);
        let link = DynamicLink::new(*link);

        let geometry: Box<Geometry<DynamicTransducer>> = Box::from_raw(geometry as *mut _);

        let cnt = Controller::open(*geometry, link);
        if cnt.is_err() {
            return false;
        }
        let cnt = Box::new(cnt.unwrap());
        let cnt = Box::into_raw(cnt);
        *out = cnt as *mut c_void;
    }
    true
}
