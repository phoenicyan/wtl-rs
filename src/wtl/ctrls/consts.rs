#![allow(dead_code)]
use winapi::shared::minwindef::*;

/*
(1) const define
^#define (\w+)\s+(\w+)
=>
pub const \1: UINT = \2;
*/

/*
 * Button Control Messages
 */
pub const BM_GETCHECK: UINT = 0x00F0;
pub const BM_SETCHECK: UINT = 0x00F1;
pub const BM_GETSTATE: UINT = 0x00F2;
pub const BM_SETSTATE: UINT = 0x00F3;
pub const BM_SETSTYLE: UINT = 0x00F4;
//#if(WINVER >= 0x0400)
pub const BM_CLICK: UINT = 0x00F5;
pub const BM_GETIMAGE: UINT = 0x00F6;
pub const BM_SETIMAGE: UINT = 0x00F7;
//#endif /* WINVER >= 0x0400 */
//#if(WINVER >= 0x0600)
pub const BM_SETDONTCLICK: UINT = 0x00F8;
//#endif /* WINVER >= 0x0600 */

//#if(WINVER >= 0x0400)
pub const BST_UNCHECKED: UINT = 0x0000;
pub const BST_CHECKED: UINT = 0x0001;
pub const BST_INDETERMINATE: UINT = 0x0002;
pub const BST_PUSHED: UINT = 0x0004;
pub const BST_FOCUS: UINT = 0x0008;
//#endif /* WINVER >= 0x0400 */

// pub const IMAGE_BITMAP: UINT = 0;
// pub const IMAGE_ICON: UINT = 1;
// pub const IMAGE_CURSOR: UINT = 2;
// //#if(WINVER >= 0x0400)
// pub const IMAGE_ENHMETAFILE: UINT = 3;



/*
 * Static Control Mesages
 */
pub const STM_SETICON: UINT = 0x0170;
pub const STM_GETICON: UINT = 0x0171;
//#if(WINVER >= 0x0400)
pub const STM_SETIMAGE: UINT = 0x0172;
pub const STM_GETIMAGE: UINT = 0x0173;
pub const STN_CLICKED: UINT = 0;
pub const STN_DBLCLK: UINT = 1;
pub const STN_ENABLE: UINT = 2;
pub const STN_DISABLE: UINT = 3;
//#endif /* WINVER >= 0x0400 */
pub const STM_MSGMAX: UINT = 0x0174;
//#endif /* !NOWINMESSAGES */


/*
 * Listbox Return Values
 */
pub const LB_OKAY: WORD = 0;
pub const LB_ERR: WORD = 0xFFFF;//(-1);
pub const LB_ERRSPACE: WORD = 0xFFFF - 1;//(-2);

/*
**  The idStaticPath parameter to DlgDirList can have the following values
**  ORed if the list box should show other details of the files along with
**  the name of the files;
*/
/* all other details also will be returned */


/*
 * Listbox Notification Codes
 */
pub const LBN_ERRSPACE: WORD = 0xFFFF - 1;//(-2);
pub const LBN_SELCHANGE: WORD = 1;
pub const LBN_DBLCLK: WORD = 2;
pub const LBN_SELCANCEL: WORD = 3;
pub const LBN_SETFOCUS: WORD = 4;
pub const LBN_KILLFOCUS: WORD = 5;
/*
 * Listbox messages
 */
