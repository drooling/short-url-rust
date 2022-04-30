use std::io::Write;
use ureq::Agent;
use urlencoding;

fn shorten(service: &str, url: &str, session: ureq::Agent) -> Result<String, ureq::Error> {
    let body: String = session
        .get(
            format!(
                "{}",
                &service.replace("url={}", format!("url={}", &url).as_str())
            )
            .as_str(),
        )
        .call()?
        .into_string()?;

    Ok(body)
}

fn main() {
    let services: Vec<&str> = vec![
        "https://clck.ru/--?url={}",
        "https://is.gd/create.php?format=simple&url={}&logstats=0",
        "https://v.gd/create.php?format=simple&url={}&logstats=0",
    ];
    let mut url = String::new();
    print!("URL to shorten: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut url)
        .expect("Couldn't understand that");
    url = urlencoding::encode(url.trim().to_string().as_str())
        .to_owned()
        .to_string();

    for (index, service) in services.iter().enumerate() {
        let domain: &str = service.split("https://").collect::<Vec<&str>>()[1]
            .split("/")
            .collect::<Vec<&str>>()[0];
        println!("[{}] -> {}", index, domain);
    }
    let mut service = String::new();
    print!("Service number: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut service)
        .expect("Couldn't understand that");
    let service_num: usize = match service.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    let agent: Agent = ureq::AgentBuilder::new().build();
    match shorten(services[service_num], &url, agent) {
        Ok(url) => println!("Your shortened url -> {}", url),
        Err(_) => println!("Err"),
    };
}
