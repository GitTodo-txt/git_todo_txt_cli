use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use chrono;

pub mod serializer;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DescToken {
    Text(String),
    Context(String),
    Project(String),
    Meta(String, String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub completed: bool,
    pub priority: Option<char>,
    pub completion_date: Option<chrono::NaiveDate>,
    pub creation_date: Option<chrono::NaiveDate>,
    pub description: Vec<DescToken>,
    pub meta: HashMap<String, String>,
}

impl Task {
    pub fn from_line(line: &str) -> Result<Self, String> {
        let mut completed = false;
        let mut priority = None;
        let mut completion_date = None;
        let mut creation_date = None;
        let mut description = Vec::new();
        let mut meta = HashMap::new();

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mut i = 0;

        if !tokens.is_empty() && tokens[0] == "x" {
            completed = true;
            i += 1;
        }

        if i < tokens.len() && tokens[i].starts_with('(') && tokens[i].ends_with(')') {
            priority = Some(tokens[i].chars().nth(1).unwrap());
            i += 1;
        }

        if i < tokens.len() {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(tokens[i], "%Y-%m-%d") {
                creation_date = Some(date);
                i += 1;
            }
        }

        if i < tokens.len() {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(tokens[i], "%Y-%m-%d") {
                completion_date = creation_date;
                creation_date = Some(date);
                i += 1;
            }
        }

        while i < tokens.len() {
            let token = tokens[i];
            if token.starts_with('+') {
                description.push(DescToken::Project(token[1..].to_string()));
            } else if token.starts_with('@') {
                description.push(DescToken::Context(token[1..].to_string()));
            } else if token.contains(':') {
                let parts: Vec<&str> = token.splitn(2, ':').collect();
                meta.insert(parts[0].to_string(), parts[1].to_string());
                description.push(DescToken::Meta(parts[0].to_string(), parts[1].to_string()));
            } else {
                description.push(DescToken::Text(token.to_string()));
            }
            i += 1;
        }

        Ok(Task {
            completed,
            priority,
            completion_date,
            creation_date,
            description,
            meta,
        })
    }

    pub fn to_line(&self) -> String {
        let mut line = String::new();

        if self.completed {
            line.push_str("x ");
        }

        if let Some(priority) = self.priority {
            line.push_str(&format!("({}) ", priority));
        }

        if let Some(completion_date) = self.completion_date {
            line.push_str(&format!("{} ", completion_date.format("%Y-%m-%d")));
            if self.creation_date.is_none() {
                line.push_str(&format!("{} ", completion_date.format("%Y-%m-%d")));
            }
        }

        if let Some(creation_date) = self.creation_date {
            line.push_str(&format!("{} ", creation_date.format("%Y-%m-%d")));
        }

        for token in &self.description {
            match token {
                DescToken::Text(text) => line.push_str(&format!("{} ", text)),
                DescToken::Context(context) => line.push_str(&format!("@{} ", context)),
                DescToken::Project(project) => line.push_str(&format!("+{} ", project)),
                DescToken::Meta(key, value) => line.push_str(&format!("{}:{} ", key, value)),
            }
        }

        line.trim_end().to_string()
    }
}

/// A collection of [`Task`]s, corresponding to the contents of a whole
/// todo.txt file (one task per line).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TaskList(pub Vec<Task>);

impl TaskList {
    /// Parses a whole todo.txt file's contents into a `TaskList`, one task
    /// per non-blank line.
    pub fn from_lines(input: &str) -> Result<Self, String> {
        let tasks = input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(Task::from_line)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TaskList(tasks))
    }

    /// Renders the list back into todo.txt file contents, one task per line.
    pub fn to_lines(&self) -> String {
        self.0
            .iter()
            .map(Task::to_line)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Deref for TaskList {
    type Target = Vec<Task>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TaskList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<Task> for TaskList {
    fn from_iter<I: IntoIterator<Item = Task>>(iter: I) -> Self {
        TaskList(iter.into_iter().collect())
    }
}

impl IntoIterator for TaskList {
    type Item = Task;
    type IntoIter = std::vec::IntoIter<Task>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a TaskList {
    type Item = &'a Task;
    type IntoIter = std::slice::Iter<'a, Task>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
