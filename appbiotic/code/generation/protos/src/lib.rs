use std::{
    collections::HashSet,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use derive_new::new;
use prost::Message;

pub struct ProtosGenerationConfig {
    pub crate_dir: PathBuf,
    pub crate_name: String,
    pub include_dir: PathBuf,
    pub mod_name: String,
    pub proto_files: Vec<PathBuf>,
    pub import_paths: Vec<PathBuf>,
    pub extern_paths: Vec<ExternPath>,
}

#[derive(strum::AsRefStr)]
#[strum(serialize_all = "PascalCase")]
pub enum SwiftNaming {
    FullPath,
    PathToUnderscores,
    DropPath,
}

impl Default for SwiftNaming {
    fn default() -> Self {
        Self::PathToUnderscores
    }
}

pub struct SwiftModuleMappings {
    pub module_name: String,
    pub proto_files: Vec<PathBuf>,
}

impl SwiftModuleMappings {
    pub fn new(module_name: String) -> Self {
        Self {
            module_name,
            proto_files: Vec::default(),
        }
    }

    pub fn with_proto_file(self, proto_file: impl AsRef<Path>) -> Self {
        let mut new_proto_files = self.proto_files;
        new_proto_files.push(proto_file.as_ref().to_owned());
        Self {
            module_name: self.module_name,
            proto_files: new_proto_files,
        }
    }

    pub fn with_proto_files<T>(self, proto_files: T) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<Path>,
    {
        let mut new_proto_files = self.proto_files;
        new_proto_files.extend(proto_files.into_iter().map(|x| x.as_ref().to_owned()));
        Self {
            module_name: self.module_name,
            proto_files: new_proto_files,
        }
    }
}

impl ProtosGenerationConfig {
    pub fn new(
        crate_dir: impl AsRef<Path>,
        crate_name: &str,
        include_dir: impl AsRef<Path>,
    ) -> Self {
        Self {
            crate_dir: crate_dir.as_ref().to_owned(),
            crate_name: crate_name.to_string(),
            include_dir: include_dir.as_ref().to_owned(),
            mod_name: "protos".to_string(),
            proto_files: Vec::default(),
            import_paths: Vec::default(),
            extern_paths: Vec::default(),
        }
    }

    pub fn with_include_dir(self, include_dir: impl AsRef<Path>) -> Self {
        let include_dir = include_dir.as_ref().to_owned();
        Self {
            crate_dir: self.crate_dir,
            crate_name: self.crate_name,
            include_dir,
            mod_name: self.mod_name,
            proto_files: self.proto_files,
            import_paths: self.import_paths,
            extern_paths: self.extern_paths,
        }
    }

    pub fn with_proto_file(self, proto_file: impl AsRef<Path>) -> Self {
        let mut proto_files = self.proto_files;
        proto_files.push(proto_file.as_ref().to_owned());
        Self {
            crate_dir: self.crate_dir,
            crate_name: self.crate_name,
            include_dir: self.include_dir,
            mod_name: self.mod_name,
            proto_files,
            import_paths: self.import_paths,
            extern_paths: self.extern_paths,
        }
    }

    pub fn with_proto_files<T>(self, proto_files: T) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<Path>,
    {
        let mut new_proto_files = self.proto_files;
        new_proto_files.extend(proto_files.into_iter().map(|x| x.as_ref().to_owned()));
        Self {
            crate_dir: self.crate_dir,
            crate_name: self.crate_name,
            include_dir: self.include_dir,
            mod_name: self.mod_name,
            proto_files: new_proto_files,
            import_paths: self.import_paths,
            extern_paths: self.extern_paths,
        }
    }

    pub fn with_import_path(self, import_path: impl AsRef<Path>) -> Self {
        let mut new_import_paths = self.import_paths;
        new_import_paths.push(import_path.as_ref().to_owned());
        Self {
            crate_dir: self.crate_dir,
            crate_name: self.crate_name,
            include_dir: self.include_dir,
            mod_name: self.mod_name,
            proto_files: self.proto_files,
            import_paths: new_import_paths,
            extern_paths: self.extern_paths,
        }
    }

    pub fn with_import_paths<T>(self, import_paths: T) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<Path>,
    {
        let mut new_import_paths = self.import_paths;
        new_import_paths.extend(import_paths.into_iter().map(|x| x.as_ref().to_owned()));
        Self {
            crate_dir: self.crate_dir,
            crate_name: self.crate_name,
            include_dir: self.include_dir,
            mod_name: self.mod_name,
            proto_files: self.proto_files,
            import_paths: new_import_paths,
            extern_paths: self.extern_paths,
        }
    }

    pub fn with_extern_path(self, extern_path: ExternPath) -> Self {
        let mut new_extern_paths = self.extern_paths;
        new_extern_paths.push(extern_path);
        Self {
            crate_dir: self.crate_dir,
            crate_name: self.crate_name,
            include_dir: self.include_dir,
            mod_name: self.mod_name,
            proto_files: self.proto_files,
            import_paths: self.import_paths,
            extern_paths: new_extern_paths,
        }
    }

    pub fn with_extern_paths<T>(self, extern_paths: T) -> Self
    where
        T: IntoIterator<Item = ExternPath>,
    {
        let mut new_extern_paths = self.extern_paths;
        new_extern_paths.extend(extern_paths);
        Self {
            crate_dir: self.crate_dir,
            crate_name: self.crate_name,
            include_dir: self.include_dir,
            mod_name: self.mod_name,
            proto_files: self.proto_files,
            import_paths: self.import_paths,
            extern_paths: new_extern_paths,
        }
    }
}

#[derive(new, PartialEq, Eq, Hash)]
pub struct ExternPath {
    pub proto_path: &'static str,
    pub rust_path: &'static str,
}

pub fn build(config: ProtosGenerationConfig) {
    let manifest_dir = config.crate_dir;
    let out_mod_dir = config
        .mod_name
        .split("::")
        .fold(manifest_dir.join("src"), |acc, x| acc.join(x));
    out_mod_dir
        .ancestors()
        .find(|path| path.eq(&manifest_dir))
        .expect("CARGO_MANIFEST_DIR is an ancestor of `out_mod_name` directory");

    if !out_mod_dir.exists() {
        fs::create_dir(&out_mod_dir).expect("create out_mod_dir");
    }
    let crate_mod_name = config.crate_name.replace('-', "_");

    let proto_file_paths: HashSet<String> = config
        .proto_files
        .iter()
        .map(|p| format!("{}/{}", config.include_dir.display(), p.display()))
        .collect();

    // NOTE: Import paths has include dir last to avoid giving precedence to
    //       undesired paths in a monorepo `include_dir`.
    let mut import_paths = config.import_paths;
    import_paths.push(config.include_dir);

    let descriptor_path = out_mod_dir.join("_descriptor.bin");

    // TODO: Create new include file generation.
    // SEE: https://github.com/tokio-rs/prost/issues/880#issuecomment-1836735892
    let mut builder = tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // for older systems
        // NOTE: Suppress rerun-if-changed for workspace directory
        // SEE: https://github.com/hyperium/tonic/issues/1070
        .emit_rerun_if_changed(false)
        .file_descriptor_set_path(&descriptor_path)
        .generate_default_stubs(true)
        .build_server(true)
        .build_client(true)
        .include_file("_includes.rs")
        .out_dir(&out_mod_dir);

    // NOTE: Add rerun-if-changed only listed files above
    // SEE: https://github.com/hyperium/tonic/issues/1070
    for proto_file_path in proto_file_paths.iter() {
        println!("cargo:rerun-if-changed={}", proto_file_path);
    }

    for extern_path in config.extern_paths {
        builder = builder.extern_path(extern_path.proto_path, extern_path.rust_path);
    }

    builder
        .compile(&Vec::from_iter(proto_file_paths), &import_paths)
        .expect("protoc compile");

    // NOTE: Do not include dependencies in descriptor
    // SEE: https://github.com/tokio-rs/prost/issues/880
    for proto_file in &config.proto_files {
        eprintln!("proto_file: {}", proto_file.display());
    }
    let descriptor_bytes = std::fs::read(&descriptor_path).unwrap();
    let mut descriptor = prost_types::FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();
    for descriptor_file in &descriptor.file {
        eprintln!("descriptor_file: {}", descriptor_file.name());
    }

    descriptor.file.retain(|x| {
        if config.proto_files.contains(&PathBuf::from(x.name())) {
            eprintln!("retaining descriptor_file: {}", x.name());
            true
        } else {
            eprintln!("removing descriptor_file: {}", x.name());
            false
        }
    });

    // TODO: Debug why writing back the descriptor was commented out.
    // std::fs::write(&descriptor_path, descriptor.encode_to_vec())?;

    let mut names = std::fs::File::create(out_mod_dir.join("_names.rs")).unwrap();
    names.write_fmt(format_args!("// TODO: Remove Name implementation after https://github.com/tokio-rs/prost/issues/921\n")).unwrap();

    let mut externs = std::fs::File::create(out_mod_dir.join("_externs.rs")).unwrap();
    externs
        .write_all(
            "pub fn externs() -> Vec<appbiotic_code_generation_protos::ExternPath> {\n".as_bytes(),
        )
        .unwrap();
    externs.write_all("    vec![\n".as_bytes()).unwrap();

    let mut mod_rs = std::fs::File::create(out_mod_dir.join("mod.rs")).unwrap();
    for include in &["_externs.rs", "_includes.rs", "_names.rs"] {
        mod_rs
            .write_fmt(format_args!("include!(\"{include}\");\n"))
            .unwrap()
    }
    mod_rs
        .write_all(
            "pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!(\"_descriptor.bin\");\n"
                .as_bytes(),
        )
        .unwrap();

    let mut gitignore = std::fs::File::create(out_mod_dir.join(".gitignore")).unwrap();
    gitignore.write_all("_descriptor.bin".as_bytes()).unwrap();

    for file in &descriptor.file {
        let package_name = file.package();
        let module_name = itertools::join(
            package_name
                .split('.')
                .map(|x| x.replace('-', "_"))
                .map(|x| if x.eq("type") { "r#type".to_owned() } else { x }),
            "::",
        );

        for message in &file.message_type {
            let message_name = message.name();

            externs
            .write_fmt(format_args!(
                "        appbiotic_code_generation_protos::ExternPath {{ proto_path: \".{}.{}\", rust_path: \"::{}::{}::{}::{}\" }},\n",
                package_name, message_name, crate_mod_name, config.mod_name, module_name, message_name
            ))
            .unwrap();

            println!(
                "package_name: {}, message_name: {}, module_name: {}",
                package_name, message_name, module_name
            );

            names.write_fmt(format_args!("\n")).unwrap();
            names
                .write_fmt(format_args!(
                    "impl prost::Name for {module_name}::{message_name} {}\n",
                    "{"
                ))
                .unwrap();
            names
                .write_fmt(format_args!(
                    "    const NAME: &'static str = \"{message_name}\";\n"
                ))
                .unwrap();
            names
                .write_fmt(format_args!(
                    "    const PACKAGE: &'static str = \"{package_name}\";\n"
                ))
                .unwrap();
            names.write_fmt(format_args!("{}\n", "}")).unwrap();
        }
    }
    externs.write_all("    ]\n".as_bytes()).unwrap();
    externs.write_all("}\n".as_bytes()).unwrap();
}
