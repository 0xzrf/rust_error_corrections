pub fn build_data_to_send(data: Vec<u8>, theta: Option<u8>) -> Vec<u8> {
    let theta = theta.unwrap_or_else(|| {
        use rand::prelude::*;
        let mut rng = rand::rng();
        let mut array: Vec<u8> = (0..=255).collect::<Vec<u8>>();

        array.shuffle(&mut rng);

        *array.choose(&mut rng).unwrap()
    });

    let mut data_to_send: Vec<u8> = vec![];

    for (i, byte) in data.iter().enumerate() {
        data_to_send.push(build_polynomial(byte, i as u32, theta));
    }

    data_to_send
}

pub fn build_polynomial(data: &u8, index: u32, theta_val: u8) -> u8 {
    data * theta_val.pow(index)
}
