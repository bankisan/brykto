// TODO: In progress.
fn aes_block(block: &[u8], key: &[u8], output: &mut [u8]) -> Vec<u8> {
    // TODO: Find a better assert.
    let key_length = key.len();
    assert!(key_length == 16 || key_length == 24 || key_length == 32);
    assert_eq!(block.len(), 16);

    // Initialize state.
    let mut state: [u8; 16] = [0; 16];
    for (i, state_value) in state.iter_mut().enumerate() {
        *state_value = block[i];
    }

    vec![]
    // First. Pad AES.
}
