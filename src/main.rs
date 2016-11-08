#![feature(plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate maud;

mod generator;

use iron::prelude::*;
use iron::status;
use std::env;

#[derive(Debug)]
struct TemplateItems {
    nom: String,
    akk: String,
    was: String,
}

#[derive(Debug)]
struct Question {
    question: String,
    answer_1_template: TemplateItems,
}

#[derive(Debug)]
struct QuestionsTopic {
    topic: String,
    questions: Vec<Question>,
}

struct Templates {
    answer_1_template: Vec<String>,
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    port_str.parse().unwrap_or(8080)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    let name = "Juju";
    let templates = Templates {
        answer_1_template: vec!["{name} beherrscht {akk} {was}.".to_string(),
                                "{nom} {was} zeigt keine Schwierigkeiten für {name}.".to_string()],
    };
    let topic_1 = QuestionsTopic {
        topic: "Mathe".to_string(),
        questions: vec![Question {
                            question: "ZR10".to_string(),
                            answer_1_template: TemplateItems {
                                nom: "der".to_string(),
                                akk: "den".to_string(),
                                was: "ZR 10".to_string(),
                            },
                        }],
    };
    let all_topics = vec![topic_1];
    Iron::new(move |r: &mut Request| {
        let markup = html! {
            h1 "Zeugnis"
            @for topic in &all_topics {
                p {
                    (topic.topic)
                }
                @for question in &topic.questions {
                    p {
                        (question.question)
                    }
                    @for template in &templates.answer_1_template {
                        p {
                            (generator::generate_one_suggestion(
                                &template,
                                &name,
                                &question.answer_1_template.nom,
                                &question.answer_1_template.akk,
                                &question.answer_1_template.was))
                        }
                    }
                }

            }
        };
        Ok(Response::with((status::Ok, markup)))
    })
    .http(("0.0.0.0", get_server_port()))
    .unwrap();
}
