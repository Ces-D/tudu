-- -------------------------------------------------
-- Table: projects
-- Stores project metadata to group related todos
-- -------------------------------------------------
CREATE TABLE projects (
    id           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,     -- Unique project identifier
    name         TEXT    NOT NULL,                              -- Project name (required)
    description  TEXT,                                          -- Detailed description or notes
    color        TEXT,                                          -- Optional hex color code for UI
    created_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,            -- When the project was created
    updated_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP             -- When the project was last updated
);

-- -------------------------------------------------
-- Table: todos
-- Stores individual tasks, linked to projects
-- Supports subtasks, prioritization, time tracking, and metadata
-- -------------------------------------------------
CREATE TABLE todos (
    id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,  -- Unique task identifier
    project_id        INTEGER NOT NULL,                            -- References projects(id), groups this task
    parent_id         INTEGER,                                     -- References todos(id), allows nested subtasks

    title             TEXT    NOT NULL,                            -- Short descriptive title for the task
    description       TEXT,                                        -- Detailed notes or acceptance criteria
    status            INTEGER NOT NULL,                            -- Current status code (e.g. 0: pending, 1: in_progress, 2: done, 3: cancelled)
    priority          INTEGER NOT NULL,                            -- Priority level; higher = more urgent
    due_date          TIMESTAMP,                                        -- Deadline date for the task
    estimated_minutes INTEGER,                                     -- Estimated time to complete (minutes)
    location          TEXT,                                        -- Context or location for the task (e.g. 'Office', 'Groceries')
    url               TEXT,                                        -- Link to related resource (spec, ticket, doc)
    created_at        TIMESTAMP DEFAULT CURRENT_TIMESTAMP,          -- When the task was created
    updated_at        TIMESTAMP DEFAULT CURRENT_TIMESTAMP,          -- When the task was last updated
    completed_at      TIMESTAMP,                                    -- When the task was marked complete

    -- Foreign key constraints
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id)  REFERENCES todos(id)    ON DELETE CASCADE
);
