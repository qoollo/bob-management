// Some structs that must be redefined for transpiling without changing actual types on backend

use tsync::tsync;

#[tsync]
pub type Hostname = String;

// Same as in `backend/src/connector/dto.rs`
// Imported for bindings generation (tsync doesn't respect serde(rename))

/// BOB's Node interface
#[tsync]
pub struct DTONode {
    pub name: String,

    pub address: String,

    pub vdisks: Option<Vec<DTOVDisk>>,
}

/// BOB's Node Configuration interface
#[tsync]
pub struct DTONodeConfiguration {
    pub blob_file_name_prefix: Option<String>,

    pub root_dir_name: Option<String>,
}

/// BOB's VDisk interface
#[tsync]
pub struct DTOVDisk {
    pub id: i32,

    pub replicas: Option<Vec<DTOReplica>>,
}

/// BOB's Replica interface
#[tsync]
pub struct DTOReplica {
    pub node: String,

    pub disk: String,

    pub path: String,
}
