use rand::thread_rng;
use rand::Rng;

pub fn random_str(length: i32) -> String {
    let mut rng = thread_rng();
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    let random_string: String = (0..length)
        .map(|_| {
            characters
                .chars()
                .nth(rng.gen_range(0..characters.len()))
                .unwrap()
        })
        .collect();

    random_string
}

pub fn random_frame() -> String {
    let mut rng = thread_rng();
    let random_f = rng.gen_range(40..300);
    random_f.to_string()
}
