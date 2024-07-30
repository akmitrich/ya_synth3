fn main() {
    println!("cargo::rerun-if-changed=tts/tts.proto");
    tonic_build::configure()
        .compile(
            &["tts/v3/tts_service.proto", "tts/v3/tts.proto"],
            &[".", "googleapis"],
        )
        .unwrap();
}
