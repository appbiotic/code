use std::{env, path::PathBuf};

use appbiotic_code_generation_protos::{build, ProtosGenerationConfig};

fn main() {
    let include_dir = PathBuf::from("../../../../../external/googleapis");

    let mut proto_files: Vec<&str> = Vec::new();
    if env::var_os("CARGO_FEATURE_RPC").is_some() {
        proto_files.extend([
            "google/rpc/code.proto",
            "google/rpc/error_details.proto",
            "google/rpc/http.proto",
            "google/rpc/status.proto",
        ]);
    }

    if env::var_os("CARGO_FEATURE_TYPES").is_some() {
        proto_files.extend([
            "google/type/calendar_period.proto",
            "google/type/color.proto",
            "google/type/date.proto",
            "google/type/datetime.proto",
            "google/type/dayofweek.proto",
            "google/type/decimal.proto",
            "google/type/expr.proto",
            "google/type/fraction.proto",
            "google/type/interval.proto",
            "google/type/latlng.proto",
            "google/type/localized_text.proto",
            "google/type/money.proto",
            "google/type/month.proto",
            "google/type/phone_number.proto",
            "google/type/postal_address.proto",
            "google/type/quaternion.proto",
            "google/type/timeofday.proto",
        ]);
    }

    build(
        ProtosGenerationConfig::new(
            env!("CARGO_MANIFEST_DIR"),
            env!("CARGO_PKG_NAME"),
            &include_dir,
        )
        .with_include_dir(include_dir)
        .with_proto_files(proto_files),
    );
}
