#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

pub const IAC:  u8  = 255; /* interpret as command: */
pub const DONT: u8  = 254; /* you are not to use option */
pub const DO:   u8  = 253; /* please, you use option */
pub const WONT: u8  = 252; /* I won't use option */
pub const WILL: u8  = 251; /* I will use option */
pub const SB:   u8  = 250; /* interpret as subnegotiation */
pub const GA:   u8  = 249; /* you may reverse the line */
pub const EL:   u8  = 248; /* erase the current line */
pub const EC:   u8  = 247; /* erase current character */
pub const AYT:  u8  = 246; /* are you there */
pub const AO:   u8  = 245; /* abort output--but let prog finish */
pub const IP:   u8  = 244; /* interrupt process--permanently */
pub const BREAK:u8  = 243; /* break */
pub const DM:   u8  = 242; /* data mark--for connect. cleaning */
pub const NOP:  u8  = 241; /* nop */
pub const SE:   u8  = 240; /* end subnegotiation */
pub const EOR:  u8  = 239; /* end of record (transparent mode) */
pub const ABORT:u8  = 238; /* abort process */
pub const SUSP: u8  = 237; /* suspend process */
pub const xEOF: u8  = 236; /* end of file: EOF is already used... */

pub const SYNCH:u8  = 242; /* for telfunc calls */

pub const TELCMD_FIRST: u8 = xEOF;
pub const TELCMD_LAST:  u8 = IAC;

/* telnet options */
pub const TELOPT_BINARY:        u8 = 0;  /* 8-bit data path */
pub const TELOPT_ECHO:          u8 = 1;  /* echo */
pub const TELOPT_RCP:           u8 = 2;  /* prepare to reconnect */
pub const TELOPT_SGA:           u8 = 3;  /* suppress go ahead */
pub const TELOPT_NAMS:          u8 = 4;  /* approximate message size */
pub const TELOPT_STATUS:        u8 = 5;  /* give status */
pub const TELOPT_TM:            u8 = 6;  /* timing mark */
pub const TELOPT_RCTE:          u8 = 7;  /* remote controlled transmission and echo */
pub const TELOPT_NAOL:          u8 = 8;  /* negotiate about output line width */
pub const TELOPT_NAOP:          u8 = 9;  /* negotiate about output page size */
pub const TELOPT_NAOCRD:        u8 = 10; /* negotiate about CR disposition */
pub const TELOPT_NAHTS:         u8 = 11; /* negotiate about horizontal tabstops */
pub const TELOPT_NAOHTD:        u8 = 12; /* negotiate about horizontal tab disposition*/
pub const TELOPT_NAOFFD:        u8 = 13; /* negotiate about formfeed disposition */
pub const TELOPT_NAOVTS:        u8 = 14; /* negotiate about vertical tab stops */
pub const TELOPT_NAOVTD:        u8 = 15; /* negotiate about vertical tab disposition */
pub const TELOPT_NAOLFD:        u8 = 16; /* negotiate about output LF disposition */
pub const TELOPT_XASCII:        u8 = 17; /* extended ascic character set */
pub const TELOPT_LOGOUT:        u8 = 18; /* force logout */
pub const TELOPT_BM:            u8 = 19; /* byte macro */
pub const TELOPT_DET:           u8 = 20; /* data entry terminal */
pub const TELOPT_SUPDUP:        u8 = 21; /* supdup protocol */
pub const TELOPT_SUPDUPOOUTPUT: u8 = 22; /* supdup output */
pub const TELOPT_SNDLOC:        u8 = 23; /* send location */
pub const TELOPT_TTYPE:         u8 = 24; /* terminal type */
pub const TELOPT_EOR:           u8 = 25; /* end or record */
pub const TELOPT_TUID:          u8 = 26; /* TACACS user identification */
pub const TELOPT_OUTMRK:        u8 = 27; /* output marking */
pub const TELOPT_TTYLOC:        u8 = 28; /* terminal location number */
pub const TELOPT_3270REGIME:    u8 = 29; /* 3270 regime */
pub const TELOPT_X3PAD:         u8 = 30; /* X.3 PAD*/
pub const TELOPT_NAWS:          u8 = 31; /* window size */
pub const TELOPT_TSPEED:        u8 = 32; /* terminal speed */
pub const TELOPT_LFLOW:         u8 = 33; /* remote flow control */
pub const TELOPT_LINEMODE:      u8 = 34; /* Linemode option */
pub const TELOPT_XDISPLOC:      u8 = 35; /* X Display Location */
pub const TELOPT_OLD_ENVIRON:   u8 = 36; /* Old - Environment variables */
pub const TELOPT_AUTHENTICATION:u8 = 37; /* Authenticate */
pub const TELOPT_ENCRYPT:       u8 = 38; /* Encryption option */
pub const TELOPT_NEW_ENVIRON:   u8 = 39; /* New - Environment variables */
pub const TELOPT_TN3270E:       u8 = 40; /* RFC2355 - TN3270 Enhancements */
pub const TELOPT_CHARSET:       u8 = 42; /* RFC2066 - Charset */
pub const TELOPT_COMPORT:       u8 = 44; /* RFC2217 - Com Port Control */
pub const TELOPT_KERMIT:        u8 = 47; /* RFC2840 - Kermit */

