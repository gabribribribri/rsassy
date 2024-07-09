#[cfg(test)]
mod tests {
    use rsassy::generate_keys;

    #[test]
    fn video_example() {
        let message = 7;
        let (pkey, skey) = generate_keys();
        let encrypted_message = rsassy::encrypt(message, pkey);
        let decrypted_message = rsassy::decrypt(encrypted_message, skey);
        assert_eq!(message, decrypted_message);
    }

    #[test]
    fn coprime_2num_test() {
        assert_eq!(
            vec![1, 5],
            rsassy::coprimes_2num(14, 6).collect::<Vec<u128>>()
        )
    }

    #[test]
    fn factors_test() {
        assert_eq!(
            rsassy::factors(12, 15),
            [1, 2, 3, 4, 5, 6].into_iter().collect()
        )
    }

    #[test]
    fn factors_piff() {
        println!("{:?}", rsassy::factors(14, 6));
    }

    // #[test]
    // fn thing() {
    //     println!("{:?}", (0..10).rev().map(|x| x).collect::<Vec<i32>>())
    // }
}
