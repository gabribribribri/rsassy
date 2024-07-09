// Interesting things :
// The message can't be bigger than the second number of the key

use rand::seq::SliceRandom;

const RANDOM_PRIMES: [u128; 600] = [
    6131, 6133, 6143, 6151, 6163, 6173, 6197, 6199, 6203, 6211, 6217, 6221, 6229, 6247, 6257, 6263,
    6269, 6271, 6277, 6287, 6299, 6301, 6311, 6317, 6323, 6329, 6337, 6343, 6353, 6359, 6361, 6367,
    6373, 6379, 6389, 6397, 6421, 6427, 6449, 6451, 6469, 6473, 6481, 6491, 6521, 6529, 6547, 6551,
    6553, 6563, 6569, 6571, 6577, 6581, 6599, 6607, 6619, 6637, 6653, 6659, 6661, 6673, 6679, 6689,
    6691, 6701, 6703, 6709, 6719, 6733, 6737, 6761, 6763, 6779, 6781, 6791, 6793, 6803, 6823, 6827,
    6829, 6833, 6841, 6857, 6863, 6869, 6871, 6883, 6899, 6907, 6911, 6917, 6947, 6949, 6959, 6961,
    6967, 6971, 6977, 6983, 6991, 6997, 7001, 7013, 7019, 7027, 7039, 7043, 7057, 7069, 7079, 7103,
    7109, 7121, 7127, 7129, 7151, 7159, 7177, 7187, 7193, 7207, 7211, 7213, 7219, 7229, 7237, 7243,
    7247, 7253, 7283, 7297, 7307, 7309, 7321, 7331, 7333, 7349, 7351, 7369, 7393, 7411, 7417, 7433,
    7451, 7457, 7459, 7477, 7481, 7487, 7489, 7499, 7507, 7517, 7523, 7529, 7537, 7541, 7547, 7549,
    7559, 7561, 7573, 7577, 7583, 7589, 7591, 7603, 7607, 7621, 7639, 7643, 7649, 7669, 7673, 7681,
    7687, 7691, 7699, 7703, 7717, 7723, 7727, 7741, 7753, 7757, 7759, 7789, 7793, 7817, 7823, 7829,
    7841, 7853, 7867, 7873, 7877, 7879, 7883, 7901, 7907, 7919, 7927, 7933, 7937, 7949, 7951, 7963,
    7993, 8009, 8011, 8017, 8039, 8053, 8059, 8069, 8081, 8087, 8089, 8093, 8101, 8111, 8117, 8123,
    8147, 8161, 8167, 8171, 8179, 8191, 8209, 8219, 8221, 8231, 8233, 8237, 8243, 8263, 8269, 8273,
    8287, 8291, 8293, 8297, 8311, 8317, 8329, 8353, 8363, 8369, 8377, 8387, 8389, 8419, 8423, 8429,
    8431, 8443, 8447, 8461, 8467, 8501, 8513, 8521, 8527, 8537, 8539, 8543, 8563, 8573, 8581, 8597,
    8599, 8609, 8623, 8627, 8629, 8641, 8647, 8663, 8669, 8677, 8681, 8689, 8693, 8699, 8707, 8713,
    8719, 8731, 8737, 8741, 8747, 8753, 8761, 8779, 8783, 8803, 8807, 8819, 8821, 8831, 8837, 8839,
    8849, 8861, 8863, 8867, 8887, 8893, 8923, 8929, 8933, 8941, 8951, 8963, 8969, 8971, 8999, 9001,
    9007, 9011, 9013, 9029, 9041, 9043, 9049, 9059, 9067, 9091, 9103, 9109, 9127, 9133, 9137, 9151,
    9157, 9161, 9173, 9181, 9187, 9199, 9203, 9209, 9221, 9227, 9239, 9241, 9257, 9277, 9281, 9283,
    9293, 9311, 9319, 9323, 9337, 9341, 9343, 9349, 9371, 9377, 9391, 9397, 9403, 9413, 9419, 9421,
    9431, 9433, 9437, 9439, 9461, 9463, 9467, 9473, 9479, 9491, 9497, 9511, 9521, 9533, 9539, 9547,
    9551, 9587, 9601, 9613, 9619, 9623, 9629, 9631, 9643, 9649, 9661, 9677, 9679, 9689, 9697, 9719,
    9721, 9733, 9739, 9743, 9749, 9767, 9769, 9781, 9787, 9791, 9803, 9811, 9817, 9829, 9833, 9839,
    9851, 9857, 9859, 9871, 9883, 9887, 9901, 9907, 9923, 9929, 9931, 9941, 9949, 9967, 9973,
    10007, 10009, 10037, 10039, 10061, 10067, 10069, 10079, 10091, 10093, 10099, 10103, 10111,
    10133, 10139, 10141, 10151, 10159, 10163, 10169, 10177, 10181, 10193, 10211, 10223, 10243,
    10247, 10253, 10259, 10267, 10271, 10273, 10289, 10301, 10303, 10313, 10321, 10331, 10333,
    10337, 10343, 10357, 10369, 10391, 10399, 10427, 10429, 10433, 10453, 10457, 10459, 10463,
    10477, 10487, 10499, 10501, 10513, 10529, 10531, 10559, 10567, 10589, 10597, 10601, 10607,
    10613, 10627, 10631, 10639, 10651, 10657, 10663, 10667, 10687, 10691, 10709, 10711, 10723,
    10729, 10733, 10739, 10753, 10771, 10781, 10789, 10799, 10831, 10837, 10847, 10853, 10859,
    10861, 10867, 10883, 10889, 10891, 10903, 10909, 10937, 10939, 10949, 10957, 10973, 10979,
    10987, 10993, 11003, 11027, 11047, 11057, 11059, 11069, 11071, 11083, 11087, 11093, 11113,
    11117, 11119, 11131, 11149, 11159, 11161, 11171, 11173, 11177, 11197, 11213, 11239, 11243,
    11251, 11257, 11261, 11273, 11279, 11287, 11299, 11311, 11317, 11321, 11329, 11351, 11353,
    11369, 11383, 11393, 11399, 11411, 11423, 11437, 11443, 11447, 11467, 11471, 11483, 11489,
    11491, 11497, 11503, 11519, 11527, 11549, 11551, 11579, 11587, 11593, 11597, 11617, 11621,
];
use std::collections::HashSet;

