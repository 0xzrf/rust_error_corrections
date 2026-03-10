pub fn build_data_to_send(data: Vec<u8>, parity_symbol: usize, theta: Option<u8>) -> Vec<u8> {
    let theta = theta.unwrap_or(2);

    let mut data_to_send: Vec<u8> = vec![];

    let mut i: usize = 0;
    while i < data.len() + parity_symbol {
        data_to_send.push(build_polynomial(&data, gf_pow(theta, (i % 255) as u8)));
        i += 1;
    }

    data_to_send
}

fn build_polynomial(data: &[u8], theta_val: u8) -> u8 {
    let mut return_val = 0u8;

    for (i, byte) in data.iter().enumerate() {
        return_val = gf_add(
            return_val,
            gf_multiplication(*byte, gf_pow(theta_val, (i % 255) as u8)),
        )
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

fn gf_add(data_a: u8, data_b: u8) -> u8 {
    data_a ^ data_b
}

fn gf_pow(a: u8, mut pow: u8) -> u8 {
    if a == 0 {
        return 0;
    };
    if pow == 0 {
        return 1;
    };
    let mut res = 1u8;

    while pow > 0 {
        res = gf_multiplication(res, a);
        pow -= 1;
    }

    res
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
}
