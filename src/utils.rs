use rand::Rng;

pub fn game_id_generator() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const STRING_LENGTH: usize = 5;

    let mut rng = rand::thread_rng();

    // Create a String with capacity for better performance
    let random_string: String = (0..STRING_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    random_string
}