pub const LB_ADDSTRING: UINT = 0x0180;
pub const LB_INSERTSTRING: UINT = 0x0181;
pub const LB_DELETESTRING: UINT = 0x0182;
pub const LB_SELITEMRANGEEX: UINT = 0x0183;
pub const LB_RESETCONTENT: UINT = 0x0184;
pub const LB_SETSEL: UINT = 0x0185;
pub const LB_SETCURSEL: UINT = 0x0186;
pub const LB_GETSEL: UINT = 0x0187;
pub const LB_GETCURSEL: UINT = 0x0188;
pub const LB_GETTEXT: UINT = 0x0189;
pub const LB_GETTEXTLEN: UINT = 0x018A;
pub const LB_GETCOUNT: UINT = 0x018B;
pub const LB_SELECTSTRING: UINT = 0x018C;
pub const LB_DIR: UINT = 0x018D;
pub const LB_GETTOPINDEX: UINT = 0x018E;
pub const LB_FINDSTRING: UINT = 0x018F;
pub const LB_GETSELCOUNT: UINT = 0x0190;
pub const LB_GETSELITEMS: UINT = 0x0191;
pub const LB_SETTABSTOPS: UINT = 0x0192;
pub const LB_GETHORIZONTALEXTENT: UINT = 0x0193;
pub const LB_SETHORIZONTALEXTENT: UINT = 0x0194;
pub const LB_SETCOLUMNWIDTH: UINT = 0x0195;
pub const LB_ADDFILE: UINT = 0x0196;
pub const LB_SETTOPINDEX: UINT = 0x0197;
pub const LB_GETITEMRECT: UINT = 0x0198;
pub const LB_GETITEMDATA: UINT = 0x0199;
pub const LB_SETITEMDATA: UINT = 0x019A;
pub const LB_SELITEMRANGE: UINT = 0x019B;
pub const LB_SETANCHORINDEX: UINT = 0x019C;
pub const LB_GETANCHORINDEX: UINT = 0x019D;
pub const LB_SETCARETINDEX: UINT = 0x019E;
pub const LB_GETCARETINDEX: UINT = 0x019F;
pub const LB_SETITEMHEIGHT: UINT = 0x01A0;
pub const LB_GETITEMHEIGHT: UINT = 0x01A1;
pub const LB_FINDSTRINGEXACT: UINT = 0x01A2;
pub const LB_SETLOCALE: UINT = 0x01A5;
pub const LB_GETLOCALE: UINT = 0x01A6;
pub const LB_SETCOUNT: UINT = 0x01A7;
//#if(WINVER >= 0x0400)
pub const LB_INITSTORAGE: UINT = 0x01A8;
pub const LB_ITEMFROMPOINT: UINT = 0x01A9;
//#endif /* WINVER >= 0x0400 */
//#if(_WIN32_WCE >= 0x0400)
//pub const LB_MULTIPLEADDSTRING: UINT = 0x01B1;
//#endif


//#if(_WIN32_WINNT >= 0x0501)
pub const LB_GETLISTBOXINFO: UINT = 0x01B2;
//#endif /* _WIN32_WINNT >= 0x0501 */

//#if(_WIN32_WINNT >= 0x0501)
pub const LB_MSGMAX: UINT = 0x01B3;
// #elif(_WIN32_WCE >= 0x0400)
// pub const LB_MSGMAX: UINT = 0x01B1;
// #elif(WINVER >= 0x0400)
// pub const LB_MSGMAX: UINT = 0x01B0;
// #else
// pub const LB_MSGMAX: UINT = 0x01A8;
//#endif

//#endif /* !NOWINMESSAGES */

//#ifndef NOWINSTYLES


/*
 * Listbox Styles
 */
// LONG??
pub const LBS_NOTIFY: UINT = 0x0001;
pub const LBS_SORT: UINT = 0x0002;
pub const LBS_NOREDRAW: UINT = 0x0004;
pub const LBS_MULTIPLESEL: UINT = 0x0008;
pub const LBS_OWNERDRAWFIXED: UINT = 0x0010;
pub const LBS_OWNERDRAWVARIABLE: UINT = 0x0020;
pub const LBS_HASSTRINGS: UINT = 0x0040;
pub const LBS_USETABSTOPS: UINT = 0x0080;
pub const LBS_NOINTEGRALHEIGHT: UINT = 0x0100;
pub const LBS_MULTICOLUMN: UINT = 0x0200;
pub const LBS_WANTKEYBOARDINPUT: UINT = 0x0400;
pub const LBS_EXTENDEDSEL: UINT = 0x0800;
pub const LBS_DISABLENOSCROLL: UINT = 0x1000;
pub const LBS_NODATA: UINT = 0x2000;
//#if(WINVER >= 0x0400)
pub const LBS_NOSEL: UINT = 0x4000;
//#endif /* WINVER >= 0x0400 */
pub const LBS_COMBOBOX: UINT = 0x8000;

