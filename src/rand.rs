// Trait should have a new, seed, rand method.
//
const TWISTER_W: u32 = 32;
const TWISTER_N: u32 = 624;
const TWISTER_M: u32 = 397;
const TWISTER_R: u32 = 31;

const TWISTER_A: u32 = 0x9908B0DF;
const TWISTER_S: u32 = 7;
const TWISTER_B: u32 = 0x9D2C5680;

const TWISTER_T: u32 = 15;
const TWISTER_C: u32 = 0xEFC60000;

const TWISTER_L: u32 = 18;

struct Twister {}
