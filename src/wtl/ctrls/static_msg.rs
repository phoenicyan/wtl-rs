
#![allow(dead_code,unused_imports)]
// https://msdn.microsoft.com/en-us/library/windows/desktop/bb760784(v=vs.85).aspx
use winapi::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;

//use super::super::cwindow::*;
//use super::consts::*;
//use super::Event;

use atl::{Handler,Event};
//use atl::CWindow;

pub struct StcMsg <'a,T:'a> {
	id:WORD,
    h:&'a mut Vec<Handler<T>>,
}

impl<'a,T> StcMsg<'a,T> {
	pub fn new(id:WORD,h:&'a mut Vec<Handler<T>>)->StcMsg<'a,T>{
		StcMsg{
			id:id,
			h:h,
		}
	}
}


impl<'a,T> StcMsg<'a,T> {

}