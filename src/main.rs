mod generator;

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

fn main() {
    let name = "Juju";
    let templates = Templates {
        answer_1_template: vec!["{name} beherrscht {akk} {was}.".to_string(),
                                "{nom} {was} zeigt keine Schwierigkeiten für {name}.".to_string()],
    };
    let questions = QuestionsTopic {
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
    println!("{:?}", questions);
}
