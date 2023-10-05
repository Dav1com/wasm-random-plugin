use rand::{Rng, RngCore};
use wasm_minimal_protocol::*;
use core::cell::RefCell;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

initiate_protocol!();

struct RandomDescriptor {
    seed_cache: Vec<Xoshiro256PlusPlus>,
    generator: Xoshiro256PlusPlus
}

thread_local! {
    static DESCRIPTORS: RefCell<Vec<RandomDescriptor>> = RefCell::new(Vec::new())
}

#[wasm_func]
pub fn init_from_u64(seed : &[u8]) -> Vec<u8> {
    let mut index : usize = 0;
    let u64_seed = u64::from_ne_bytes(seed.try_into().expect("Seed must be a 8 bytes in length."));
    DESCRIPTORS.with(|descriptors: &RefCell<Vec<RandomDescriptor>>| {
        let vec = &mut descriptors.borrow_mut();
        index = vec.len();
        vec.push(RandomDescriptor { seed_cache: Vec::new(), generator: Xoshiro256PlusPlus::seed_from_u64(u64_seed) })
    });

    return (index as u64).to_ne_bytes().to_vec();
}

#[wasm_func]
pub fn random_u64(index : &[u8], pos : &[u8], max : &[u8]) -> Vec<u8> {
    let mut result : u64 = 0;

    let u64_index = u64::from_ne_bytes(index.try_into().expect("index must be a 8 bytes in length."));
    assert!(u64_index <= (usize::MAX as u64), "index is too large for usize.");
    let usize_index = u64_index as usize;

    let u64_pos = u64::from_ne_bytes(pos.try_into().expect("pos must be a 8 bytes in length."));
    assert!(u64_pos <= (usize::MAX as u64), "pos is too large for usize.");
    let usize_pos = u64_pos as usize;

    let u64_max = u64::from_ne_bytes(max.try_into().expect("max must be a 8 bytes in length."));

    DESCRIPTORS.with(|descriptors: &RefCell<Vec<RandomDescriptor>>| {
        let vec = &mut descriptors.borrow_mut();
        if vec.len() as u64 <= u64_index {
            panic!("")
        }
        
        let descriptor = &mut vec[usize_index];
        let generator = &mut descriptor.generator;
        let cache = &mut descriptor.seed_cache;

        for _ in cache.len()..=usize_pos+1 {
            cache.push(generator.clone());
            generator.next_u64();
        }

        result = descriptor.seed_cache[usize_pos].clone().gen_range(0..=u64_max);
    });

    return result.to_ne_bytes().to_vec();
}

#[wasm_func]
pub fn random_f64(index: &[u8], pos: &[u8]) -> Vec<u8> {
    let mut result : f64 = 0.0;

    let u64_index = u64::from_ne_bytes(index.try_into().expect("index must be a 8 bytes in length."));
    assert!(u64_index <= (usize::MAX as u64), "index is too large for usize.");
    let usize_index = u64_index as usize;

    let u64_pos = u64::from_ne_bytes(pos.try_into().expect("pos must be a 8 bytes in length."));
    assert!(u64_pos <= (usize::MAX as u64), "pos is too large for usize.");
    let usize_pos = u64_pos as usize;

    DESCRIPTORS.with(|descriptors: &RefCell<Vec<RandomDescriptor>>| {
        let vec = &mut descriptors.borrow_mut();
        if vec.len() as u64 <= u64_index {
            panic!("")
        }
        
        let descriptor = &mut vec[usize_index];
        let generator = &mut descriptor.generator;
        let cache = &mut descriptor.seed_cache;

        for _ in cache.len()..=usize_pos+1 {
            cache.push(generator.clone());
            generator.next_u64();
        }

        result = descriptor.seed_cache[usize_pos].clone().gen();
    });

    return result.to_ne_bytes().to_vec();
}
