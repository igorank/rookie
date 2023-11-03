use rookie;

fn main() {
    let cookies_path = "C:\\Users\\Igor\\AppData\\Roaming\\Octo Browser\\tmp\\adf0db8a\\Default\\Network\\Cookies";
    let key_path = "C:\\Users\\Igor\\AppData\\Roaming\\Octo Browser\\tmp\\adf0db8a\\Local State";

    let cookies = rookie::any_browser(cookies_path, None,
                                                   Some(key_path)).unwrap();
    println!("{:?}", cookies);
}