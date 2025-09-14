#![no_main]
#![feature(map_first_last)]
#![feature(map_try_insert)]

use std::cmp::Ordering;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::ops::Bound::Included;
use std::ops::Range;

use libfuzzer_sys::{
    arbitrary::{unstructured::Int, Arbitrary, Unstructured},
    fuzz_target,
};

use escapegoat::map_types::Entry as SgEntry;
use escapegoat::SgMap;
use std::collections::btree_map::Entry as BtEntry;
use std::collections::BTreeMap;

const CAPACITY: usize = 2048;

// Map's Entry ---------------------------------------------------------------------------------------------------------

// Top-level Entry
#[derive(Arbitrary, Debug)]
enum MapEntry<K: Ord + Debug, V: Debug> {
    // Methods
    AndModify { rand_val: V },
    Key,
    OrDefault,
    OrInsert { default: V },
    OrInsertWith { rand_val: V },
    OrInsertWithKey { rand_key: K },
    Occupied { inner: MapOccupiedEntry<V> },
    Vacant { inner: MapVacantEntry<V> },
}

// Occupied
#[derive(Arbitrary, Debug)]
enum MapOccupiedEntry<V: Debug> {
    Get,
    GetMut,
    Insert { val: V },
    IntoMut,
    Key,
    Remove,
    RemoveEntry,
}

// Vacant
#[derive(Arbitrary, Debug)]
enum MapVacantEntry<V: Debug> {
    Insert { val: V },
    IntoKey,
    Key,
}

// Map -----------------------------------------------------------------------------------------------------------------

#[derive(Arbitrary, Debug)]
enum MapMethod<K: Ord + Debug, V: Debug> {
    // APIs ------------------------------------------------------------------------------------------------------------
    Append { other: Vec<(K, V)> },
    // capacity() returns a constant. Omitted, irrelevant coverage.
    Clear,
    ContainsKey { key: K },
    Entry { key: K, entry: MapEntry<K, V> },
    FirstEntry,
    FirstKey,
    FirstKeyValue,
    Get { key: K },
    GetKeyValue { key: K },
    GetMut { key: K },
    Insert { key: K, val: V },
    IsEmpty,
    Iter,
    IterMut,
    Keys,
    LastEntry,
    LastKey,
    LastKeyValue,
    Len,
    New,
    PopFirst,
    PopLast,
    Range { bitstream: Vec<u8> },
    RangeMut { bitstream: Vec<u8> },
    Remove { key: K },
    RemoveEntry { key: K },
    Retain { rand_key: K },
    SplitOff { key: K },
    TryInsertStd { key: K, val: V },
    Values,
    ValuesMut,
    // Trait Equivalence -----------------------------------------------------------------------------------------------
    Clone,
    Debug,
    Extend { other: Vec<(K, V)> },
    // FromIterator already tested in several of the below
    Ord { other: Vec<(K, V)> },
}

// Harness Helpers -----------------------------------------------------------------------------------------------------

fn checked_get_len<K: Ord , V, const N: usize>(
    sg_map: &SgMap<K, V, N>,
    bt_map: &BTreeMap<K, V>,
) -> usize {
    let len = sg_map.len();
    assert_eq!(len, bt_map.len());

    len
}

// TODO: is this an ideal way to generate a valid range?
fn gen_valid_range<K: Ord  + Debug + Int, V, const N: usize>(
    sg_map: &SgMap<K, V, N>,
    bt_map: &BTreeMap<K, V>,
    bitstream: &[u8],
) -> Option<Range<K>> {
    let mut opt_range = None;

    // Get valid range min
    if let (Some(sg_min), Some(bt_min)) =
        (sg_map.first_key(), bt_map.first_key_value().map(|(k, _)| k))
    {
        assert_eq!(sg_min, bt_min);

        // Get valid range max
        if let (Some(sg_max), Some(bt_max)) =
            (sg_map.last_key(), bt_map.last_key_value().map(|(k, _)| k))
        {
            assert_eq!(sg_max, bt_max);

            // Generate valid range
            let mut u = Unstructured::new(&bitstream);
            if let (Ok(r1), Ok(r2)) = (
                u.int_in_range(*sg_min..=*sg_max),
                u.int_in_range(*sg_min..=*sg_max),
            ) {
                match r1.cmp(&r2) {
                    Ordering::Less => opt_range = Some(Range { start: r1, end: r2 }),
                    Ordering::Greater => opt_range = Some(Range { start: r2, end: r1 }),
                    Ordering::Equal => opt_range = None,
                }
            }
        }
    }

    opt_range
}

