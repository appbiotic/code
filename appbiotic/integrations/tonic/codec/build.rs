use std::{env, path::PathBuf};

use anyhow::anyhow;
use protobuf_codegen::Customize;

fn main() -> Result<(), anyhow::Error> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let workspace_dir = manifest_dir
        .ancestors()
        .skip_while(|p| !p.join("WORKSPACE.bazel").exists())
        .next()
        .ok_or_else(|| {
            anyhow!("Did not find WORKSPACE.bazel file in CARGO_MANIFEST_DIR ancestors")
        })?;
    let out_dir = manifest_dir.join("src").join("test_pb");

    let proto_files = [
        "appbiotic/integrations/tonic/codec/test/account.proto",
        // "appbiotic/integrations/tonic/codec/test/group.proto",
        "appbiotic/integrations/tonic/codec/test/color/themes.proto",
    ]
    .into_iter()
    .map(|f| PathBuf::from(f));

    let mut codegen = protobuf_codegen::Codegen::new();

    codegen.protoc().includes(&[workspace_dir]);

    for proto_file in proto_files {
        codegen.input(workspace_dir.join(proto_file));
    }

    codegen.out_dir(out_dir);
    codegen.customize(
        Customize::default()
            .gen_mod_rs_hierarchy_out_dir_mod_name("test_pb".to_owned())
            .gen_mod_rs(false),
    );
    codegen.run()
}
