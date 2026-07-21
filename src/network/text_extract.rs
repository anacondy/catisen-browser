use scraper::{node::Node, ElementRef, Html, Selector};

const PARA_MARK: &str = "\u{001E}";

fn is_ignored_tag(name: &str) -> bool {
    matches!(name, "script" | "style" | "noscript" | "svg" | "head" | "meta" | "link")
}

fn is_paragraph_tag(name: &str) -> bool {
    matches!(name, "p" | "br" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "div" | "article" | "section" | "li" | "tr")
}

fn is_heading_tag(name: &str) -> bool {
    matches!(name, "h1" | "h2" | "h3" | "h4" | "h5" | "h6")
}

fn push_para_mark(out: &mut String) {
    if !out.ends_with(PARA_MARK) {
        out.push_str(PARA_MARK);
    }
}

fn walk_element(element: ElementRef<'_>, out: &mut String) {
    let tag_name = element.value().name();
    if is_ignored_tag(tag_name) {
        return;
    }

    if is_paragraph_tag(tag_name) {
        push_para_mark(out);
    }

    if is_heading_tag(tag_name) {
        // Add a marker for headings to make them visually distinct in Text Mode
        if tag_name == "h1" || tag_name == "h2" {
            out.push_str("## ");
        } else {
            out.push_str("# ");
        }
    }

    if tag_name == "img" {
        let src = element.value().attr("src").or_else(|| element.value().attr("data-src")).unwrap_or("");
        let alt = element.value().attr("alt").unwrap_or("Image");
        if !src.is_empty() {
            let img_marker = format!(" [IMAGE: {alt} | {src}] ");
            out.push_str(&img_marker);
        }
    }

    let is_link = tag_name == "a";
    let link_href = if is_link { element.value().attr("href").unwrap_or("") } else { "" };
    
    if is_link && !link_href.is_empty() && !link_href.starts_with("javascript:") {
        // We can track if we want to format links like markdown
        // but for now let's just make sure text isn't lost.
    }

    for child in element.children() {
        if let Some(child_el) = ElementRef::wrap(child) {
            walk_element(child_el, out);
            continue;
        }

        if let Node::Text(text) = child.value() {
            let cleaned = text.trim();
            if !cleaned.is_empty() {
                out.push_str(cleaned);
                out.push(' ');
            }
        }
    }

    if is_link && !link_href.is_empty() && !link_href.starts_with("javascript:") {
        // Append the target URL
        let link_marker = format!(" (→ {})", link_href);
        out.push_str(&link_marker);
    }

    if is_paragraph_tag(tag_name) {
        push_para_mark(out);
    }
}

pub fn extract_clean_text(html: &str) -> String {
    let document = Html::parse_document(html);

    let mut raw = String::new();
    if let Ok(selector) = Selector::parse("body") {
        let mut had_body = false;
        for body in document.select(&selector) {
            walk_element(body, &mut raw);
            push_para_mark(&mut raw);
            had_body = true;
            break; // Only parse the first body
        }

        if !had_body {
            walk_element(document.root_element(), &mut raw);
        }
    } else {
        walk_element(document.root_element(), &mut raw);
    }

    raw.split(PARA_MARK)
        .map(|segment| segment.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}
