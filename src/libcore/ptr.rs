#[doc = "Unsafe pointer utility functions"];

export addr_of;
export mut_addr_of;
export offset;
export mut_offset;
export null;
export memcpy;
export memmove;

import libc::c_void;

#[nolink]
#[abi = "cdecl"]
native mod libc_ {
    #[rust_stack]
    fn memcpy(dest: *c_void, src: *c_void, n: libc::size_t) -> *c_void;
    #[rust_stack]
    fn memmove(dest: *c_void, src: *c_void, n: libc::size_t) -> *c_void;
}

#[abi = "rust-intrinsic"]
native mod rusti {
    fn addr_of<T>(val: T) -> *T;
}

#[doc = "Get an unsafe pointer to a value"]
#[inline(always)]
fn addr_of<T>(val: T) -> *T { rusti::addr_of(val) }

#[doc = "Get an unsafe mut pointer to a value"]
#[inline(always)]
fn mut_addr_of<T>(val: T) -> *mut T unsafe {
    unsafe::reinterpret_cast(rusti::addr_of(val))
}

#[doc = "Calculate the offset from a pointer"]
#[inline(always)]
fn offset<T>(ptr: *T, count: uint) -> *T unsafe {
    (ptr as uint + count * sys::size_of::<T>()) as *T
}

#[doc = "Calculate the offset from a mut pointer"]
#[inline(always)]
fn mut_offset<T>(ptr: *mut T, count: uint) -> *mut T {
    (ptr as uint + count * sys::size_of::<T>()) as *mut T
}


#[doc = "Create an unsafe null pointer"]
#[inline(always)]
pure fn null<T>() -> *T unsafe { ret unsafe::reinterpret_cast(0u); }

#[doc = "Returns true if the pointer is equal to the null pointer"]
pure fn is_null<T>(ptr: *const T) -> bool { ptr == null() }

#[doc = "Returns true if the pointer is not equal to the null pointer"]
pure fn is_not_null<T>(ptr: *const T) -> bool { !is_null(ptr) }

#[doc = "
Copies data from one location to another

Copies `count` elements (not bytes) from `src` to `dst`. The source
and destination may not overlap.
"]
#[inline(always)]
unsafe fn memcpy<T>(dst: *T, src: *T, count: uint) {
    let n = count * sys::size_of::<T>();
    libc_::memcpy(dst as *c_void, src as *c_void, n);
}

#[doc = "
Copies data from one location to another

Copies `count` elements (not bytes) from `src` to `dst`. The source
and destination may overlap.
"]
#[inline(always)]
unsafe fn memmove<T>(dst: *T, src: *T, count: uint)  {
    let n = count * sys::size_of::<T>();
    libc_::memmove(dst as *c_void, src as *c_void, n);
}

#[test]
fn test() unsafe {
    type pair = {mut fst: int, mut snd: int};
    let p = {mut fst: 10, mut snd: 20};
    let pptr: *mut pair = mut_addr_of(p);
    let iptr: *mut int = unsafe::reinterpret_cast(pptr);
    assert (*iptr == 10);;
    *iptr = 30;
    assert (*iptr == 30);
    assert (p.fst == 30);;

    *pptr = {mut fst: 50, mut snd: 60};
    assert (*iptr == 50);
    assert (p.fst == 50);
    assert (p.snd == 60);

    let v0 = [32000u16, 32001u16, 32002u16];
    let v1 = [0u16, 0u16, 0u16];

    ptr::memcpy(ptr::offset(vec::unsafe::to_ptr(v1), 1u),
                ptr::offset(vec::unsafe::to_ptr(v0), 1u), 1u);
    assert (v1[0] == 0u16 && v1[1] == 32001u16 && v1[2] == 0u16);
    ptr::memcpy(vec::unsafe::to_ptr(v1),
                ptr::offset(vec::unsafe::to_ptr(v0), 2u), 1u);
    assert (v1[0] == 32002u16 && v1[1] == 32001u16 && v1[2] == 0u16);
    ptr::memcpy(ptr::offset(vec::unsafe::to_ptr(v1), 2u),
                vec::unsafe::to_ptr(v0), 1u);
    assert (v1[0] == 32002u16 && v1[1] == 32001u16 && v1[2] == 32000u16);
}