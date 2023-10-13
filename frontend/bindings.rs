// Some structs that must be redefined for transpiling without changing actual types on backend

use tsync::tsync;

#[tsync]
pub type Hostname = String;
