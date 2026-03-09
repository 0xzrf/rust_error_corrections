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

fn build_polynomial(data: &[u8], theta_val: u8) -> u8 {
    let mut return_val = 0u8;

    for (i, byte) in data.iter().enumerate() {
        return_val += byte * theta_val.pow((i % 255) as u32);
    }

    return_val
}

fn gf_multiplication(mut data_a: u8, mut data_b: u8) -> u8 {
    let mut result = 0u8;

    while data_b != 0 {
        if (data_b & 1) != 0 {
            result ^= data_a;
        }

        data_a = multiple_by_x(data_a);
        data_b >>= 1;
    }

    result
}
fn multiple_by_x(a: u8) -> u8 {
    if (a & 0x80) != 0 {
        (a << 1) ^ 0x1d
    } else {
        a << 1
    }
}

fn get_bits(data: u8) -> Vec<u8> {
    (0..8).map(|i| get_bit(i, data)).collect::<Vec<u8>>()
}

#[inline(always)]
fn get_bit(index: usize, data: u8) -> u8 {
    (data >> index) & 1
}

fn gf_add(data_a: u8, data_b: u8) -> u8 {
    data_a ^ data_b
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_gf_multiplication() {
        let a = 7;
        let b = 2;

        assert_eq!(14, gf_multiplication(a, b));
    }

    #[test]
    fn test_get_bits() {
        let data = 0;

        assert_eq!([0u8; 8], *get_bits(data).as_array().unwrap());
    }
}
