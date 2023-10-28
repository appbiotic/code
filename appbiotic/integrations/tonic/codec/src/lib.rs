use std::marker::PhantomData;

use bytes::{Buf, BufMut};
use protobuf::MessageFull;
use tonic::{
    codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder},
    Status,
};

#[cfg(test)]
mod test_pb;

/// A [`Codec`] that implements `application/grpc+proto` via the `protobuf`
/// library for generated with the "full" flavor.
///
/// TODO: Write a `RustProtobufMessageLiteCodec`
pub struct RustProtobufMessageFullCodec<T, U>(PhantomData<(T, U)>);

impl<T, U> Codec for RustProtobufMessageFullCodec<T, U>
where
    T: MessageFull,
    U: MessageFull,
{
    type Encode = T;
    type Decode = U;

    type Encoder = RustProtobufMessageFullEncoder<T>;
    type Decoder = RustProtobufMessageFullDecoder<U>;

    fn encoder(&mut self) -> Self::Encoder {
        RustProtobufMessageFullEncoder(PhantomData)
    }

    fn decoder(&mut self) -> Self::Decoder {
        RustProtobufMessageFullDecoder(PhantomData)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RustProtobufEncodingError {
    #[error("Architecture has not enough capacity to write message `{message}` requested {requested_bytes} bytes, but only {max_bytes} maximum bytes supported")]
    ArchitectureHasNotEnoughCapacity {
        message: String,
        requested_bytes: u64,
        max_bytes: usize,
    },
    #[error("Provided buffer has not enough capacity to write message `{message}` requested {requested_bytes} bytes, but only {remaining_bytes} remaining bytes")]
    BufferHasNotEnoughCapacity {
        message: String,
        requested_bytes: u64,
        remaining_bytes: usize,
    },
    #[error("Writing message to buffer failed: {message}")]
    WriteToBufferFailed { message: String },
}

/// A [`Decoder`] for encoding the `T` impl of [`MessageFull`].
pub struct RustProtobufMessageFullEncoder<T>(PhantomData<T>);

impl<T: MessageFull> Encoder for RustProtobufMessageFullEncoder<T> {
    type Item = T;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        let required_size = item.compute_size();
        let required: usize = required_size.try_into().map_err(|_| {
            Status::failed_precondition(
                format!("Architecture does not have enough capacity to write message `{}` requested {required_size} bytes, but only {} maximum bytes supported", Self::Item::descriptor().full_name(), usize::MAX))
        })?;
        let remaining = buf.remaining_mut();
        if required > buf.remaining_mut() {
            return Err(Status::failed_precondition(format!("Provided buffer does not have enough capacity to write message `{}` requested {required_size} bytes, but only {remaining} remaining bytes", Self::Item::descriptor().full_name())));
        }

        item.write_to_writer(&mut buf.writer())
            .map_err(|error| Status::internal(error.to_string()))?;

        Ok(())
    }
}

/// A [`Decoder`] that knows how to decode `U`.
pub struct RustProtobufMessageFullDecoder<U>(PhantomData<U>);

impl<U: MessageFull + Default> Decoder for RustProtobufMessageFullDecoder<U> {
    type Item = U;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Some(
            *Self::Item::descriptor()
                .parse_from_reader(&mut buf.reader())
                .map_err(|error| Status::internal(error.to_string()))?
                .downcast_box::<U>()
                .map_err(|_| {
                    Status::internal(format!(
                        "Failed to downcast decoded for message type `{}`",
                        Self::Item::descriptor().full_name()
                    ))
                })?,
        ))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