fn assert_len_unchanged<K: Ord , V, const N: usize>(
    sg_map: &SgMap<K, V, N>,
    bt_map: &BTreeMap<K, V>,
    old_len: usize,
) {
    assert_eq!(checked_get_len(sg_map, bt_map), old_len);
}

fn assert_eq_entry<K: Ord  + Debug, V:  Debug, const N: usize>(
    sg_entry: &SgEntry<K, V, N>,
    bt_entry: &BtEntry<K, V>,
) {
    // Check top-level key equivalence
    assert_eq!(sg_entry.key(), bt_entry.key());

    // Check variant equivalence && variant key equivalence
    match bt_entry {
        BtEntry::Vacant(btv) => match sg_entry {
            SgEntry::Occupied(_) => {
                panic!("Entry mismatch: BtEntry::Vacant vs. SgEntry::Occupied");
            }
            SgEntry::Vacant(sgv) => {
                assert_eq!(btv.key(), sgv.key());
            }
        },
        BtEntry::Occupied(bto) => match sg_entry {
            SgEntry::Vacant(_) => {
                panic!("Entry mismatch: BtEntry::Occupied vs. SgEntry::Vacant");
            }
            SgEntry::Occupied(sgo) => {
                assert_eq!(bto.key(), sgo.key());
            }
        },
    }
}

// Harness -------------------------------------------------------------------------------------------------------------

