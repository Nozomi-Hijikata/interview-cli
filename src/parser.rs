use markdown::mdast::{Heading, Node, Paragraph, Text};
use std::io;
use std::vec::Vec;

pub fn parse_answers(node: &Node) -> Vec<(&str, u32)> {
    let mut answers = Vec::new();
    let mut question = "";
    let mut score = 0;

    if let Node::Root(root) = node {
        for child in &root.children {
            match child {
                Node::Heading(heading) if is_question_heading(heading) => {
                    if !question.is_empty() {
                        answers.push((question, score));
                    }
                    question = extract_text(heading);
                    // Reset score for this question
                    score = 0;
                }
                Node::Heading(heading) if is_score_heading(heading) => {
                    score = 0;
                }
                Node::Paragraph(paragraph) => {
                    if let Some(text) = extract_text_from_paragraph(paragraph) {
                        if !question.is_empty() && score == 0 {
                            score = text.parse().unwrap_or(0);
                        }
                    }
                }
                _ => {}
            }
        }
        if !question.is_empty() {
            answers.push((question, score));
        }
    }

    answers
}

pub fn write_answers(content: &str, total_score: &u32) -> io::Result<String> {
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    for i in 0..lines.len() {
        if lines[i].starts_with("# 合計点数") {
            let score_line = format!("合計点: {}", total_score);
            if i + 1 < lines.len() && !lines[i + 1].starts_with('#') {
                lines[i + 1] = score_line;
            } else {
                lines.insert(i + 1, score_line);
            }
            break;
        }
    }

    Ok(lines.join("\n"))
}

fn is_question_heading(heading: &Heading) -> bool {
    heading.depth == 1
}

fn is_score_heading(heading: &Heading) -> bool {
    if let Some(Node::Text(Text { value, .. })) = heading.children.first() {
        value == "点数"
    } else {
        false
    }
}

fn extract_text(heading: &Heading) -> &str {
    if let Some(Node::Text(Text { value, .. })) = heading.children.first() {
        value
    } else {
        ""
    }
}

fn extract_text_from_paragraph(paragraph: &Paragraph) -> Option<&str> {
    if let Some(Node::Text(Text { value, .. })) = paragraph.children.first() {
        Some(value)
    } else {
        None
    }
}
