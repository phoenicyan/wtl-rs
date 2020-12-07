
use std::rc::Rc;
use std::fmt;
use std::cmp::Ordering;

use winapi::shared::minwindef::*;

use atl::{Event};
////////////////////////////////////////////////////////////////////
// handler
// all structs who want to process the win message need to impl this
// and then called to put all the closure into Dialog.handlers
// all message process structs must live as long as MessageLoop
// all message process structs will be create and call the register function with MessageLoop as param
// dynamic generated code
// pub trait HandlerRegister {
//     fn register(&mut self,&MessageLoop);
// }

#[repr(C,packed)]
pub struct HandleKey {
    priority:u16,
    code:WORD,
    id  :WORD,
    msg :WORD,
}

impl HandleKey {
    #[inline(always)]
    pub fn new_msg(msg:UINT)->HandleKey{
        HandleKey{
            msg:msg as WORD,
            id:0,
            code:0,
            priority:0,
            //hwnd:h,
        }
    }

    #[inline(always)]
    pub fn new(msg:UINT,id:u16,code:u16)->HandleKey{
        HandleKey{
            msg:msg as WORD,
            id:id,
            code:code,
            priority:0,
            //hwnd:h,
        }
    }

    #[inline(always)]
    pub fn key(&self)->u64{
        unsafe{
            *(self as *const _ as *const u64)
        }
    }
}

/// Priority which is ranging from 0 ~ 65535. If more than one handler for the same message/command/notify,
/// then the wtl-rs will call them by priority,the smaller number the higher priority.
/// 0 ~ 99 been reserved for wtl-rs system,so user can use 100 ~ 65535
//this only use for sorting algorithm ,after that it will be set to zero for search algorithm
//so user can't set priority at runtime

/// the sort algorithm convert *self to u64, so only little endian machine can be sort correct
#[repr(C,packed)]
pub struct Handler<T> {
    pub priority:u16,
    pub code:WORD,
    pub id  :WORD,
    pub msg :WORD,
    
    pub call:Rc<dyn Fn(&mut Event,&mut T)>,
}

impl<T> fmt::Display for Handler<T>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe{write!(f, "Handler:\nmsg:{}\nid:{}\ncode:{}\npriority:{}\n", self.msg,self.id,self.code,self.priority)}
    }
}

impl<T> Handler<T> {
    #[inline(always)]
    pub fn new<F>(msg:UINT,id:u16,code:u16,priority:u16,f:F)->Handler<T> where F:Fn(&mut Event,&mut T) + 'static {
        Handler{
            msg:msg as WORD,
            id :id,
            code:code,
            priority:priority,
            call:Rc::new(f),
        }
    }

    #[inline(always)]
    pub fn key(&self)->u64{
        unsafe{
            *(self as *const _ as *const u64)
        }
    }

    // for sort
    pub fn cmp(&self,other:&Self)->Ordering {
        self.key().cmp(&other.key())
    }
}

/// only handle the priority setting method
pub struct HandlerPriority<'a,T:'a> {
    h: &'a mut Handler<T>
}

/// priority ranges from 0 ~ 65535
impl <'a,T> HandlerPriority<'a,T> {
    #[inline(always)]
    pub fn new(h:&'a mut Handler<T>)->HandlerPriority<'a,T>{
        HandlerPriority{
            h:h,
        }
    }

    /// low priority,p range from 0 ~ 32767(0x7FFF),and real priority range from 32768 ~ 65535
    pub fn set_user_priority(&mut self,p:u16){
        debug_assert!(p < 8000);
        self.h.priority = p + 8000;        //32767
    }

    /// high priority, p range from 0 ~ 32767,and real priority range from 0 ~ 32767
    pub fn set_system_priority(&mut self,p:u16){
        debug_assert!(p < 8000);
        self.h.priority = p;
    }
}