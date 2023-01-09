// use winapi::shared::minwindef::LPCVOID;
// // use winapi::um::memoryapi::{
// //     VirtualAlloc, VirtualFree, VirtualLock, VirtualProtect, VirtualQuery, VirtualUnlock};
// use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
// use std;
// use winapi::um::memoryapi::VirtualQueryEx;
// use winapi::um::{processthreadsapi, sysinfoapi};
// use winapi::um::sysinfoapi::{GetNativeSystemInfo, SYSTEM_INFO};
// use winapi::um::winnt::{PVOID, MEMORY_BASIC_INFORMATION, MEM_RELEASE, MEM_RESERVE, MEM_COMMIT, PROCESS_QUERY_INFORMATION,PROCESS_VM_READ, MEM_PRIVATE, MEM_IMAGE, MEM_MAPPED};

// use std::cmp::{max, min};
// use std::io;
// use std::mem::{size_of, MaybeUninit};
// use winapi::shared::basetsd::SIZE_T;
// use winapi::um::memoryapi::{
//   VirtualAlloc, VirtualFree, VirtualLock, VirtualProtect, VirtualQuery, VirtualUnlock,
// };

// use winapi::{ctypes::c_void, um::winnt::MEMORY_BASIC_INFORMATION};
use std::{ffi::c_void};
use ptree::{TreeBuilder, print_tree};
use windows::Win32::{System::{Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ}, Memory::MEMORY_BASIC_INFORMATION}, Foundation::{HANDLE, CloseHandle}};
use windows::Win32::System::Memory::VirtualQueryEx;
use ::core::option::Option;
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please use: {} <pid>", args[0]); 
        std::process::abort();
    }


    let pid: u32 = args[1].parse::<u32>().expect("Invalid Process ID.");


    let handle = unsafe {
        OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)
    };

    let handle = handle.expect("Handle was not opened.");

    const MEM_SIZE: usize = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();

    let mut basic_info_vec: Vec<MEMORY_BASIC_INFORMATION> = Vec::new();
    unsafe {
        let mut basic_info = MEMORY_BASIC_INFORMATION::default();
        let mut base_addr: Option<*const c_void> = Option::Some(basic_info.BaseAddress);
        loop {
            let res = VirtualQueryEx(
                handle,
                base_addr,
                &mut basic_info,
                MEM_SIZE
            );   
            // ...
            if res == 0 {
                break;
            }

            basic_info_vec.push(basic_info);
            
            base_addr = Some(base_addr.unwrap().add(basic_info.RegionSize));
        }

        CloseHandle(handle);
    }


    // Build a tree using a TreeBuilder
    let mut tree = TreeBuilder::new("Virtual Memory Regions".to_string());

    let mut base_addr: usize = 0;
    for basic in basic_info_vec {
        

        let info_addr = basic.AllocationBase as usize;

        if info_addr != 00000000 {
            tree.add_empty_child(format!("{:08x}", info_addr));
        } 

        let range = unsafe { basic.BaseAddress.add(basic.RegionSize as usize) } as usize;

        if basic.BaseAddress as usize == 00000000 {
            continue;
        }
        
        tree.add_empty_child(format!("\t{:08x}-{:08x}", basic.BaseAddress as usize, range));
    }

    print_tree(&tree.build()).expect("Error");

}