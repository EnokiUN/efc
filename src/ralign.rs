pub fn ralign(code: String) -> String {
    let code: Vec<&str> = code.lines().collect();

    let line_width = code.iter().map(|l| l.trim().len()).max().unwrap();

    code.iter()
        .map(|l| {
            let l = l.trim();
            format!("{}{}", " ".repeat(line_width - l.len()), l)
        })
        .collect::<Vec<String>>()
        .join("\n")
}
