
pub fn generate_one_suggestion(template: &str,
                               name: &str,
                               nom: &str,
                               akk: &str,
                               was: &str)
                               -> String {
    uppercase_first_letter(template.replace("{name}", &uppercase_first_letter(name.to_string()))
        .replace("{nom}", nom)
        .replace("{akk}", akk)
        .replace("{was}", was))
}

fn uppercase_first_letter(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_one_suggestion_with_akk() {
        let possible_solution = "{name} beherrscht {akk} {was}.";
        let result = generate_one_suggestion(possible_solution, "Maria", "der", "den", "ZR 10");
        println!("{}", result);
        assert_eq!(result, "Maria beherrscht den ZR 10.");
    }

    #[test]
    fn test_generate_one_suggestion_with_nom() {
        let possible_solution = "{nom} {was} zeigt keine Schwierigkeiten für {name}.";
        let result = generate_one_suggestion(possible_solution, "Peter", "der", "den", "ZR 10");
        println!("{}", result);
        assert_eq!(result, "Der ZR 10 zeigt keine Schwierigkeiten für Peter.");
    }

    #[test]
    fn test_uppercase_name() {
        let possible_solution = "{nom} {was} zeigt keine Schwierigkeiten für {name}.";
        let result = generate_one_suggestion(possible_solution, "peter", "der", "den", "ZR 10");
        println!("{}", result);
        assert_eq!(result, "Der ZR 10 zeigt keine Schwierigkeiten für Peter.");
    }
}