pub type PublicKey = (u32, u128);
pub type SecreteKey = (u32, u128);

pub fn encrypt(message: u128, pkey: PublicKey) -> u128 {
    println!("Encrypting message: {}", message);
    message.pow(pkey.0) % pkey.1
}

pub fn decrypt(cypher: u128, skey: SecreteKey) -> u128 {
    println!("Decrypting cypher: {}", cypher);
    cypher.pow(skey.0) % skey.1
}

pub fn generate_keys() -> (PublicKey, SecreteKey) {
    // Choose two prime numbers
    // let mut rng = rand::thread_rng();
    // let pr1: u128 = RANDOM_PRIMES.choose(&mut rng).unwrap().clone();
    // let pr2: u128 = RANDOM_PRIMES.choose(&mut rng).unwrap().clone();
    // println!("Primes : {}, {}", pr1, pr2);
    // let pr1 = 307;
    // let pr2 = 719;
    let pr1 = 11;
    let pr2 = 7;

    // Product of p and q
    let product = pr1 * pr2;
    println!("Product: {}", product);

    // numbers of coprimes with n
    let number_product_coprimes = coprimes_count(pr1, pr2);
    println!("Number of Coprime Numbers: {}", number_product_coprimes);

    // e must be coprime with n and cop_n
    let encrypt_num = coprimes_2num(product, number_product_coprimes)
        .nth(1)
        .unwrap();
    println!("Number for encryption: {}", encrypt_num);

    // d must be d*e%cop_n=1
    let decrypt_num = find_decrypt_num(encrypt_num, number_product_coprimes, product);
    println!("Number for decryption: {}", decrypt_num);

    return ((encrypt_num as u32, product), (decrypt_num as u32, product));
}

fn coprimes_count(p: u128, q: u128) -> u128 {
    (p - 1) * (q - 1)
}

fn find_decrypt_num(e: u128, cop_n: u128, n: u128) -> u128 {
    for (i, d) in (1..=n).rev().map(|x| (x, (x * e) % cop_n)) {
        if d == 1 {
            return i;
        }
    }
    panic!("No Decrypt Number was found");
}

// return the number of numbers lesser than n that doesn't share a common factor with n
pub fn coprimes_2num(n: u128, m: u128) -> impl Iterator<Item = u128> {
    let mut nmfactors = factors(n, m);
    nmfactors.remove(&1);
    (1..=std::cmp::min(n, m)).filter(move |a| nmfactors.iter().all(|b| a % b != 0))
}

// factors of n
pub fn factors(n: u128, m: u128) -> HashSet<u128> {
    println!("Finding all factors of {} and {}...", n, m);
    (1..=(std::cmp::max(n, m)) / 2 + 1)
        .filter(|x| n % x == 0 || m % x == 0)
        .collect::<HashSet<u128>>()
}
