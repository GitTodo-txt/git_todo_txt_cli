use super::*;

#[test]
fn serializes_task_as_its_todo_txt_line() {
    let task = Task::from_line("x (A) 2024-01-02 2024-01-01 Buy milk +errands @shop due:2024-01-10")
        .expect("line should parse");

    let json = serde_json::to_string(&task).expect("serialization should succeed");

    assert_eq!(
        json,
        "\"x (A) 2024-01-02 2024-01-01 Buy milk +errands @shop due:2024-01-10\""
    );
}

#[test]
fn deserializes_task_from_a_todo_txt_line() {
    let json = "\"(B) 2024-03-05 Pay rent @home\"";

    let task: Task = serde_json::from_str(json).expect("deserialization should succeed");

    assert!(!task.completed);
    assert_eq!(task.priority, Some('B'));
    assert_eq!(
        task.creation_date,
        Some(chrono::NaiveDate::from_ymd_opt(2024, 3, 5).unwrap())
    );
    assert_eq!(task.completion_date, None);
    assert_eq!(
        task.description,
        vec![
            DescToken::Text("Pay".to_string()),
            DescToken::Text("rent".to_string()),
            DescToken::Context("home".to_string()),
        ]
    );
}

#[test]
fn round_trips_through_serialize_and_deserialize() {
    let original =
        Task::from_line("x (C) 2024-06-01 2024-05-01 Finish report +work @office note:draft")
            .expect("line should parse");

    let json = serde_json::to_string(&original).expect("serialization should succeed");
    let restored: Task = serde_json::from_str(&json).expect("deserialization should succeed");

    assert_eq!(original, restored);
}

#[test]
fn deserialize_rejects_non_string_input() {
    let err = serde_json::from_str::<Task>("42").unwrap_err();

    assert!(err.to_string().contains("todo.txt"));
}

#[test]
fn parses_whole_file_into_tasks() {
    let file = "\
x (A) 2024-01-02 2024-01-01 Buy milk +errands @shop due:2024-01-10
(B) 2024-03-05 Pay rent @home

Just some plain text task
";

    let list = TaskList::from_lines(file).expect("file should parse");

    assert_eq!(list.len(), 3);
    assert!(list[0].completed);
    assert_eq!(list[1].priority, Some('B'));
    assert_eq!(
        list[2].description,
        vec![
            DescToken::Text("Just".to_string()),
            DescToken::Text("some".to_string()),
            DescToken::Text("plain".to_string()),
            DescToken::Text("text".to_string()),
            DescToken::Text("task".to_string()),
        ]
    );
}

#[test]
fn skips_blank_lines() {
    let file = "(A) Buy milk\n\n\n(B) Pay rent\n";

    let list = TaskList::from_lines(file).expect("file should parse");

    assert_eq!(list.len(), 2);
}

#[test]
fn round_trips_through_lines() {
    let file = "(A) 2024-01-01 Buy milk +errands @shop\n(B) Pay rent @home";

    let list = TaskList::from_lines(file).expect("file should parse");

    assert_eq!(list.to_lines(), file);
}

#[test]
fn serializes_task_list_as_todo_txt_file() {
    let list = TaskList::from_lines("(A) Buy milk +errands @shop\n(B) Pay rent @home")
        .expect("file should parse");

    let json = serde_json::to_string(&list).expect("serialization should succeed");

    assert_eq!(
        json,
        "\"(A) Buy milk +errands @shop\\n(B) Pay rent @home\""
    );
}

#[test]
fn deserializes_task_list_from_todo_txt_file() {
    let json = "\"(A) Buy milk +errands @shop\\n(B) Pay rent @home\"";

    let list: TaskList = serde_json::from_str(json).expect("deserialization should succeed");

    assert_eq!(list.len(), 2);
    assert_eq!(list[0].priority, Some('A'));
    assert_eq!(list[1].priority, Some('B'));
}

#[test]
fn task_list_round_trips_through_serialize_and_deserialize() {
    let original = TaskList::from_lines(
        "x (C) 2024-06-01 2024-05-01 Finish report +work @office note:draft\n(D) Water plants",
    )
    .expect("file should parse");

    let json = serde_json::to_string(&original).expect("serialization should succeed");
    let restored: TaskList = serde_json::from_str(&json).expect("deserialization should succeed");

    assert_eq!(original, restored);
}
