#![no_std]

use ergot::endpoint;

endpoint! {
    LedEndpoint,
    [[u8; 3]; 2], // request: two colors
    (),           // response
    "led"
}
