#![allow(non_snake_case)]
extern crate time;

use std::time::SystemTime;
use time::Timespec;
use std::net::{IpAddr, Ipv4Addr};

pub struct User {
    pub userid:      [u8; 13],
    pub realname:    [u8; 32],
    pub nickname:    [u8; 32],
    pub passwd:      [u8; 14],

    pub userLevel:   u32,
    pub ufo:         u8,
    pub signature:   u8,
    pub birthday:    [u16; 3],
    pub sex:         u8,
    pub money:       u32,
    pub gold:        u32,
    pub numLogins:   u32,
    pub numPosts:    u32,
    pub numBadPosts: u32,
    pub numEmails:   u32,

    pub firstLogin:  Timespec,
    pub lastLogin:   Timespec,

    // If account suspended, the time is in futurer,
    // otherwise, the valid date.
    // set to Timespec(0, 0) means not valid yet.
    pub timeValid:   Timespec,
    pub lastIp:      IpAddr,
}

impl User {
    pub fn new() -> User {
        User {..Default::default()}
    }
}

impl Default for User {
    fn default() -> User {
        User {
            userid:      [0; 13],
            realname:    [0; 32],
            nickname:    [0; 32],
            passwd:      [0; 14],
            userLevel:   0,
            ufo:         0,
            signature:   0,
            birthday:    [1970, 1, 1],
            sex:         0,
            money:       0,
            gold:        0,
            numLogins:   0,
            numPosts:    0,
            numBadPosts: 0,
            numEmails:   0,
            firstLogin:  Timespec::new(0, 0),
            lastLogin:   Timespec::new(0, 0),
            timeValid:   Timespec::new(0, 0),
            lastIp:      IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        }
    }
}

pub struct UserDetail {
    pub userid:      [u8; 13],
    pub email:       [u8; 60],
    pub address:     [u8; 60],
    pub career:      [u8; 50],
    pub phone:       [u8; 20],
    pub updateTime:  Timespec,
}
