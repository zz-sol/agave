use {
    agave_feature_set::FeatureSet,
    solana_falcon_signature::{
        Falcon512SignatureOffsets, PublicKey, Signature, PUBKEY_SIZE, SIGNATURE_OFFSETS_SIZE,
        SIGNATURE_OFFSETS_START,
    },
    solana_precompile_error::PrecompileError,
};

pub fn verify(
    data: &[u8],
    instruction_datas: &[&[u8]],
    _feature_set: &FeatureSet,
) -> Result<(), PrecompileError> {
    if data.len() < SIGNATURE_OFFSETS_START {
        return Err(PrecompileError::InvalidInstructionDataSize);
    }
    let num_signatures = data[0] as usize;
    if num_signatures == 0 && data.len() > SIGNATURE_OFFSETS_START {
        return Err(PrecompileError::InvalidInstructionDataSize);
    }
    let expected_data_size = num_signatures
        .saturating_mul(SIGNATURE_OFFSETS_SIZE)
        .saturating_add(SIGNATURE_OFFSETS_START);
    // We do not check or use the byte at data[1]
    if data.len() < expected_data_size {
        return Err(PrecompileError::InvalidInstructionDataSize);
    }

    for i in 0..num_signatures {
        let start = i
            .saturating_mul(SIGNATURE_OFFSETS_SIZE)
            .saturating_add(SIGNATURE_OFFSETS_START);

        // SAFETY:
        // - data[start..] is guaranteed to be >= size of Falcon512SignatureOffsets
        // - Falcon512SignatureOffsets is a POD type, so we can safely read it as an unaligned struct
        let offsets = unsafe {
            core::ptr::read_unaligned(data.as_ptr().add(start) as *const Falcon512SignatureOffsets)
        };

        let signature = get_data_slice(
            data,
            instruction_datas,
            offsets.signature_instruction_index,
            offsets.signature_offset,
            offsets.signature_length as usize,
        )?;
        let signature = Signature::from_slice(signature)
            .map_err(|_| PrecompileError::InvalidSignature)?;

        let pubkey = get_data_slice(
            data,
            instruction_datas,
            offsets.public_key_instruction_index,
            offsets.public_key_offset,
            PUBKEY_SIZE,
        )?;
        let public_key =
            PublicKey::from_slice(pubkey).map_err(|_| PrecompileError::InvalidPublicKey)?;

        let message = get_data_slice(
            data,
            instruction_datas,
            offsets.message_instruction_index,
            offsets.message_offset,
            offsets.message_length as usize,
        )?;

        public_key
            .verify(message, &signature)
            .map_err(|_| PrecompileError::InvalidSignature)?;
    }

    Ok(())
}

fn get_data_slice<'a>(
    data: &'a [u8],
    instruction_datas: &'a [&[u8]],
    instruction_index: u16,
    offset_start: u16,
    size: usize,
) -> Result<&'a [u8], PrecompileError> {
    let instruction = if instruction_index == u16::MAX {
        data
    } else {
        let signature_index = instruction_index as usize;
        if signature_index >= instruction_datas.len() {
            return Err(PrecompileError::InvalidDataOffsets);
        }
        instruction_datas[signature_index]
    };

    let start = offset_start as usize;
    let end = start.saturating_add(size);
    if end > instruction.len() {
        return Err(PrecompileError::InvalidDataOffsets);
    }

    Ok(&instruction[start..end])
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::test_verify_with_alignment,
        bytemuck::from_bytes,
        solana_falcon_signature::{
            new_falcon512_instruction_with_signature, Falcon512SignatureOffsets, SecretKey,
            DATA_START, SIGNATURE_HEADER, SIGNATURE_OFFSETS_SIZE, SIGNATURE_OFFSETS_START,
        },
    };

    #[test]
    fn test_falcon_ok() {
        let message = b"falcon precompile";
        let secret = SecretKey::generate().expect("key generation failed");
        let signature = secret.sign(message).expect("signing failed");
        let instruction =
            new_falcon512_instruction_with_signature(message, &signature, secret.public_key());

        test_verify_with_alignment(
            verify,
            &instruction.data,
            &[],
            &FeatureSet::all_enabled(),
        )
        .unwrap();
    }

    #[test]
    fn test_invalid_instruction_data_size() {
        assert_eq!(
            verify(&[], &[], &FeatureSet::all_enabled()),
            Err(PrecompileError::InvalidInstructionDataSize)
        );
    }

    #[test]
    fn test_invalid_offsets() {
        let offsets = Falcon512SignatureOffsets {
            signature_offset: DATA_START as u16,
            signature_length: 50,
            signature_instruction_index: 0,
            public_key_offset: DATA_START as u16,
            public_key_instruction_index: 0,
            message_offset: DATA_START as u16,
            message_length: 4,
            message_instruction_index: 0,
        };
        let mut instruction_data = Vec::with_capacity(
            SIGNATURE_OFFSETS_START.saturating_add(SIGNATURE_OFFSETS_SIZE),
        );
        instruction_data.push(1);
        instruction_data.push(0);
        instruction_data.extend_from_slice(bytemuck::bytes_of(&offsets));

        assert_eq!(
            verify(&instruction_data, &[], &FeatureSet::all_enabled()),
            Err(PrecompileError::InvalidDataOffsets)
        );
    }

    #[test]
    fn test_invalid_signature_header() {
        let message = b"falcon invalid signature";
        let secret = SecretKey::generate().expect("key generation failed");
        let signature = secret.sign(message).expect("signing failed");
        let mut instruction =
            new_falcon512_instruction_with_signature(message, &signature, secret.public_key());

        let offsets_slice = &instruction.data
            [SIGNATURE_OFFSETS_START..SIGNATURE_OFFSETS_START + SIGNATURE_OFFSETS_SIZE];
        let offsets: &Falcon512SignatureOffsets = from_bytes(offsets_slice);
        let sig_offset = offsets.signature_offset as usize;
        instruction.data[sig_offset] = SIGNATURE_HEADER ^ 0x01;

        assert_eq!(
            verify(&instruction.data, &[], &FeatureSet::all_enabled()),
            Err(PrecompileError::InvalidSignature)
        );
    }
}
