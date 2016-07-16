
use libc::{c_char, c_void, c_int};
use libpulse_sys::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr::{null, null_mut};

#[derive(Clone, Debug, Default)]
pub struct PaSourceInfo {
    pub name: String,
    pub rate: u32,
}

// we can ignore the sample format and spec - only need rate for pulse-simple api
#[derive(Clone)]
struct RawSourceInfo {
    pub index: usize,
    pub name: String,
    pub rate: u32,
}

impl Into<PaSourceInfo> for RawSourceInfo {
    fn into(self) -> PaSourceInfo {
        PaSourceInfo {
            name: self.name,
            rate: self.rate,
        }
    }
}

struct CbData {
    items: Vec<RawSourceInfo>,
    mainloop: *mut pa_mainloop,
}

unsafe extern "C" fn source_test_cb(_: *mut pa_context,
                                    i: *const pa_source_info,
                                    eol: c_int,
                                    userdata: *mut c_void) {
    let ref mut cb_data = *(userdata as *mut CbData);
    if eol == 0 {
        // still more entries
        let source_info = *i;
        let rs_source_info = RawSourceInfo {
            index: source_info.index as usize,
            name: ::std::ffi::CStr::from_ptr(source_info.name).to_str().unwrap().to_string(),
            rate: source_info.sample_spec.rate,
        };
        cb_data.items.push(rs_source_info);
    } else if eol > 0 {
        // no more entries
        pa_mainloop_quit(cb_data.mainloop, 0);
        pa_mainloop_free(cb_data.mainloop);
    } else {
        // error
        pa_mainloop_quit(cb_data.mainloop, 1);
        pa_mainloop_free(cb_data.mainloop);
    }
}

unsafe extern "C" fn state_cb(ctxt: *mut pa_context, userdata: *mut c_void) {
    let state = pa_context_get_state(ctxt);
    if state == PA_CONTEXT_READY {
        pa_context_get_source_info_by_index(ctxt, 1, Some(source_test_cb), userdata);
    } else if state == PA_CONTEXT_FAILED || state == PA_CONTEXT_TERMINATED {
        let ref mut cb_data = *(userdata as *mut CbData);
        pa_mainloop_quit(cb_data.mainloop, 1);
        pa_mainloop_free(cb_data.mainloop);
    }
}

pub fn get_devices() -> Result<HashMap<usize, PaSourceInfo>, String> {
    unsafe {
        let mainloop = pa_mainloop_new();
        let mainloop_ret_ptr = null_mut::<c_int>();
        let api = pa_mainloop_get_api(mainloop);
        let ctxt =
            pa_context_new(api,
                           CString::new("Output Device Query").unwrap().as_ptr() as *const c_char);
        let mut cb_data = CbData {
            items: Vec::new(),
            mainloop: mainloop,
        };
        pa_context_connect(ctxt, null(), PA_CONTEXT_NOFLAGS, null());
        pa_context_set_state_callback(ctxt,
                                      Some(state_cb),
                                      &mut cb_data as *mut CbData as *mut c_void);
        pa_mainloop_run(mainloop, mainloop_ret_ptr);
        if mainloop_ret_ptr != null_mut() {
            if *mainloop_ret_ptr != 0 {
                return Err("Error when querying output devices".to_string());
            }
        }
        let mut ret_data = HashMap::new();
        for source in cb_data.items {
            let item = ret_data.entry(source.index).or_insert_with(PaSourceInfo::default);
            *item = source.to_owned().into();
        }
        Ok(ret_data)
    }
}
