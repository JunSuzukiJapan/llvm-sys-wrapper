use std::ffi::CString;

#[derive(Debug)]
pub struct CStringManager {
    cstrings: *mut Vec<CString>
}

static mut INSTANCE: CStringManager = CStringManager { cstrings: 0 as *mut Vec<CString> };

impl CStringManager {
    pub fn new_cstring_as_ptr(string: &str) -> *const i8 {
        let cstring = CString::new(string).unwrap();
        let ptr = cstring.as_ptr();
        unsafe { INSTANCE.add(cstring); }
        ptr
    }

    fn add(&mut self, cstring: CString){
        if self.cstrings == 0 as *mut Vec<CString> {
            self.cstrings = &mut Vec::new();
        }
        let mut v: &mut Vec<CString> = unsafe { &mut *self.cstrings };
        v.push(cstring)
    }
}