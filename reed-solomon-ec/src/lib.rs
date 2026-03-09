pub fn build_data_to_send(data: Vec<u8>, theta: Option<u8>) -> Vec<u8> {
    let theta = theta.unwrap_or_else(|| {
        use rand::prelude::*;
        let mut rng = rand::rng();
        let mut array: Vec<u8> = (0..=255).collect::<Vec<u8>>();

        array.shuffle(&mut rng);

        *array.choose(&mut rng).unwrap()
    });

    let mut data_to_send: Vec<u8> = vec![];

    let mut i: usize = 0;
    while i < data.len() {
        data_to_send.push(build_polynomial(&data, theta.pow(i as u32)));
        i += 1;
    }

    data_to_send
}

pub fn build_polynomial(data: &[u8], theta_val: u8) -> u8 {
    let mut return_val = 0u8;

    for (i, byte) in data.iter().enumerate() {
        return_val += byte * theta_val.pow((i % 255) as u32);
    }

    return_val
}

pub fn gf_multiplication(data_a: u8, data_b: u8) -> u8 {
    todo!()
}

pub fn gf_add(data_a: u8, data_b: u8) -> u8 {
    todo!()
}
