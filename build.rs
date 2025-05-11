fn main() {
    cynic_codegen::register_schema("acl")
        .from_sdl_file("schemas/acl.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
