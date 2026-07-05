use crate::state_instruction_skeleton::XxxlGatewayInstructionTag;

pub const XXXL_INSTRUCTION_CODEC_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

pub const INSTRUCTION_TAG_LEN: usize = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlInstructionCodecSkeletonError {
    EmptyInstructionData,
    InvalidInstructionTag,
    InvalidPayloadLength,
    PayloadNotImplemented,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlDecodedInstructionSkeleton<'a> {
    pub tag: XxxlGatewayInstructionTag,
    pub payload: &'a [u8],
}

pub fn decode_instruction_skeleton(
    data: &[u8],
) -> Result<XxxlDecodedInstructionSkeleton<'_>, XxxlInstructionCodecSkeletonError> {
    let (tag_byte, payload) = data
        .split_first()
        .ok_or(XxxlInstructionCodecSkeletonError::EmptyInstructionData)?;

    let tag = XxxlGatewayInstructionTag::try_from(*tag_byte)
        .map_err(|_| XxxlInstructionCodecSkeletonError::InvalidInstructionTag)?;

    Ok(XxxlDecodedInstructionSkeleton { tag, payload })
}

pub fn encode_instruction_skeleton(tag: XxxlGatewayInstructionTag, payload: &[u8]) -> Vec<u8> {
    let mut data = Vec::with_capacity(INSTRUCTION_TAG_LEN + payload.len());
    data.push(u8::from(tag));
    data.extend_from_slice(payload);
    data
}

pub fn require_empty_payload_for_local_skeleton(
    decoded: XxxlDecodedInstructionSkeleton<'_>,
) -> Result<XxxlGatewayInstructionTag, XxxlInstructionCodecSkeletonError> {
    if decoded.payload.is_empty() {
        Ok(decoded.tag)
    } else {
        Err(XxxlInstructionCodecSkeletonError::PayloadNotImplemented)
    }
}

pub fn decode_empty_payload_instruction_skeleton(
    data: &[u8],
) -> Result<XxxlGatewayInstructionTag, XxxlInstructionCodecSkeletonError> {
    let decoded = decode_instruction_skeleton(data)?;
    require_empty_payload_for_local_skeleton(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codec_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_INSTRUCTION_CODEC_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn decode_rejects_empty_instruction_data() {
        assert_eq!(
            decode_instruction_skeleton(&[]),
            Err(XxxlInstructionCodecSkeletonError::EmptyInstructionData)
        );
    }

    #[test]
    fn decode_rejects_invalid_instruction_tag() {
        assert_eq!(
            decode_instruction_skeleton(&[4]),
            Err(XxxlInstructionCodecSkeletonError::InvalidInstructionTag)
        );
        assert_eq!(
            decode_instruction_skeleton(&[u8::MAX]),
            Err(XxxlInstructionCodecSkeletonError::InvalidInstructionTag)
        );
    }

    #[test]
    fn decode_accepts_reserved_instruction_tags() {
        for expected in [
            XxxlGatewayInstructionTag::InitializeGatewayConfig,
            XxxlGatewayInstructionTag::InitializeGuardianSet,
            XxxlGatewayInstructionTag::InitializeMintState,
            XxxlGatewayInstructionTag::ConsumeGatewayMint,
        ] {
            let data = [u8::from(expected)];
            let decoded = decode_instruction_skeleton(&data).expect("reserved tag decodes");

            assert_eq!(decoded.tag, expected);
            assert!(decoded.payload.is_empty());
        }
    }

    #[test]
    fn encode_decode_round_trip_preserves_tag_and_payload() {
        let payload = [1_u8, 2, 3, 4, 5];
        let encoded =
            encode_instruction_skeleton(XxxlGatewayInstructionTag::ConsumeGatewayMint, &payload);
        let decoded = decode_instruction_skeleton(&encoded).expect("encoded data decodes");

        assert_eq!(decoded.tag, XxxlGatewayInstructionTag::ConsumeGatewayMint);
        assert_eq!(decoded.payload, payload);
    }

    #[test]
    fn empty_payload_skeleton_rejects_non_empty_payload_until_payload_layouts_exist() {
        let encoded =
            encode_instruction_skeleton(XxxlGatewayInstructionTag::InitializeGatewayConfig, &[1]);
        let result = decode_empty_payload_instruction_skeleton(&encoded);

        assert_eq!(
            result,
            Err(XxxlInstructionCodecSkeletonError::PayloadNotImplemented)
        );
    }

    #[test]
    fn empty_payload_skeleton_accepts_empty_payload_reserved_tags() {
        let encoded =
            encode_instruction_skeleton(XxxlGatewayInstructionTag::InitializeGatewayConfig, &[]);
        let tag = decode_empty_payload_instruction_skeleton(&encoded).expect("empty payload ok");

        assert_eq!(tag, XxxlGatewayInstructionTag::InitializeGatewayConfig);
    }
}
