fn main() {
    let _ = dotenv::dotenv();
    let folder_id = std::env::var("YANDEX_FOLDER").unwrap();
    let iam_token = std::env::var("YANDEX_IAM").unwrap();
    println!("{:?}/{:?}", folder_id, iam_token);
}
