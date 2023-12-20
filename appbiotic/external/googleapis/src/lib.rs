#[cfg(feature = "protos")]
pub mod protos;

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(all(feature = "protos", feature = "rpc"))]
    fn it_works() {
        use bytes::Bytes;
        use prost::Message;

        use super::protos::google::rpc;

        let serialized = Bytes::from(
            rpc::Status {
                code: rpc::Code::Internal.into(),
                message: "Something".to_owned(),
                details: Vec::default(),
            }
            .encode_to_vec(),
        );

        let deserialized = rpc::Status::decode(serialized).expect("deserialized");

        assert_eq!(deserialized.code, Into::<i32>::into(rpc::Code::Internal));
        assert_eq!(deserialized.message, "Something");
        assert!(deserialized.details.is_empty());
    }
}
