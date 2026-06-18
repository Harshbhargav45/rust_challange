use super::*;

fn sample_person() -> Person {
    Person {
        name: "André".to_string(),
        age: 30,
        active: true,
    }
}

#[test]
fn borsh_can_save_and_load() {
    let person = sample_person();
    let mut storage = Storage::new(Borsh);
    assert!(!storage.has_data());
    storage.save(&person).unwrap();
    assert!(storage.has_data());
    let loaded = storage.load().unwrap();
    assert_eq!(loaded, person);
}

#[test]
fn wincode_can_save_and_load() {
    let person = sample_person();
    let mut storage = Storage::new(Wincode);
    storage.save(&person).unwrap();
    let loaded = storage.load().unwrap();
    assert_eq!(loaded, person);
}

#[test]
fn json_can_save_and_load() {
    let person = sample_person();
    let mut storage = Storage::new(Json);
    storage.save(&person).unwrap();
    let loaded = storage.load().unwrap();
    assert_eq!(loaded, person);
}

#[test]
fn convert_json_to_borsh() {
    let person = sample_person();
    let mut json_storage = Storage::new(Json);
    json_storage.save(&person).unwrap();

    let borsh_storage = json_storage.convert(Borsh).unwrap();
    let loaded = borsh_storage.load().unwrap();
    assert_eq!(loaded, person);
}
