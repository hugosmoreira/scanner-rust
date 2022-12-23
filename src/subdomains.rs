pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
    // make a GET request to crt.sh to retrieve a list of subdomains for the given target
    let entries: Vec<CrtShEntry> = http_client
    .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
    .send()?
    .json()?;

    // clean and dedup results
    // create a HashSet to store unique subdomains
    let mut subdomains: HashSet<String> = entries
        // convert each entry into a Vec of subdomains
        .into_iter()
        .map(|entry| {
            entry
                .name_value
                .split("\n")
                .map(|subdomain| subdomain.trim().to_string())
                .collect:<Vec<String>>()
        })
        // flatten the Vec<Vec<String>> into a single Vec<String>
        .flatten()
        // filter out the target itself and any subdomains with a '*'
        .filter(|subdomain: &String| subdomain != target)
        .filter(|subdomain: &String| !subdomain.contains("*"))
        // add the subdomains to the HashSet
        .collect();
    // create a Vec of Subdomain objects from the unique subdomains
    let subdomains: Vec<Subdomain> = subdomains
        .into_iter()
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        // filter out any subdomains that don't resolve to an IP address
        .filter(resolves)
        .collect();
    Ok(subdomains)        
}