// Differential fuzzing harness
fuzz_target!(|methods: Vec<MapMethod<usize, usize>>| {
    let mut sg_map = SgMap::<_, _, CAPACITY>::new(); // Data structure under test
    let mut bt_map = BTreeMap::new(); // Reference data structure

    for m in methods {
        match m {
            // API Equivalence -----------------------------------------------------------------------------------------
            MapMethod::Append { other } => {
                if other.len() > sg_map.capacity() {
                    continue;
                }

                let mut sg_other = SgMap::from_iter(other.clone());
                let mut bt_other = BTreeMap::from_iter(other);
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(sg_other.len(), bt_other.len());
                if (len_old + sg_other.len()) <= sg_map.capacity() {
                    sg_map.append(&mut sg_other);
                    bt_map.append(&mut bt_other);

                    assert!(sg_other.is_empty());
                    assert!(bt_other.is_empty());

                    assert!(checked_get_len(&sg_map, &bt_map) >= len_old);
                }
            }
            MapMethod::Clear => {
                sg_map.clear();
                bt_map.clear();

                assert!(sg_map.is_empty());
                assert!(bt_map.is_empty());

                assert_eq!(sg_map.len(), 0);
                assert_eq!(bt_map.len(), 0);
            }
            MapMethod::ContainsKey { key } => {
                assert_eq!(sg_map.contains_key(&key), bt_map.contains_key(&key));
            }
            MapMethod::Entry { key, entry } => {
                let sg_entry = sg_map.entry(key);
                let bt_entry = bt_map.entry(key);

                assert_eq_entry(&sg_entry, &bt_entry);

                match entry {
                    MapEntry::AndModify { rand_val } => {
                        // Check only keys here, if values diverge, MapOccupiedEntry should catch
                        assert_eq!(
                            sg_entry
                                .and_modify(|e| { *e = e.wrapping_add(rand_val) })
                                .key(),
                            bt_entry
                                .and_modify(|e| { *e = e.wrapping_add(rand_val) })
                                .key(),
                        );
                    }
                    MapEntry::Key => {
                        assert_eq!(sg_entry.key(), bt_entry.key());
                    }
                    MapEntry::OrDefault => {
                        assert_eq!(sg_entry.or_default(), bt_entry.or_default());
                    }
                    MapEntry::OrInsert { default } => {
                        assert_eq!(sg_entry.or_insert(default), bt_entry.or_insert(default));
                    }
                    MapEntry::OrInsertWith { rand_val } => {
                        assert_eq!(
                            sg_entry.or_insert_with(|| rand_val),
                            bt_entry.or_insert_with(|| rand_val)
                        );
                    }
                    MapEntry::OrInsertWithKey { rand_key } => {
                        assert_eq!(
                            sg_entry.or_insert_with_key(|k| k.overflowing_add(rand_key).0),
                            bt_entry.or_insert_with_key(|k| k.overflowing_add(rand_key).0)
                        );
                    }
                    MapEntry::Occupied { inner } => {
                        // Variant equivalence already checked by `assert_eq_entry`
                        if let (SgEntry::Occupied(mut sgo), BtEntry::Occupied(mut bto)) =
                            (sg_entry, bt_entry)
                        {
                            match inner {
                                MapOccupiedEntry::Get => {
                                    assert_eq!(sgo.get(), bto.get());
                                }
                                MapOccupiedEntry::GetMut => {
                                    assert_eq!(sgo.get_mut(), bto.get_mut());
                                }
                                MapOccupiedEntry::Insert { val } => {
                                    assert_eq!(sgo.insert(val), bto.insert(val));
                                }
                                MapOccupiedEntry::IntoMut => {
                                    assert_eq!(sgo.into_mut(), bto.into_mut());
                                }
                                MapOccupiedEntry::Key => {
                                    assert_eq!(sgo.key(), bto.key());
                                }
                                MapOccupiedEntry::Remove => {
                                    assert_eq!(sgo.remove(), bto.remove());
                                }
                                MapOccupiedEntry::RemoveEntry => {
                                    assert_eq!(sgo.remove_entry(), bto.remove_entry());
                                }
                            }
                        }
                    }
                    MapEntry::Vacant { inner } => {
                        // Variant equivalence already checked by `assert_eq_entry`
                        if let (SgEntry::Vacant(sgv), BtEntry::Vacant(btv)) = (sg_entry, bt_entry) {
                            match inner {
                                MapVacantEntry::Insert { val } => {
                                    assert_eq!(sgv.insert(val), btv.insert(val));
                                }
                                MapVacantEntry::IntoKey => {
                                    assert_eq!(sgv.into_key(), btv.into_key());
                                }
                                MapVacantEntry::Key => {
                                    assert_eq!(sgv.key(), btv.key());
                                }
                            }
                        }
                    }
                }
            }
            MapMethod::FirstEntry => match (sg_map.first_entry(), bt_map.first_entry()) {
                (Some(sgo), Some(bto)) => assert_eq!(sgo.key(), bto.key()),
                (None, None) => continue,
                _ => panic!("First entry Some-None mismatch!"),
            },
            MapMethod::FirstKey => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                match bt_map.first_entry() {
                    Some(occupied_entry) => {
                        assert_eq!(sg_map.first_key(), Some(occupied_entry.key()),);
                    }
                    None => {
                        assert_eq!(sg_map.first_key(), None);
                    }
                };

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            }
            MapMethod::FirstKeyValue => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(sg_map.first_key_value(), bt_map.first_key_value());

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            }
            MapMethod::Get { key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(sg_map.get(&key), bt_map.get(&key));

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            }
            MapMethod::GetKeyValue { key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(sg_map.get_key_value(&key), bt_map.get_key_value(&key));

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            }
            MapMethod::GetMut { key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(sg_map.get_mut(&key), bt_map.get_mut(&key));

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            }
            MapMethod::Insert { key, val } => {
                let len_old = checked_get_len(&sg_map, &bt_map);
                if len_old < sg_map.capacity() {
                    assert_eq!(sg_map.insert(key, val), bt_map.insert(key, val));

                    assert!(checked_get_len(&sg_map, &bt_map) >= len_old);
                }
            }
            MapMethod::IsEmpty => {
                assert_eq!(sg_map.is_empty(), bt_map.is_empty(),);
            }
            MapMethod::Iter => {
                assert!(sg_map.iter().eq(bt_map.iter()));
            }
            MapMethod::IterMut => {
                assert!(sg_map.iter_mut().eq(bt_map.iter_mut()));
            }
            MapMethod::Keys => {
                assert!(sg_map.keys().eq(bt_map.keys()));
            }
            MapMethod::LastEntry => match (sg_map.last_entry(), bt_map.last_entry()) {
                (Some(sgo), Some(bto)) => assert_eq!(sgo.key(), bto.key()),
                (None, None) => continue,
                _ => panic!("Last entry Some-None mismatch!"),
            },
            MapMethod::LastKey => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                match bt_map.last_entry() {
                    Some(occupied_entry) => {
                        assert_eq!(sg_map.last_key(), Some(occupied_entry.key()),);
                    }
                    None => {
                        assert_eq!(sg_map.last_key(), None);
                    }
                };

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            }
            MapMethod::LastKeyValue => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(sg_map.last_key_value(), bt_map.last_key_value());

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            }
            MapMethod::Len => {
                assert_eq!(sg_map.len(), bt_map.len());
            }
            MapMethod::New => {
                sg_map = SgMap::new();
                bt_map = BTreeMap::new();
            }
            MapMethod::PopFirst => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(sg_map.pop_first(), bt_map.pop_first());

                assert!(checked_get_len(&sg_map, &bt_map) <= len_old);
            }
            MapMethod::PopLast => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(sg_map.pop_last(), bt_map.pop_last());

                assert!(checked_get_len(&sg_map, &bt_map) <= len_old);
            }
            MapMethod::Range { bitstream } => {
                if let Some(range) = gen_valid_range(&sg_map, &bt_map, &bitstream) {
                    let sg_range_iter = sg_map.range((Included(range.start), Included(range.end)));
                    let bt_range_iter = bt_map.range((Included(range.start), Included(range.end)));
                    assert!(sg_range_iter.eq(bt_range_iter));
                }
            }
            MapMethod::RangeMut { bitstream } => {
                if let Some(range) = gen_valid_range(&sg_map, &bt_map, &bitstream) {
                    let sg_range_iter =
                        sg_map.range_mut((Included(range.start), Included(range.end)));
                    let bt_range_iter =
                        bt_map.range_mut((Included(range.start), Included(range.end)));
                    assert!(sg_range_iter.eq(bt_range_iter));
                }
            }
            MapMethod::Remove { key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(sg_map.remove(&key), bt_map.remove(&key));

                assert!(checked_get_len(&sg_map, &bt_map) <= len_old);
            }
            MapMethod::RemoveEntry { key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(sg_map.remove_entry(&key), bt_map.remove_entry(&key));

                assert!(checked_get_len(&sg_map, &bt_map) <= len_old);
            }
            MapMethod::Retain { rand_key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                sg_map.retain(|&k, _| (k.wrapping_add(rand_key) % 2 == 0));
                bt_map.retain(|&k, _| (k.wrapping_add(rand_key) % 2 == 0));

                assert!(sg_map.iter().eq(bt_map.iter()));
                assert!(checked_get_len(&sg_map, &bt_map) <= len_old);
            }
            MapMethod::SplitOff { key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert!(sg_map
                    .split_off(&key)
                    .iter()
                    .eq(bt_map.split_off(&key).iter()));

                assert!(sg_map.iter().eq(bt_map.iter()));
                assert!(checked_get_len(&sg_map, &bt_map) <= len_old);
            }
            MapMethod::TryInsertStd { key, val } => {
                assert_eq!(
                    sg_map
                        .try_insert_std(key, val)
                        .map_err(|oe| (*oe.entry.key(), oe.value)),
                    bt_map
                        .try_insert(key, val)
                        .map_err(|oe| (*oe.entry.key(), oe.value))
                );
            }
            // Trait Equivalence ---------------------------------------------------------------------------------------
            MapMethod::Clone => {
                assert!(sg_map.clone().iter().eq(bt_map.clone().iter()));
            }
            MapMethod::Debug => {
                assert_eq!(format!("{:?}", sg_map), format!("{:?}", bt_map),);
            }
            MapMethod::Extend { other } => {
                let len_old = checked_get_len(&sg_map, &bt_map);
                if (len_old + other.len()) <= sg_map.capacity() {
                    sg_map.extend(other.clone().into_iter());
                    bt_map.extend(other.into_iter());

                    assert!(sg_map.iter().eq(bt_map.iter()));
                    assert!(checked_get_len(&sg_map, &bt_map) >= len_old);
                }
            }
            MapMethod::Ord { other } => {
                if other.len() > sg_map.capacity() {
                    continue;
                }

                let sg_map_new = SgMap::from_iter(other.clone().into_iter());
                let bt_map_new = BTreeMap::from_iter(other.into_iter());

                assert_eq!(sg_map.cmp(&sg_map_new), bt_map.cmp(&bt_map_new),);
            }
            MapMethod::Values => {
                assert!(sg_map.values().eq(bt_map.values()));
            }
            MapMethod::ValuesMut => {
                assert!(sg_map.values_mut().eq(bt_map.values_mut()));
            }
        }
    }
});
