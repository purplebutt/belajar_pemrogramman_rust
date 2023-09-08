mod chapter5;
mod chapter11;
mod chapter12;
mod chapter13;
mod chapter14;
mod chapter14_builder;
mod chapter14_matching;
mod chapter14_err_handling;
mod chapter14_opt_arg;

use chapter11::trait_std_lib_cmp::demo;
// use chapter12::rc_refcell::{demo, test};
use chapter12::rc_weak::weakdemo;
// use chapter5::{closure::*, fn_fnmut_fnonce::{closure_demo, caller}};
// use chapter11::trait_bound::*;
// use chapter11::trait_definisi::*;
// use chapter11::trait_with_generic::*;
// use chapter11::trait_type_system::*;
// use chapter11::trait_type_system_generic::*;
// use chapter11::trait_std_library::{test, test_clone};
use chapter12::smart_pointer_12_1::*;
// use chapter12::sp_box::{box_example, test};
// use chapter12::refcell::*;
// use chapter13::server_request::*;
// use chapter13::spawning_thread::*;
// use chapter13::joinhandle::*;
//use chapter13::{closure_with_variable::*, scoped_thread::{scoped_thread, scoped_demo}, mutex_poison::poison_demo, mpsc::mpsc_demo, send_sync::send_sync_demo};
// use chapter13::atomic_rc::*;
// use chapter13::mutex::*;
#[allow(unused)]
use chapter14::builder::with_builder;
use chapter14::with_state::{self, with_state_demo};
use chapter14::{no_builder::no_builder, with_generic::with_generic_demo};
use chapter14::with_generic::{Bicycle, Finishable};
use chapter14::with_generic::{Builder, Buildable};
use chapter14_builder::demo::demo1;
use chapter14_builder::entry::entry_demo;
use chapter14_builder::{builder_demo, demo};
use chapter14_err_handling::error_handling::err_handling;
use chapter14_matching::patt_match::{pattern_matching_demo, number_matching, tuple_matching, tuple_matching2};
use chapter14_matching::{struct_extract_demo, tuple_extract_demo};
use chapter14_opt_arg::baz;


fn build_bicycle() {
    let builder = Bicycle::builder()
        .with_make("Poligon")
        .with_model("A11B")
        .with_size(20)
        .with_color("Green")
        .build();

    builder.print();
}


fn main() {
    
    // with_builder();
    // no_builder();
    // with_generic_demo();
    // build_bicycle();
    // with_state_demo();
    // demo1();
    // builder_demo();
    // entry_demo();
    // pattern_matching_demo();
    // tuple_matching();
    // number_matching();
    // tuple_matching2();
    // struct_extract_demo();
    // tuple_extract_demo();

    // let r = err_handling()?;
    // println!("{r}");
    // Ok(())
    // err_handling();
    // baz();
    // demo_kartu(); 
    // demo();
    // test();
    // weakdemo();
    demo();

    // send_sync_demo()
    // int_mut_cell();
    // int_mut();
    // box_example();
    // test();
    // instant_demo()
    // demo_closure();
    //  sample_closure2();
    // closure_demo();
    // caller()
    // scoped_thread();
    // scoped_demo();
    // atomic_rc_demo();
    // mutex_demo();
    // poison_demo();
    // mpsc_demo();
}

