#[cfg(test)]
mod tests {
    use std::{collections::HashSet, hash::Hash};

    #[test]
    fn video_example() {
        let message = 13;
        let encrypted_message = rsassy::encrypt(message, (5, 14));
        let decrypted_message = rsassy::decrypt(encrypted_message, (11, 14));
        assert_eq!(message, decrypted_message);
    }

    #[test]
    fn factors_test() {
        for i in 2..=100 {
            // println!("{}: {:?}", i, rsassy::factors(i));
        }
    }

    #[test]
    fn coprime_2num_test() {
        assert_eq!(
            rsassy::coprimes_2num(14, 6),
            [1, 3, 5, 9, 11, 13].into_iter().collect()
        )
    }
}
