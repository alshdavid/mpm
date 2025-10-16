pub fn format_name(input: &str) -> String {
    if !input.contains("@") {
        return slugify::slugify(input, "", "-", None);
    }

    let Some((name, version)) = input.split_once("@") else {
        todo!();
    };

    let name = slugify::slugify(name, "", "-", None);
    format!("{}@{}", name, version)
}
