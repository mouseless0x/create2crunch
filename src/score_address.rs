pub fn score_address(address: &[u8]) -> i32 {
    // Convert the address bytes to a fixed array of nibbles
    let mut nibbles = [0u8; 40]; // An Ethereum address has 20 bytes, hence 40 nibbles
    for (i, &byte) in address.iter().enumerate() {
        nibbles[2 * i] = byte >> 4;      // High nibble (top 4 bits)
        nibbles[2 * i + 1] = byte & 0x0F; // Low nibble (bottom 4 bits)
    }

    // Initialize total score
    let mut total_score = 0;

    // 1. Ten (10) points for every leading 0 nibble
    let leading_zeros_count = nibbles.iter().take_while(|&&n| n == 0).count();
    total_score += (leading_zeros_count * 10) as i32;

    // 2. Forty (40) points if the first nibble '4' is followed by 3 more '4's
    // 3. Twenty (20) points if the first nibble after these 4 '4's is NOT '4'
    for window in nibbles.windows(5) {
        // Check if the first 4 nibbles are '4'
        if window[0..4] == [4, 4, 4, 4] {
            total_score += 40; // Found '4444' sequence
            if window[4] != 4 {
                total_score += 20; // Next nibble after '4444' is not '4'
            }
            break; // No need to check further once the first sequence is found
        }
    }

    // 4. Twenty (20) points if the last 4 nibbles are '4's
    let nibble_count = nibbles.len();
    if nibble_count >= 4 && nibbles[nibble_count - 4..] == [4, 4, 4, 4] {
        total_score += 20;
    }

    // 5. One (1) point for every '4' nibble
    let fours_count = nibbles.iter().filter(|&&n| n == 4).count();
    total_score += fours_count as i32;

    // total_score now holds the final calculated score
    total_score
}

