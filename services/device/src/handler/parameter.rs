//
//  Copyright 2024 Ram Flux, LLC.
//


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInit {
    pub uuid: String,
    pub account: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRes {
    pub osrng: String,
    pub pubkey: String,
    // pub signature: String,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceBinding {
    pub device_pubkey: Option<String>,
}