pub const TELOPT_EXOPL:         u8 = 255; /* extended-options-list */

pub const TELOPT_FIRST: u8 = TELOPT_BINARY;
pub const TELOPT_LAST: u8 = TELOPT_KERMIT;

/* sub-option qualifiers */
pub const TELQUAL_IS:   u8 = 0; /* option is ... */
pub const TELQUAL_SEND: u8 = 1; /* send option */
pub const TELQUAL_INFO: u8 = 2; /* ENVIRON: informational version of IS */
pub const TELQUAL_REPLY:u8 = 2; /* AUTHENTICATION: client version of IS */
pub const TELQUAL_NAME: u8 = 3; /* AUTHENTICATION: client version of IS */

pub const LFLOW_OFF:         u8 = 0; /* Disable remote flow control */
pub const LFLOW_ON:          u8 = 1; /* Enable remote flow control */
pub const LFLOW_RESTART_ANY: u8 = 2; /* Restart output on any car */
pub const LFLOW_RESTART_XON: u8 = 3; /* Restart output only on XON */

/* LINEMODE suboptions */
pub const LM_MODE:        u8 = 1;
pub const LM_FORWARDMASK: u8 = 2;
pub const LM_SLC:         u8 = 3;

pub const MODE_EDIT:      u8 = 0x01;
pub const MODE_TRAPSIG:   u8 = 0x02;
pub const MODE_ACK:       u8 = 0x04;
pub const MODE_SOFT_TAB:  u8 = 0x08;
pub const MODE_LIT_ECHO:  u8 = 0x10;

pub const MODE_MASK:      u8 = 0x1f;

/* Not part of protocol, but need to simplify things */
pub const MODE_FLOW:      u32 = 0x0100;
pub const MODE_ECHO:      u32 = 0x0200;
pub const MODE_INBIN:     u32 = 0x0400;
pub const MODE_OUTBIN:    u32 = 0x0800;
pub const MODE_FORCE:     u32 = 0x1000;

pub const SLC_SYNCH: u8 = 1;
pub const SLC_BRK:   u8 = 2;
pub const SLC_IP:    u8 = 3;
pub const SLC_AO:    u8 = 4;
pub const SLC_AYT:   u8 = 5;
pub const SLC_EOR:   u8 = 6;
pub const SLC_ABORT: u8 = 7;
pub const SLC_EOF:   u8 = 8;
pub const SLC_SUSP:  u8 = 9;
pub const SLC_EC:    u8 = 10;
pub const SLC_EL:    u8 = 11;
pub const SLC_EW:    u8 = 12;
pub const SLC_RP:    u8 = 13;
pub const SLC_LNEXT: u8 = 14;
pub const SLC_XON:   u8 = 15;
pub const SLC_XOFF:  u8 = 16;
pub const SLC_FORW1: u8 = 17;
pub const SLC_FORW2: u8 = 18;
pub const SLC_MCL:   u8 = 19;
pub const SLC_MCR:   u8 = 20;
pub const SLC_MCWL:  u8 = 21;
pub const SLC_MCWR:  u8 = 22;
pub const SLC_MCBOL: u8 = 23;
pub const SLC_MCEOL: u8 = 24;
pub const SLC_INSRT: u8 = 25;
pub const SLC_OVER:  u8 = 26;
pub const SLC_ECR:   u8 = 27;
pub const SLC_EWR:   u8 = 28;
pub const SLC_EBOL:  u8 = 29;
pub const SLC_EEOL:  u8 = 30;