pub const WS_VSCROLL: DWORD = 0x00200000;   // copied from um::winuser
pub const WS_BORDER: DWORD = 0x00800000;

pub const LBS_STANDARD:UINT = LBS_NOTIFY | LBS_SORT | WS_VSCROLL | WS_BORDER;


//#endif /* !NOWINSTYLES */



///////////////////////////////////////////////////
// combox
/*
 * Combo Box return Values
 */
// #define CB_OKAY             0
// #define CB_ERR              (-1)
// #define CB_ERRSPACE         (-2)


/*
 * Combo Box Notification Codes
 */
pub const CBN_ERRSPACE: WORD = 0xFFFF;//(-1);
pub const CBN_SELCHANGE: WORD = 1;
pub const CBN_DBLCLK: WORD = 2;
pub const CBN_SETFOCUS: WORD = 3;
pub const CBN_KILLFOCUS: WORD = 4;
pub const CBN_EDITCHANGE: WORD = 5;
pub const CBN_EDITUPDATE: WORD = 6;
pub const CBN_DROPDOWN: WORD = 7;
pub const CBN_CLOSEUP: WORD = 8;
pub const CBN_SELENDOK: WORD = 9;
pub const CBN_SELENDCANCEL: WORD = 10;

//#ifndef NOWINSTYLES

/*
 * Combo Box styles
 */
// #define CBS_SIMPLE            0x0001L
// #define CBS_DROPDOWN          0x0002L
// #define CBS_DROPDOWNLIST      0x0003L
// #define CBS_OWNERDRAWFIXED    0x0010L
// #define CBS_OWNERDRAWVARIABLE 0x0020L
// #define CBS_AUTOHSCROLL       0x0040L
// #define CBS_OEMCONVERT        0x0080L
// #define CBS_SORT              0x0100L
// #define CBS_HASSTRINGS        0x0200L
// #define CBS_NOINTEGRALHEIGHT  0x0400L
// #define CBS_DISABLENOSCROLL   0x0800L
// #if(WINVER >= 0x0400)
// #define CBS_UPPERCASE         0x2000L
// #define CBS_LOWERCASE         0x4000L
// #endif /* WINVER >= 0x0400 */

// #endif  /* !NOWINSTYLES */


/*
 * Combo Box messages
 */
// //#ifndef NOWINMESSAGES
// pub const CB_GETEDITSEL: UINT = 0x0140;
// pub const CB_LIMITTEXT: UINT = 0x0141;
// pub const CB_SETEDITSEL: UINT = 0x0142;
// pub const CB_ADDSTRING: UINT = 0x0143;
// pub const CB_DELETESTRING: UINT = 0x0144;
// pub const CB_DIR: UINT = 0x0145;
// pub const CB_GETCOUNT: UINT = 0x0146;
// pub const CB_GETCURSEL: UINT = 0x0147;
// pub const CB_GETLBTEXT: UINT = 0x0148;
// pub const CB_GETLBTEXTLEN: UINT = 0x0149;
// pub const CB_INSERTSTRING: UINT = 0x014A;
// pub const CB_RESETCONTENT: UINT = 0x014B;
// pub const CB_FINDSTRING: UINT = 0x014C;
// pub const CB_SELECTSTRING: UINT = 0x014D;
// pub const CB_SETCURSEL: UINT = 0x014E;
// pub const CB_SHOWDROPDOWN: UINT = 0x014F;
// pub const CB_GETITEMDATA: UINT = 0x0150;
// pub const CB_SETITEMDATA: UINT = 0x0151;
// pub const CB_GETDROPPEDCONTROLRECT: UINT = 0x0152;
// pub const CB_SETITEMHEIGHT: UINT = 0x0153;
// pub const CB_GETITEMHEIGHT: UINT = 0x0154;
// pub const CB_SETEXTENDEDUI: UINT = 0x0155;
// pub const CB_GETEXTENDEDUI: UINT = 0x0156;
// pub const CB_GETDROPPEDSTATE: UINT = 0x0157;
// pub const CB_FINDSTRINGEXACT: UINT = 0x0158;
// pub const CB_SETLOCALE: UINT = 0x0159;
// pub const CB_GETLOCALE: UINT = 0x015A;
// //#if(WINVER >= 0x0400)
// pub const CB_GETTOPINDEX: UINT = 0x015b;
// pub const CB_SETTOPINDEX: UINT = 0x015c;
// pub const CB_GETHORIZONTALEXTENT: UINT = 0x015d;
// pub const CB_SETHORIZONTALEXTENT: UINT = 0x015e;
// pub const CB_GETDROPPEDWIDTH: UINT = 0x015f;
// pub const CB_SETDROPPEDWIDTH: UINT = 0x0160;
// pub const CB_INITSTORAGE: UINT = 0x0161;
// //#if(_WIN32_WCE >= 0x0400)
// //pub const CB_MULTIPLEADDSTRING: UINT = 0x0163;
// //#endif
// //#endif /* WINVER >= 0x0400 */

