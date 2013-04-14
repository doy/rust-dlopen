#[link(name = "dlopen",
       vers = "0.0.1",
       uuid = "c6a5758b-9b45-41fd-a935-f3cb7251b4eb",
       url  = "https://github.com/doy/rust-dlopen")];

#[crate_type = "lib"];

pub use os::Library;

#[cfg(target_os = "linux")]
#[cfg(target_os = "android")]
#[cfg(target_os = "freebsd")]
#[cfg(target_os = "macos")]
mod os {
    use core::libc::{c_char,c_int,c_void};

    pub struct Library(*c_void);
    struct Function(*c_void);

    impl Library {
        pub fn open (name: &str) -> Library {
            do str::as_c_str(libname(name)) |c_name| {
                let lib = unsafe { dlopen(c_name, 0) };
                if lib == ptr::null() {
                    fail!(unsafe { str::raw::from_c_str(dlerror()) })
                }
                else {
                    Library(lib)
                }
            }
        }

        pub fn get_fn (&self, name: &str) -> Function {
            unsafe { dlerror() };
            do str::as_c_str(name) |c_name| {
                let Library(lib) = *self;
                let fun = unsafe { dlsym(lib, c_name) };
                let err = unsafe { dlerror() };
                if err == ptr::null() {
                    fail!(unsafe { str::raw::from_c_str(err) })
                }
                else {
                    Function(fun)
                }
            }
        }
    }

    impl Drop for Library {
        fn finalize (&self) {
            let Library(lib) = *self;
            unsafe { dlclose(lib) };
        }
    }

    impl Function {
        fn call0<A>(&self) -> A {
            let Function(fun) = *self;
            // XXX how do you call a c function pointer from rust?
            let c_fun = unsafe { cast::transmute(fun) };
            c_fun()
        }
    }

    #[cfg(target_os = "linux")]
    #[cfg(target_os = "android")]
    #[cfg(target_os = "freebsd")]
    fn libname (name: &str) -> ~str {
        fmt!("lib%s.so", name)
    }

    #[cfg(target_os = "macos")]
    fn libname (name: &str) -> ~str {
        fmt!("lib%s.dylib", name)
    }

    #[link_name = "dl"]
    extern "C" {
        fn dlopen (filename: *c_char, flag: c_int) -> *c_void;
        fn dlerror () -> *c_char;
        fn dlsym (handle: *c_void, symbol: *c_char) -> *c_void;
        fn dlclose (handle: *c_void) -> c_int;
    }
}

#[cfg(target_os = "win32")]
mod os {
    use core::libc::{BOOL,DWORD,HMODULE,LPCTSTR};

    struct FARPROC(*c_void);

    pub struct Library(HMODULE);
    struct Function(FARPROC);

    impl Library {
        pub fn open (name: &str) -> Library {
            fail!();
        }

        pub fn get_fn (&self, name: &str) -> Function {
            fail!();
        }
    }

    impl Drop for Library {
        fn finalize (&self) {
            fail!();
        }
    }

    impl Function {
        fn call0<A>(&self) -> A {
            fail!();
        }
    }

    fn libname (name: &str) -> ~str {
        fmt!("%s.dll", name)
    }

    extern "C" {
        fn LoadLibrary(filename: LPCTSTR) -> HMODULE;
        fn GetLastError() -> DWORD;
        fn GetProcAddress(hModule: HMODULE, lpProcName: LPCTSTR) -> FARPROC;
        fn FreeLibrary(hModule: HMODULE) -> BOOL;
    }
}
