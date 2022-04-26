/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    let empty_vector: Vec<u8> = Vec::new();
    empty_vector
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut buffer_vector: Vec<u8> = Vec::new();
    for _ in 0..count {
        buffer_vector.push(0);
    }
    buffer_vector
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut buffer_vector: Vec<u8> = vec![1,1];
    for _ in 0..3 {
        buffer_vector.push(buffer_vector[buffer_vector.len() - 1] + buffer_vector[buffer_vector.len() - 2]);
    }
    buffer_vector
}
