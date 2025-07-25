use serde::Serialize;

use crate::todo::sql::Todo;

/// Represents a hierarchical group of todos, with a main todo and its sub-todos.
#[derive(Debug, Serialize)]
pub struct TodoGroup {
    /// The main todo item that acts as the parent.
    pub main_todo: Todo,
    /// A vector of sub-todos associated with the main todo.
    pub subtodos: Vec<Todo>,
}

impl TodoGroup {
    /// Creates a new `TodoGroup` with a given main todo.
    pub fn new(main_todo: Todo) -> Self {
        Self {
            main_todo,
            subtodos: Vec::new(),
        }
    }

    /// Adds a sub-todo to this group.
    pub fn add_subtodo(&mut self, subtodo: Todo) {
        self.subtodos.push(subtodo);
    }
}

/// Organizes a flat list of `Todo` items into a hierarchical structure of `TodoGroup`s.
///
/// Main todos are sorted by priority (descending) and then by status. Sub-todos within
/// each group are also sorted similarly.
pub fn organize_todos_hierarchically(todos: Vec<Todo>) -> Vec<TodoGroup> {
    use std::collections::HashMap;

    let mut main_todos = Vec::new();
    let mut subtodos_map: HashMap<i32, Vec<Todo>> = HashMap::new();

    // Separate main todos from subtodos
    for todo in todos {
        if todo.parent_id.is_none() {
            main_todos.push(todo);
        } else if let Some(parent_id) = todo.parent_id {
            subtodos_map
                .entry(parent_id)
                .or_insert_with(Vec::new)
                .push(todo);
        }
    }

    // Sort main todos by priority (higher first) and then by status
    main_todos.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then_with(|| a.status.cmp(&b.status))
    });

    // Sort subtodos within each group
    for subtodos in subtodos_map.values_mut() {
        subtodos.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.status.cmp(&b.status))
        });
    }

    // Create TodoGroups
    let mut todo_groups = Vec::new();
    for main_todo in main_todos {
        let mut group = TodoGroup::new(main_todo);

        // Add subtodos if they exist (already sorted)
        if let Some(subtodos) = subtodos_map.remove(&group.main_todo.id) {
            for subtodo in subtodos {
                group.add_subtodo(subtodo);
            }
        }

        todo_groups.push(group);
    }

    todo_groups
}

/// Represents a `Todo` item within a flattened hierarchical list.
///
/// This structure is used to represent a tree of todos in a flat list,
/// while preserving information about depth and relationships for display purposes.
#[derive(Debug, Clone, Serialize)]
pub struct FlatTodo {
    /// The underlying `Todo` item.
    pub todo: Todo,
    /// The depth of the todo in the hierarchy (0 for main todos, 1 for their children, etc.).
    pub depth: usize,
    /// A flag indicating if this is a main todo (i.e., has no parent).
    pub is_main_todo: bool,
    /// A flag indicating if this todo has any sub-todos.
    pub has_subtodos: bool,
}

impl FlatTodo {
    /// Creates a new `FlatTodo` representing a main todo item.
    pub fn new_main(todo: Todo, has_subtodos: bool) -> Self {
        Self {
            todo,
            depth: 0,
            is_main_todo: true,
            has_subtodos,
        }
    }

    /// Creates a new `FlatTodo` representing a sub-todo item.
    pub fn new_sub(todo: Todo, depth: usize) -> Self {
        Self {
            todo,
            depth,
            is_main_todo: false,
            has_subtodos: false,
        }
    }
}

