pub fn editable(authors: Vec<String>) -> String {
    let mut o = "".to_string();
    for a in authors {
        o.push_str(&a);
        o.push_str(",");
    }
    return o;
}

pub fn fancy_html(authors: String) -> String {
    let mut o = "".to_string();
    let mut index = 0;
    let b: Vec<&str> = authors.split(",").collect();
    for a in b {
        let fa = a.replace(" ", "-");
        let link = format!("/author/{fa}");

        if index > 0 {
            o.push_str(format!("<a href={link}>+ {a}</a>").as_str());
        } else {
            o.push_str(format!("<a href={link}>{a}</a>").as_str());
        }
        index = index + 1;
    }
    return o;
}
