use std::collections::HashSet;
use rand::Rng;
use bloomfilter::Bloom;

const NUMBER_OF_ITEMS:usize = 100000;
const FLASE_POSITIVE_RATE: f64 = 0.001;

fn test_bloom_filter() {
    let mut bloom = Bloom::new_for_fp_rate(NUMBER_OF_ITEMS, FLASE_POSITIVE_RATE);
    bloom.set(&10);   // insert 10 in the bloom filter
    println!("Check 10: {:?}", bloom.check(&10)); // return true
    println!("Check 20: {:?}", bloom.check(&20)); // return false
}

fn test_private_set_intersection() {
    let mut rng = rand::thread_rng();

    // Generate two sets of elements
    let alice_set: HashSet<u32> = (0..20).map(|_| rng.gen::<u32>()).collect();
    let bob_set: HashSet<u32> = (15..100).map(|_| rng.gen::<u32>()).collect();

    // Create Bloom filters for each set
    // let mut alice_bloom = Bloom::new_for_fp_rate(NUMBER_OF_ITEMS, FLASE_POSITIVE_RATE);
    // let mut bob_bloom = Bloom::new_for_fp_rate(NUMBER_OF_ITEMS, FLASE_POSITIVE_RATE);
    let mut alice_bloom = Bloom::new(100, 5); // bitmap size: 100 and number of hash functions: 5
    let mut bob_bloom = Bloom::new(128, 7); // bitmap size: 128 and number of hash functions: 7
    
    // Insert elements into Bloom filters
    for element in alice_set.iter() {
        alice_bloom.set(element);
    }
    for element in bob_set.iter() {
        bob_bloom.set(element);
    }

    // Compute the intersection using Bloom filters (remember false positives)
    let mut intersection = HashSet::new();
    for element in alice_set.iter() {
        if bob_bloom.check(element) { // Check if element also in filter_b
            intersection.insert(element);
        }
    }

    println!("Intersection using Bloom filters (may contain false positives): {:?}", intersection);
}

fn main() {
    test_bloom_filter();
    test_private_set_intersection();
}
