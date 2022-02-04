use std::fs;

fn main() {
    (1..=6)
        .into_iter()
        .map(|n| format!("../json/hsk-level-{n}.json"))
        .for_each(|path| {
            let gen: String = fs::read_to_string(path.clone())
                .expect("Error reading")
                .lines()
                .filter(|line| {
                    !line.contains("CL")
                        && (!line.starts_with("      \"") || line.chars().all(|c| c.is_ascii()))
                })
                .collect::<Vec<_>>()
                .join("\n");

            fs::write(
                format!("{}-no-cl.json", path.chars().take(19).collect::<String>()),
                gen,
            )
            .expect("Error writing");
        });
}
