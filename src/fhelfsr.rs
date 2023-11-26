use tfhe::FheUint8;

pub struct FHELFSR {
    initial_state: FheUint8,
    state: FheUint8
}

impl FHELFSR {
    pub fn new(initial_state: FheUint8) -> Self {
        FHELFSR {
            initial_state: initial_state.clone(),
            state: initial_state.clone(),
        }
    }

    pub fn next(&mut self) -> FheUint8 {
        // Taps: 8 6 5 4
        let bit: FheUint8 = self.state.clone() ^ (self.state.clone() >> 2u8) ^ (self.state.clone() >> 3u8) ^ (self.state.clone() >> 4u8) & 1u8;
        self.state = (self.state.clone() >> 1u8) | (bit << 7u8);
        self.state.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::FHELFSR;
    use std::{fs::File, io::{BufReader, BufRead}};
    use tfhe::prelude::{FheTryEncrypt, FheDecrypt};
    use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint8};

    #[test]
    fn test_lsfr() {
        let config = ConfigBuilder::all_disabled()
            .enable_default_integers()
            .build();
        let (keys, server_keys) = generate_keys(config);
        set_server_key(server_keys);
        let clear_initial_state = 0x01u8;

        let mut fhe_initial_state = FheUint8::try_encrypt(clear_initial_state, &keys).unwrap();
    
        let file = File::open("lfsr-test.txt").unwrap();

        let buf = BufReader::new(file);
        let outputs = buf.lines()
            .map(|l| l.unwrap().parse().unwrap())
            .collect::<Vec<u8>>();

        assert!(outputs[0] == 0x1);

        let mut lfsr = FHELFSR::new(fhe_initial_state);

        println!("Starting LFSR...");
        for i in 1..4 {
            let decrypted  = lfsr.next().decrypt(&keys);
            println!("\t{} =? {}", outputs[i as usize], decrypted);
            assert!(outputs[i as usize] == decrypted);
        }
    
    }
}