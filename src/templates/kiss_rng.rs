#![allow(dead_code)]

/*
* KISS = "keep it simple, stupid", but high quality random number generator
* http://www0.cs.ucl.ac.uk/staff/d.jones/GoodPracticeRNG.pdf -> "Use a good RNG and build it into your code"
* http://mathforum.org/kb/message.jspa?messageID=6627731
* https://de.wikipedia.org/wiki/KISS_(Zufallszahlengenerator)
*/

struct Rng32 {
    x: u32,
    y: u32,
    z: u32,
    c: u32,
}

impl Rng32 {
    pub fn new() -> Self {
        Rng32 {
            x: 123456789,
            y: 362436000,
            z: 521288629,
            c: 7654321,
        }
    }

    pub fn next(&mut self) -> u32 {
        self.x = self.x.overflowing_mul(69069).0.overflowing_add(12345).0;

        self.y = self.y ^ (self.y << 13);
        self.y = self.y ^ (self.y >> 17);
        self.y = self.y ^ (self.y << 5);

        let t = 698769069u64 * self.z as u64 + self.c as u64;
        self.c = (t >> 32) as u32;
        self.z = t as u32;
        self.x.overflowing_add(self.y).0.overflowing_add(self.z).0
    }
}

struct Rng64 {
    x: u64,
    y: u64,
    z: u64,
    c: u64,
}

impl Rng64 {
    pub fn new() -> Self {
        Rng64 {
            x: 1066149217761810u64,
            y: 362436362436362436u64,
            z: 1234567890987654321u64,
            c: 123456123456123456u64,
        }
    }

    pub fn next(&mut self) -> u64 {
        self.x = self
            .x
            .overflowing_mul(6906969069u64)
            .0
            .overflowing_add(1234567u64)
            .0;

        self.y = self.y ^ (self.y << 13);
        self.y = self.y ^ (self.y >> 17);
        self.y = self.y ^ (self.y << 43);

        let t = (self.z << 58).overflowing_add(self.c).0;
        self.c = self.z >> 6;
        self.z = self.z.overflowing_add(t).0;
        if self.z < t {
            self.c = self.c.overflowing_add(1).0;
        }
        self.x.overflowing_add(self.y).0.overflowing_add(self.z).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng32() {
        let mut rng32 = Rng32::new();
        assert_eq!(rng32.next(), 2079675107);
        assert_eq!(rng32.next(), 4185567647);
        assert_eq!(rng32.next(), 2837635843);
        assert_eq!(rng32.next(), 1057683632);
        assert_eq!(rng32.next(), 1715709901);
        assert_eq!(rng32.next(), 2813364309);
        assert_eq!(rng32.next(), 4260296908);
        assert_eq!(rng32.next(), 1972631519);
        assert_eq!(rng32.next(), 843352983);
        assert_eq!(rng32.next(), 4105814351);
    }

    #[test]
    fn test_rng64() {
        let mut rng64 = Rng64::new();
        assert_eq!(rng64.next(), 8932985056925012148u64);
        assert_eq!(rng64.next(), 5710300428094272059u64);
        assert_eq!(rng64.next(), 18342510866933518593u64);
        assert_eq!(rng64.next(), 14303636270573868250u64);
        assert_eq!(rng64.next(), 542381058189297533u64);
        assert_eq!(rng64.next(), 14201812252854837425u64);
        assert_eq!(rng64.next(), 6853720724624422285u64);
        assert_eq!(rng64.next(), 17679201207208679348u64);
        assert_eq!(rng64.next(), 18189539760622684491u64);
        assert_eq!(rng64.next(), 8128797625455304420u64);
    }
}
