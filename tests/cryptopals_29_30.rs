use brykto::hasher::*;

#[test]
fn challenge_29() {
    let secret_key = "spooky";
    let message = "comment1=cooking%20MCs;userdata=foo;comment2=%20like%20a%20pound%20of%20bacon";

    // MAC using a key prefix: H(K, m).
    let mac = sha1([secret_key, message].concat().as_str());

    // Output of the MAC will be used as the IV to implement length
    // extension.
    let mut iv: (u32, u32, u32, u32, u32) = (0, 0, 0, 0, 0);
    let to_32bit: Vec<_> = mac
        .chunks(4)
        .map(|chunk| {
            let to_32_bit: [u8; 4] = [chunk[0], chunk[1], chunk[2], chunk[3]];
            u32::from_be_bytes(to_32_bit)
        })
        .collect();
    iv.0 = to_32bit[0];
    iv.1 = to_32bit[1];
    iv.2 = to_32bit[2];
    iv.3 = to_32bit[3];
    iv.4 = to_32bit[4];

    let attacker_message = ";admin=true";
    let mut attacker_mac: [u8; 20];
    let mut expected_keyed_mac: [u8; 20];
    let mut found_match = false;
    for i in 1..40 {
        // I know the message but I don't know the secret key.
        let glue_padding = md_padding(i + message.len(), Endian::Big);
        let total_length = i + message.len() + glue_padding.len() + attacker_message.len();
        attacker_mac = sha1::core(attacker_message, total_length, iv);

        expected_keyed_mac = sha1(
            [
                secret_key.as_bytes(),
                message.as_bytes(),
                glue_padding.as_slice(),
                attacker_message.as_bytes(),
            ]
            .concat()
            .as_slice(),
        );

        found_match = attacker_mac == expected_keyed_mac;
        if found_match {
            println!(
                "Found with secret key length {}!\nATTACKER_MAC: {:02X?}\nNEW_MAC: {:02X?}\n",
                i, attacker_mac, expected_keyed_mac
            );
            break;
        }
    }
    assert!(found_match)
}

#[test]
fn challenge_30() {
    let secret_key = "veryspooky";
    let message = "comment1=cooking%20MCs;userdata=foo;comment2=%20like%20a%20pound%20of%20bacon";

    // MAC using a key prefix: H(K, m).
    let mac = md4([secret_key, message].concat().as_str());

    // Output of the MAC will be used as the IV to implement length
    // extension.
    let mut iv: (u32, u32, u32, u32) = (0, 0, 0, 0);
    let to_32bit: Vec<_> = mac
        .chunks(4)
        .map(|chunk| {
            let to_32_bit: [u8; 4] = [chunk[0], chunk[1], chunk[2], chunk[3]];
            u32::from_le_bytes(to_32_bit)
        })
        .collect();
    iv.0 = to_32bit[0];
    iv.1 = to_32bit[1];
    iv.2 = to_32bit[2];
    iv.3 = to_32bit[3];

    let attacker_message = ";admin=true";
    let mut attacker_mac: [u8; 16];
    let mut expected_keyed_mac: [u8; 16];
    let mut found_match = false;
    for i in 1..40 {
        // I know the message but I don't know the secret key.
        let glue_padding = md_padding(i + message.len(), Endian::Little);
        let total_length = i + message.len() + glue_padding.len() + attacker_message.len();
        attacker_mac = md4::core(attacker_message, total_length, iv);

        expected_keyed_mac = md4([
            secret_key.as_bytes(),
            message.as_bytes(),
            glue_padding.as_slice(),
            attacker_message.as_bytes(),
        ]
        .concat()
        .as_slice());

        found_match = attacker_mac == expected_keyed_mac;
        if found_match {
            println!(
                "Found with secret key length {}!\nATTACKER_MAC: {:02X?}\nNEW_MAC: {:02X?}\n",
                i, attacker_mac, expected_keyed_mac
            );
            break;
        }
    }
    assert!(found_match)
}
