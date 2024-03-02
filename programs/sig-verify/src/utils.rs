use anchor_lang::solana_program::{ed25519_program, instruction::Instruction, pubkey::Pubkey};
use bytemuck::{bytes_of, Pod, Zeroable};

const PUBKEY_SERIALIZED_SIZE: usize = 32;
const SIGNATURE_SERIALIZED_SIZE: usize = 64;
const SIGNATURE_OFFSETS_SERIALIZED_SIZE: usize = 14;
// bytemuck requires structures to be aligned
const SIGNATURE_OFFSETS_START: usize = 2;
const DATA_START: usize = SIGNATURE_OFFSETS_SERIALIZED_SIZE + SIGNATURE_OFFSETS_START;

#[derive(Default, Debug, Copy, Clone, Zeroable, Pod, Eq, PartialEq)]
#[repr(C)]
pub struct Ed25519SignatureOffsets {
    signature_offset: u16,             // offset to ed25519 signature of 64 bytes
    signature_instruction_index: u16,  // instruction index to find signature
    public_key_offset: u16,            // offset to public key of 32 bytes
    public_key_instruction_index: u16, // instruction index to find public key
    message_data_offset: u16,          // offset to start of message data
    message_data_size: u16,            // size of message data
    message_instruction_index: u16,    // index of instruction data to get message data
}

pub fn construct_ed25519_instruction(
    // pubkey: [u8; 32],
    // signature: [u8; 64],
    // msg: Vec<u8>,
    pubkey: Pubkey,
    signature: [u8; 64],
    msg: String,
) -> Instruction {
    let msg_array = msg.as_bytes();

    let mut instruction_data: Vec<u8> = Vec::with_capacity(
        DATA_START
            .saturating_add(SIGNATURE_SERIALIZED_SIZE)
            .saturating_add(PUBKEY_SERIALIZED_SIZE)
            .saturating_add(msg.len()),
    );

    let num_signatures: u8 = 1;
    let public_key_offset = DATA_START;
    let signature_offset = public_key_offset.saturating_add(PUBKEY_SERIALIZED_SIZE);
    let message_data_offset = signature_offset.saturating_add(SIGNATURE_SERIALIZED_SIZE);

    // add padding byte so that offset structure is aligned
    instruction_data.extend_from_slice(bytes_of(&[num_signatures, 0]));

    // let signature_offset = signature_offset as u16;
    // let signature_instruction_index = u16::MAX;
    // let public_key_offset = public_key_offset as u16;
    // let public_key_instruction_index = u16::MAX;
    // let message_data_offset = message_data_offset as u16;
    // let message_data_size = msg.len() as u16;
    // let message_instruction_index = u16::MAX;
    // append_array_to_vec(&mut instruction_data, &signature_offset.to_le_bytes());
    // append_array_to_vec(
    //     &mut instruction_data,
    //     &signature_instruction_index.to_le_bytes(),
    // );
    // append_array_to_vec(&mut instruction_data, &public_key_offset.to_le_bytes());
    // append_array_to_vec(
    //     &mut instruction_data,
    //     &public_key_instruction_index.to_le_bytes(),
    // );
    // append_array_to_vec(&mut instruction_data, &message_data_offset.to_le_bytes());
    // append_array_to_vec(&mut instruction_data, &message_data_size.to_le_bytes());
    // append_array_to_vec(
    //     &mut instruction_data,
    //     &message_instruction_index.to_le_bytes(),
    // );

    let offsets = Ed25519SignatureOffsets {
        signature_offset: signature_offset as u16,
        signature_instruction_index: u16::MAX,
        public_key_offset: public_key_offset as u16,
        public_key_instruction_index: u16::MAX,
        message_data_offset: message_data_offset as u16,
        message_data_size: msg.len() as u16,
        message_instruction_index: u16::MAX,
    };

    instruction_data.extend_from_slice(bytes_of(&offsets));

    instruction_data.extend_from_slice(&pubkey.to_bytes());

    instruction_data.extend_from_slice(&signature);

    instruction_data.extend_from_slice(msg_array);

    Instruction {
        program_id: ed25519_program::ID,
        accounts: vec![],
        data: instruction_data,
    }
}

// fn append_array_to_vec(vec: &mut Vec<u8>, array: &[u8]) {
//     for &element in array.iter() {
//         vec.push(element);
//     }
// }
