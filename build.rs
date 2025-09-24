fn main() {
    cynic_codegen::register_schema("acl")
        .from_sdl_file("schemas/acl.graphql")
        .unwrap()
        .as_default()
        .unwrap();

    cynic_codegen::register_schema("shared")
        .from_sdl_file("schemas/shared.graphql")
        .unwrap();
}
