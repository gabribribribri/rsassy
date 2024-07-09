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
    fn coprime_2num_test() {
        assert_eq!(rsassy::coprimes_2num(14, 6), [1, 5].into_iter().collect())
    }

    #[test]
    fn factors_test() {
        assert_eq!(rsassy::factors(12, 15), [1, 3].into_iter().collect())
    }

    #[test]
    fn factors_piff() {
        println!("{:?}", rsassy::factors(14, 6));
    }
}