pub const NSLC:      u8 = 30;


pub const SLC_NOSUPPORT:  u8 = 0;
pub const SLC_CANTCHANGE: u8 = 1;
pub const SLC_VARIABLE:   u8 = 2;
pub const SLC_DEFAULT:    u8 = 3;
pub const SLC_LEVELBITS:  u8 = 0x03;

pub const SLC_FUNC:  u8 = 0;
pub const SLC_FLAGS: u8 = 1;
pub const SLC_VALUE: u8 = 2;

pub const SLC_ACK:      u8 = 0x80;
pub const SLC_FLUSHIN:  u8 = 0x40;
pub const SLC_FLUSHOUT: u8 = 0x20;

pub const OLD_ENV_VAR:   u8 = 1;
pub const OLD_ENV_VALUE: u8 = 0;
pub const NEW_ENV_VAR:   u8 = 0;
pub const NEW_ENV_VALUE: u8 = 1;
pub const ENV_ESC:       u8 = 2;
pub const ENV_USERVAR:   u8 = 3;

/* who is authenticating to whom */
pub const AUTH_WHO_CLIENT: u8 = 0; /* Client authenticating server */
pub const AUTH_WHO_SERVER: u8 = 1; /* Server authenticating client */
pub const AUTH_WHO_MASK:   u8 = 1;

/* amount of authentication done */
pub const AUTH_HOW_ONE_WAY: u8 = 0;
pub const AUTH_HOW_MUTUAL:  u8 = 2;
pub const AUTH_HOW_MASK:    u8 = 2;

/* should we be encrypting? (not yet formally standardized) */
pub const AUTH_ENCRYPT_OFF:  u8 = 0;
pub const AUTH_ENCRYPT_ON:   u8 = 4;
pub const AUTH_ENCRYPT_MASK: u8 = 4;

pub const AUTHTYPE_NULL:        u8 = 0;
pub const AUTHTYPE_KERBEROS_V4: u8 = 1;
pub const AUTHTYPE_KERBEROS_V5: u8 = 2;
pub const AUTHTYPE_SPX:         u8 = 3;
pub const AUTHTYPE_MINK:        u8 = 4;
pub const AUTHTYPE_SRA:         u8 = 6;
pub const AUTHTYPE_CNT:         u8 = 7;

pub const AUTHTYPE_TEST:        u8 = 99;

/* ENCRYPTION supboptions */
pub const ENCRYPT_IS:        u8 = 0; /* I pick encryption type ... */
pub const ENCRYPT_SUPPORT:   u8 = 1; /* I support encryption types ... */
pub const ENCRYPT_REPLY:     u8 = 2; /* Initial setup response */
pub const ENCRYPT_START:     u8 = 3; /* Am starting to send encrypted */
pub const ENCRYPT_END:       u8 = 4; /* Am ending encrypted */
pub const ENCRYPT_REQSTART:  u8 = 5; /* Request you start encrypting */
pub const ENCRYPT_REQEND:    u8 = 6; /* Request you send encrypting */
pub const ENCRYPT_ENC_KEYID: u8 = 7;
pub const ENCRYPT_DEC_KEYID: u8 = 8;
pub const ENCRYPT_CNT:       u8 = 9;

pub const ENCTYPE_ANY:       u8 = 0;
pub const ENCTYPE_DES_CFB64: u8 = 1;
pub const ENCTYPE_DES_OFB64: u8 = 2;
pub const ENCTYPE_CNT:       u8 = 3;

/* communicate with client to set in bbs mode */
pub const TELNET_INIT: [u8; 15] = [
    IAC, DO, TELOPT_TTYPE,
    IAC, SB, TELOPT_TTYPE, TELQUAL_SEND, IAC, SE,
    IAC, WILL, TELOPT_ECHO,
    IAC, WILL, TELOPT_SGA,
];

pub const TELNET_RESIZABLE: [u8; 3] = [
    IAC, DO, TELOPT_NAWS,
];
