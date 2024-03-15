use hidapi::{DeviceInfo, HidApi};

const VID: u16 = 0x046D; // 046D:C54D
const PID: u16 = 0xC54D;
const USAGE: u16 = 1;
const USAGEPAGE: u16 = 65280;

fn get_device_info(hidapi: &HidApi) -> Result<&DeviceInfo, &'static str> {
    for device in hidapi.device_list() {
        if device.vendor_id() != VID
            || device.product_id() != PID
            || device.usage() != USAGE
            || device.usage_page() != USAGEPAGE
        {
            continue;
        } else {
            println!("path: {:?}", device.path());
            println!("{:04x}:{:04x}", device.vendor_id(), device.product_id());

            return Ok(device);
        }
    }
    return Err("invalid version");
}

fn main() {
    let loop_device_index = vec![
        0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xef,
    ];

    let hidapi: HidApi = match HidApi::new() {
        Ok(api) => api,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let device_info_path = match get_device_info(&hidapi) {
        Ok(device_info) => device_info.path(),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let hid_result = hidapi.open_path(device_info_path).unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x02, 0x0b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x02, 0x1b, 0x01, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x02, 0x1b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x02, 0x1b, 0x01, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x06, 0x0b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x0b, 0x00, 0x00, 0x00])
        .unwrap();

    let _ = hid_result.write(&[
        0x11, 0x01, 0x0c, 0x5b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    let _ = hid_result.write(&[
        0x11, 0x01, 0x0c, 0x5b, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x4b, 0x00, 0x00, 0x00])
        .unwrap();

    for i in loop_device_index.iter() {
        let _ = hid_result.write(&[
            0x11, 0x01, 0x0c, 0x5b, 0x01, 0x01, 0x00, *i, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    }

    for i in loop_device_index.iter() {
        let _ = hid_result.write(&[
            0x11, 0x01, 0x0c, 0x5b, 0x00, 0x01, 0x00, *i, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    }

    for i in loop_device_index.iter() {
        let _ = hid_result.write(&[
            0x11, 0x01, 0x0c, 0x5b, 0x00, 0x03, 0x00, *i, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    }

    for i in loop_device_index.iter() {
        let _ = hid_result.write(&[
            0x11, 0x01, 0x0c, 0x5b, 0x00, 0x04, 0x00, *i, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    }

    for i in loop_device_index.iter() {
        let _ = hid_result.write(&[
            0x11, 0x01, 0x0c, 0x5b, 0x00, 0x05, 0x00, *i, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    }

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0xbb, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x06, 0x1b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x1b, 0x01, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x2b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x4b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x1b, 0x01, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x2b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x4b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x06, 0x0b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x0b, 0x01, 0x00, 0x00])
        .unwrap();

    let _ = hid_result.write(&[
        0x11, 0x01, 0x0c, 0x5b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    let _ = hid_result.write(&[
        0x11, 0x01, 0x0c, 0x5b, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x4b, 0x00, 0x00, 0x00])
        .unwrap();

    for i in loop_device_index.iter() {
        let _ = hid_result.write(&[
            0x11, 0x01, 0x0c, 0x5b, 0x01, 0x01, 0x00, *i, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    }

    for i in loop_device_index.iter() {
        let _ = hid_result.write(&[
            0x11, 0x01, 0x0c, 0x5b, 0x00, 0x01, 0x00, *i, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    }

    for i in loop_device_index.iter() {
        let _ = hid_result.write(&[
            0x11, 0x01, 0x0c, 0x5b, 0x00, 0x02, 0x00, *i, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    }

    for i in loop_device_index.iter() {
        let _ = hid_result.write(&[
            0x11, 0x01, 0x0c, 0x5b, 0x00, 0x03, 0x00, *i, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    }

    for i in loop_device_index.iter() {
        let _ = hid_result.write(&[
            0x11, 0x01, 0x0c, 0x5b, 0x00, 0x04, 0x00, *i, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    }

    for i in loop_device_index.iter() {
        let _ = hid_result.write(&[
            0x11, 0x01, 0x0c, 0x5b, 0x00, 0x05, 0x00, *i, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    }

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0xbb, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0d, 0x0b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0d, 0x3b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x09, 0x5b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0a, 0x2b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x1b, 0x02, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0c, 0x2b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x06, 0x1b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0d, 0x1b, 0x00, 0x00, 0x00])
        .unwrap();

    let _ = hid_result.write(&[
        0x11, 0x01, 0x0d, 0x4b, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    let _ = hid_result.write(&[
        0x11, 0x01, 0x09, 0x6b, 0x00, 0x06, 0x40, 0x06, 0x40, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    let _ = hid_result.write(&[
        0x11, 0x01, 0x09, 0x7b, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    hid_result
        .write(&[0x10, 0x01, 0x0b, 0x2b, 0x01, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x0a, 0x0b, 0x00, 0x00, 0x00])
        .unwrap();

    let _ = hid_result.write(&[
        0x11, 0x01, 0x09, 0x6b, 0x00, 0x06, 0x40, 0x06, 0x40, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    let _ = hid_result.write(&[
        0x11, 0x01, 0x09, 0x7b, 0x00, 0x01, 0x40, 0x06, 0x40, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    hid_result
        .write(&[0x10, 0x01, 0x0b, 0x3b, 0x05, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x02, 0x0b, 0x00, 0x00, 0x00])
        .unwrap();

    hid_result
        .write(&[0x10, 0x01, 0x02, 0x1b, 0x01, 0x00, 0x00])
        .unwrap();
}