// //#if(_WIN32_WINNT >= 0x0501)
pub const CB_GETCOMBOBOXINFO: UINT = 0x0164;
// //#endif /* _WIN32_WINNT >= 0x0501 */

// //#if(_WIN32_WINNT >= 0x0501)
// pub const CB_MSGMAX: UINT = 0x0165;
// //#elif(_WIN32_WCE >= 0x0400)
// //pub const CB_MSGMAX: UINT = 0x0163;
// //#elif(WINVER >= 0x0400)
// //pub const CB_MSGMAX: UINT = 0x0162;
// //#else
// //pub const CB_MSGMAX: UINT = 0x015B;
// //#endif
// //#endif  /* !NOWINMESSAGES */


//////////////////////////////////////////////////////////
// edit
/*
 * Edit Control Styles
 */
pub const ES_LEFT: DWORD = 0x0000;
pub const ES_CENTER: DWORD = 0x0001;
pub const ES_RIGHT: DWORD = 0x0002;
pub const ES_MULTILINE: DWORD = 0x0004;
pub const ES_UPPERCASE: DWORD = 0x0008;
pub const ES_LOWERCASE: DWORD = 0x0010;
pub const ES_PASSWORD: DWORD = 0x0020;
pub const ES_AUTOVSCROLL: DWORD = 0x0040;
pub const ES_AUTOHSCROLL: DWORD = 0x0080;
pub const ES_NOHIDESEL: DWORD = 0x0100;
pub const ES_OEMCONVERT: DWORD = 0x0400;
pub const ES_READONLY: DWORD = 0x0800;
pub const ES_WANTRETURN: DWORD = 0x1000;
//#if(WINVER >= 0x0400)
pub const ES_NUMBER: DWORD = 0x2000;
//#endif /* WINVER >= 0x0400 */


//#endif /* !NOWINSTYLES */

// /*
//  * Edit Control Notification Codes
//  */
pub const EN_SETFOCUS: WORD = 0x0100;
pub const EN_KILLFOCUS: WORD = 0x0200;
pub const EN_CHANGE: WORD = 0x0300;
pub const EN_UPDATE: WORD = 0x0400;
pub const EN_ERRSPACE: WORD = 0x0500;
pub const EN_MAXTEXT: WORD = 0x0501;
pub const EN_HSCROLL: WORD = 0x0601;
pub const EN_VSCROLL: WORD = 0x0602;

// #if(_WIN32_WINNT >= 0x0500)
// #define EN_ALIGN_LTR_EC     0x0700
// #define EN_ALIGN_RTL_EC     0x0701
// #endif /* _WIN32_WINNT >= 0x0500 */

