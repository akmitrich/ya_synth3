mod ya {
    tonic::include_proto!("speechkit.tts.v3");
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let folder_id = std::env::var("YANDEX_FOLDER").unwrap();
    let iam_token = std::env::var("YANDEX_IAM").unwrap();
    println!("{:?}/{:?}", folder_id, iam_token);

    let mut req = tonic::Request::new(ya::UtteranceSynthesisRequest {
        model: String::new(),
        hints: vec![
            ya::Hints {
                hint: Some(ya::hints::Hint::Voice("lera".into())),
            },
            ya::Hints {
                hint: Some(ya::hints::Hint::Role("neutral".into())),
            },
        ],
        output_audio_spec: Some(ya::AudioFormatOptions {
            audio_format: Some(ya::audio_format_options::AudioFormat::RawAudio(
                ya::RawAudio {
                    audio_encoding: ya::raw_audio::AudioEncoding::Linear16Pcm.into(),
                    sample_rate_hertz: 8000,
                },
            )),
        }),
        loudness_normalization_type:
            ya::utterance_synthesis_request::LoudnessNormalizationType::Lufs.into(),
        unsafe_mode: true,
        utterance: Some(ya::utterance_synthesis_request::Utterance::Text(
            "мама мыла раму".into(),
        )),
    });
    let headers = req.metadata_mut();
    headers.insert(
        "authorization",
        format!("Bearer {}", iam_token).parse().unwrap(),
    );
    headers.insert("x-folder-id", folder_id.parse().unwrap());
    println!("REQ: {:?}", req);

    let channel = tonic::transport::Channel::from_static("https://tts.api.cloud.yandex.net:443")
        .tls_config(tls_config())
        .unwrap()
        .connect()
        .await
        .unwrap();
    println!("Channel: {:?}", channel);
    let mut client = ya::synthesizer_client::SynthesizerClient::new(channel);
    println!("Client: {:?}", client);
    let resp = client.utterance_synthesis(req).await.unwrap();
    let mut stream = resp.into_inner();
    let mut record = Vec::new();
    while let Some(x) = stream.message().await.unwrap() {
        println!("{} -> {} : {:?}", x.start_ms, x.length_ms, x.text_chunk);
        x.audio_chunk
            .map(|a| record.extend_from_slice(a.data.as_slice()))
            .unwrap();
    }
    tokio::fs::write("../audio.pcm", record).await.unwrap();
}

pub fn tls_config() -> tonic::transport::ClientTlsConfig {
    const CERT_PATH: &str = "/etc/ssl/certs/GlobalSign_Root_CA.pem";
    let pem = std::fs::read(CERT_PATH).expect("read the cert file");
    let cert = tonic::transport::Certificate::from_pem(pem);
    tonic::transport::ClientTlsConfig::new().ca_certificate(cert)
}
