use std::collections::BTreeSet;
use std::iter::FromIterator;

use rand::Rng;
use scapegoat::SgSet;

// Random Test Data ----------------------------------------------------------------------------------------------------

pub struct RandTestData {
    pub keys: Vec<usize>,
    pub get_idxs: Vec<usize>,
    pub remove_idxs: Vec<usize>,
}

impl RandTestData {
    pub fn new(size: usize) -> Self {
        let mut rng = rand::rng();

        RandTestData {
            keys: (0..size).map(|_| rng.random_range(0..usize::MAX)).collect(),
            get_idxs: (0..size).map(|_| rng.random_range(0..size)).collect(),
            remove_idxs: (0..size).map(|_| rng.random_range(0..size)).collect(),
        }
    }
}

// Init Random Test Data (Immutable, Global) ---------------------------------------------------------------------------

lazy_static::lazy_static! {
    pub static ref RAND_100: RandTestData = RandTestData::new(100);
    pub static ref RAND_1_000: RandTestData = RandTestData::new(1_000);
    pub static ref RAND_10_000: RandTestData = RandTestData::new(10_000);
}

lazy_static::lazy_static! {
    pub static ref SGS_100_RAND: SgSet<usize, 100> = SgSet::from_iter(RAND_100.keys.clone());
    pub static ref SGS_1_000_RAND: SgSet<usize, 1_000> = SgSet::from_iter(RAND_1_000.keys.clone());
    pub static ref SGS_10_000_RAND: SgSet<usize, 10_000> = SgSet::from_iter(RAND_10_000.keys.clone());

    pub static ref STD_100_RAND: BTreeSet<usize> = BTreeSet::from_iter(RAND_100.keys.clone());
    pub static ref STD_1_000_RAND: BTreeSet<usize> = BTreeSet::from_iter(RAND_1_000.keys.clone());
    pub static ref STD_10_000_RAND: BTreeSet<usize> = BTreeSet::from_iter(RAND_10_000.keys.clone());
}

// Sequential Test Data ------------------------------------------------------------------------------------------------

pub struct SeqTestData {
    pub keys: Vec<usize>,
    pub get_idxs: Vec<usize>,
    pub remove_idxs: Vec<usize>,
}

impl SeqTestData {
    pub fn new(size: usize) -> Self {
        SeqTestData {
            keys: (0..size).collect(),
            get_idxs: (0..size).collect(),
            remove_idxs: (0..size).collect(),
        }
    }
}

// Init Sequential Test Data (Immutable, Global) -----------------------------------------------------------------------

lazy_static::lazy_static! {
    pub static ref SEQ_100: SeqTestData = SeqTestData::new(100);
    pub static ref SEQ_1_000: SeqTestData = SeqTestData::new(1_000);
    pub static ref SEQ_10_000: SeqTestData = SeqTestData::new(10_000);
}

lazy_static::lazy_static! {
    pub static ref SGS_100_SEQ: SgSet<usize, 100> = SgSet::from_iter(SEQ_100.keys.clone());
    pub static ref SGS_1_000_SEQ: SgSet<usize, 1_000> = SgSet::from_iter(SEQ_1_000.keys.clone());
    pub static ref SGS_10_000_SEQ: SgSet<usize, 10_000> = SgSet::from_iter(SEQ_10_000.keys.clone());

    pub static ref STD_100_SEQ: BTreeSet<usize> = BTreeSet::from_iter(SEQ_100.keys.clone());
    pub static ref STD_1_000_SEQ: BTreeSet<usize> = BTreeSet::from_iter(SEQ_1_000.keys.clone());
    pub static ref STD_10_000_SEQ: BTreeSet<usize> = BTreeSet::from_iter(SEQ_10_000.keys.clone());
}