// #if(WINVER >= 0x0400)
// /* Edit control EM_SETMARGIN parameters */
// #define EC_LEFTMARGIN       0x0001
// #define EC_RIGHTMARGIN      0x0002
// #define EC_USEFONTINFO      0xffff
// #endif /* WINVER >= 0x0400 */

// #if(WINVER >= 0x0500)
// /* wParam of EM_GET/SETIMESTATUS  */
// #define EMSIS_COMPOSITIONSTRING        0x0001

// /* lParam for EMSIS_COMPOSITIONSTRING  */
// #define EIMES_GETCOMPSTRATONCE         0x0001
// #define EIMES_CANCELCOMPSTRINFOCUS     0x0002
// #define EIMES_COMPLETECOMPSTRKILLFOCUS 0x0004
// #endif /* WINVER >= 0x0500 */

// #ifndef NOWINMESSAGES


// /*
//  * Edit Control Messages
//  */
pub const EM_GETSEL: UINT = 0x00B0;
pub const EM_SETSEL: UINT = 0x00B1;
pub const EM_GETRECT: UINT = 0x00B2;
pub const EM_SETRECT: UINT = 0x00B3;
pub const EM_SETRECTNP: UINT = 0x00B4;
pub const EM_SCROLL: UINT = 0x00B5;
pub const EM_LINESCROLL: UINT = 0x00B6;
pub const EM_SCROLLCARET: UINT = 0x00B7;
pub const EM_GETMODIFY: UINT = 0x00B8;
pub const EM_SETMODIFY: UINT = 0x00B9;
pub const EM_GETLINECOUNT: UINT = 0x00BA;
pub const EM_LINEINDEX: UINT = 0x00BB;
pub const EM_SETHANDLE: UINT = 0x00BC;
pub const EM_GETHANDLE: UINT = 0x00BD;
pub const EM_GETTHUMB: UINT = 0x00BE;
pub const EM_LINELENGTH: UINT = 0x00C1;
pub const EM_REPLACESEL: UINT = 0x00C2;
pub const EM_GETLINE: UINT = 0x00C4;
pub const EM_LIMITTEXT: UINT = 0x00C5;
pub const EM_CANUNDO: UINT = 0x00C6;
pub const EM_UNDO: UINT = 0x00C7;
pub const EM_FMTLINES: UINT = 0x00C8;
pub const EM_LINEFROMCHAR: UINT = 0x00C9;
pub const EM_SETTABSTOPS: UINT = 0x00CB;
pub const EM_SETPASSWORDCHAR: UINT = 0x00CC;
pub const EM_EMPTYUNDOBUFFER: UINT = 0x00CD;
pub const EM_GETFIRSTVISIBLELINE: UINT = 0x00CE;
pub const EM_SETREADONLY: UINT = 0x00CF;
pub const EM_SETWORDBREAKPROC: UINT = 0x00D0;
pub const EM_GETWORDBREAKPROC: UINT = 0x00D1;
pub const EM_GETPASSWORDCHAR: UINT = 0x00D2;
//#if(WINVER >= 0x0400)
pub const EM_SETMARGINS: UINT = 0x00D3;
pub const EM_GETMARGINS: UINT = 0x00D4;
pub const EM_SETLIMITTEXT: UINT = EM_LIMITTEXT;   /* ;win40 Name change */
pub const EM_GETLIMITTEXT: UINT = 0x00D5;
pub const EM_POSFROMCHAR: UINT = 0x00D6;
pub const EM_CHARFROMPOS: UINT = 0x00D7;
//#endif /* WINVER >= 0x0400 */

//#if(WINVER >= 0x0500)
pub const EM_SETIMESTATUS: UINT = 0x00D8;
pub const EM_GETIMESTATUS: UINT = 0x00D9;
//#endif /* WINVER >= 0x0500 */


// #endif /* !NOWINMESSAGES */

// /*
//  * EDITWORDBREAKPROC code values
//  */
// #define WB_LEFT            0
// #define WB_RIGHT           1
// #define WB_ISDELIMITER     2
