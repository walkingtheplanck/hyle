use hyle_compiler::{compile, CompileInput, CompileOptions, SourceFile};
use hyle_sole::{
    decode_sole_json, decode_sole_json_bytes, encode_sole_json, encode_sole_json_bytes, SoleModule,
};

const GAME: &str = include_str!("../../../examples/game.hyle");
const GAME_SOLE_JSON: &str = include_str!("../../../examples/game.sole.json");

#[test]
fn decodes_sole_json() {
    let module = decode_sole_json(GAME_SOLE_JSON).expect("decode .sole.json");

    assert_eq!(module.version, "0.1");
    assert_eq!(module.world.dimensions, 3);
    assert_eq!(module.models.len(), 3);
    assert_eq!(module.rules.len(), 4);
}

#[test]
fn encodes_sole_json() {
    let module = SoleModule::from_json_str(GAME_SOLE_JSON).expect("decode .sole.json");

    assert_eq!(
        encode_sole_json(&module).expect("encode"),
        GAME_SOLE_JSON.trim_end()
    );
    assert_eq!(
        module.to_json_string().expect("encode"),
        GAME_SOLE_JSON.trim_end()
    );
    assert_eq!(module.to_string(), GAME_SOLE_JSON.trim_end());
}

#[test]
fn round_trips_sole_json_bytes() {
    let module = decode_sole_json_bytes(GAME_SOLE_JSON.as_bytes()).expect("decode bytes");
    let bytes = encode_sole_json_bytes(&module).expect("encode bytes");
    let decoded = decode_sole_json_bytes(&bytes).expect("decode encoded bytes");

    assert_eq!(decoded, module);
}

#[test]
fn compiled_game_hyle_matches_decoded_game_sole_json() {
    let compiled = compile(
        CompileInput {
            source: SourceFile::new("game.hyle", GAME),
            module_name: None,
        },
        CompileOptions::default(),
    )
    .expect("compile game.hyle");

    let decoded = decode_sole_json(GAME_SOLE_JSON).expect("decode game.sole.json");
    let compiled_json = encode_sole_json(&compiled.module).expect("encode compiled module");
    let decoded_json = encode_sole_json(&decoded).expect("encode decoded module");

    assert_eq!(compiled.module, decoded);
    assert_eq!(compiled_json, decoded_json);
}