/// Organizes a flat list of `Todo` items into a single, flattened list that
/// preserves hierarchical information for display.
///
/// The list is sorted by main todo priority, with sub-todos listed directly
/// after their parents.
pub fn organize_todos_flat_hierarchically(todos: Vec<Todo>) -> Vec<FlatTodo> {
    use std::collections::HashMap;

    let mut main_todos = Vec::new();
    let mut subtodos_map: HashMap<i32, Vec<Todo>> = HashMap::new();

    // Separate main todos from subtodos
    for todo in todos {
        if todo.parent_id.is_none() {
            main_todos.push(todo);
        } else if let Some(parent_id) = todo.parent_id {
            subtodos_map
                .entry(parent_id)
                .or_insert_with(Vec::new)
                .push(todo);
        }
    }

    // Sort main todos by priority (higher first) and then by status
    main_todos.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then_with(|| a.status.cmp(&b.status))
    });

    // Sort subtodos within each group
    for subtodos in subtodos_map.values_mut() {
        subtodos.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.status.cmp(&b.status))
        });
    }

    // Create flat list with hierarchy information
    let mut flat_todos = Vec::new();
    for main_todo in main_todos {
        let has_subtodos = subtodos_map.contains_key(&main_todo.id);

        // Add main todo
        flat_todos.push(FlatTodo::new_main(main_todo.clone(), has_subtodos));

        // Add subtodos if they exist (already sorted)
        if let Some(subtodos) = subtodos_map.remove(&main_todo.id) {
            for subtodo in subtodos {
                flat_todos.push(FlatTodo::new_sub(subtodo, 1));
            }
        }
    }

    flat_todos
}

#[cfg(test)]
mod tests {
    use crate::todo::{
        group::organize_todos_hierarchically,
        sql::{Todo, TodoPriority, TodoStatus},
    };

    fn create_test_todo(
        id: i32,
        parent_id: Option<i32>,
        title: &str,
        priority: TodoPriority,
    ) -> Todo {
        Todo {
            id,
            project_id: 1,
            parent_id,
            title: title.to_string(),
            description: None,
            status: TodoStatus::ToDo,
            priority,
            due_date: None,
            estimated_minutes: None,
            location: None,
            url: None,
            created_at: None,
            updated_at: None,
            completed_at: None,
        }
    }

    #[test]
    fn test_organize_todos_hierarchically() {
        // Create test data
        let todos = vec![
            create_test_todo(1, None, "Main Task 1", TodoPriority::High),
            create_test_todo(2, Some(1), "Subtask 1.1", TodoPriority::Medium),
            create_test_todo(3, Some(1), "Subtask 1.2", TodoPriority::Low),
            create_test_todo(4, None, "Main Task 2", TodoPriority::Medium),
            create_test_todo(5, Some(4), "Subtask 2.1", TodoPriority::High),
            create_test_todo(6, None, "Main Task 3", TodoPriority::Low),
        ];

        let result = organize_todos_hierarchically(todos);

        // Should have 3 main todo groups
        assert_eq!(result.len(), 3);

        // Check first group (Main Task 1 - High priority should be first)
        assert_eq!(result[0].main_todo.title, "Main Task 1");
        assert_eq!(result[0].subtodos.len(), 2);
        assert_eq!(result[0].subtodos[0].title, "Subtask 1.1"); // Medium priority first
        assert_eq!(result[0].subtodos[1].title, "Subtask 1.2"); // Low priority second

        // Check second group (Main Task 2 - Medium priority)
        assert_eq!(result[1].main_todo.title, "Main Task 2");
        assert_eq!(result[1].subtodos.len(), 1);
        assert_eq!(result[1].subtodos[0].title, "Subtask 2.1");

        // Check third group (Main Task 3 - Low priority should be last)
        assert_eq!(result[2].main_todo.title, "Main Task 3");
        assert_eq!(result[2].subtodos.len(), 0);
    }

    #[test]
    fn test_organize_todos_hierarchically_empty() {
        let todos = vec![];
        let result = organize_todos_hierarchically(todos);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_organize_todos_hierarchically_only_main_todos() {
        let todos = vec![
            create_test_todo(1, None, "Main Task 1", TodoPriority::High),
            create_test_todo(2, None, "Main Task 2", TodoPriority::Low),
        ];

        let result = organize_todos_hierarchically(todos);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].main_todo.title, "Main Task 1"); // High priority first
        assert_eq!(result[0].subtodos.len(), 0);
        assert_eq!(result[1].main_todo.title, "Main Task 2"); // Low priority second
        assert_eq!(result[1].subtodos.len(), 0);
    }
}