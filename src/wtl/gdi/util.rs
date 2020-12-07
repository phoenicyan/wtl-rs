

#[macro_export]
macro_rules! extract_opt_by_default {
	($d:expr,$v:ident,$t:ty) => {
	    {
    		match $v {
    			Some(v1) => (v1),
    			None => ($d as $t),
    		}
		}
	};
}

#[macro_export]
macro_rules! extract_opt_by_null {
	($v:ident,$t:ty) => {
	    {
    		match $v {
    			Some(v1) => (v1),
    			None => (0 as $t),
    		}
		}
	};
}