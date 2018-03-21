use libc;
use libc::*;
use std::mem;

#[derive(Debug)]
struct List {
    value_ptr: *mut c_void,
    next: *mut List,
}

impl List {
    fn new(value: *mut c_void, next: *mut List) -> *mut List {
        let ptr: *mut List = unsafe { libc::malloc(mem::size_of::<List>()) } as *mut List;
        unsafe {
            (*ptr).value_ptr = value;
            (*ptr).next = next;
        }

        ptr
    }

    fn add(&mut self, value: *mut c_void){
        let old = self.next;
        let new = List::new(value, old);
        self.next = new;
    }
}

#[derive(Debug)]
pub struct CStringManager {
    cstrings: *mut List,
}

static mut INSTANCE: CStringManager = CStringManager {
    cstrings: 0 as *mut List,
};

static mut ZERO_BYTE_STRING: *mut i8 = 0 as *mut i8;

impl CStringManager {
    pub fn new_cstring_as_ptr(string: &str) -> *const i8 {
        if string.len() == 0 {
            unsafe {
                if ZERO_BYTE_STRING.is_null() {
                    ZERO_BYTE_STRING = libc::malloc(1) as *mut i8;
                    *ZERO_BYTE_STRING.offset(0) = '\0' as i8;
                }
                return ZERO_BYTE_STRING;
            }
        }

        let ptr = unsafe {
            // malloc
            let buf : *mut c_void = libc::malloc(string.len() + 1);
            INSTANCE.add(buf);

            // init
            let mut p = buf as *mut i8;
            let bytes = string.as_bytes();
            let len = bytes.len();
            for i in 0..len {
                *p.offset(i as isize) = bytes[i] as i8;
            }
            *p.offset(len as isize) = '\0' as i8;

            buf as *mut i8
        };

        ptr
    }

    fn add(&mut self, cstring: *mut c_void){
        if self.cstrings.is_null() {
            self.cstrings = List::new(cstring, 0 as *mut List);
        }else{
            unsafe { (*self.cstrings).add(cstring) };
        }
    }
}

impl Drop for CStringManager {
    fn drop(&mut self){
        let mut list: *mut List = unsafe { &mut *self.cstrings };
        unsafe {
            while !list.is_null() {
                libc::free((*list).value_ptr);
                let ptr = list;
                list = (*list).next;
                libc::free(ptr as *mut c_void);
            }

            libc::free(ZERO_BYTE_STRING as *mut c_void);
        }
    }
}