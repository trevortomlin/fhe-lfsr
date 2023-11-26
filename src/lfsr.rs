pub struct LFSR {
    initial_state: u8,
    state: u8
}

impl LFSR {
    pub fn new(initial_state: u8) -> Self {
        LFSR {
            initial_state,
            state: initial_state,
        }
    }

    pub fn next(&mut self) -> u8 {
        // Taps: 8 6 5 4
        let bit: u8 = self.state ^ (self.state >> 2) ^ (self.state >> 3) ^ (self.state >> 4) & 1;
        self.state = (self.state >> 1) | (bit << 7);
        self.state
    }

    pub fn period(&self) -> u8 {
        let mut local_state = self.initial_state;
        let bit: u8 = local_state ^ (local_state >> 2) ^ (local_state >> 3) ^ (local_state >> 4) & 1;
        local_state = (local_state >> 1) | (bit << 7);
        let mut period = 1;

        loop {
            if local_state == self.initial_state {
                break;
            }
            
            let bit: u8 = local_state ^ (local_state >> 2) ^ (local_state >> 3) ^ (local_state >> 4) & 1;
            local_state = (local_state >> 1) | (bit << 7);

            period+=1;

        }
        period

    }
}

#[cfg(test)]
mod tests {
    use super::LFSR;
    use std::{fs::File, io::{BufReader, BufRead}};

    #[test]
    fn test_period() {
        let lfsr = LFSR::new(0x01);
        assert!(lfsr.period() == 255);
    }

    #[test]
    fn test_lsfr() {
        let file = File::open("lfsr-test.txt").unwrap();

        let buf = BufReader::new(file);
        let outputs = buf.lines()
            .map(|l| l.unwrap().parse().unwrap())
            .collect::<Vec<u8>>();

        assert!(outputs[0] == 0x1);

        let mut lfsr = LFSR::new(0x01);

        for i in 1..lfsr.period() - 1 {
            assert!(outputs[i as usize] == lfsr.next());
        }


    }
}