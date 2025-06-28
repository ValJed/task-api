pub const LIST_TASKS_ACTIVE: &str = r#"
        WITH ordered_tasks AS (
            SELECT * FROM task ORDER BY id ASC
        )
        SELECT 
        context.id,
        context.name,
        context.active,
        COALESCE(
            json_group_array(
                json_object(
                    'id', task.id,
                    'content', task.content,
                    'done', task.done,
                    'creation_date', task.creation_date,
                    'modification_date', task.modification_date,
                    'due_date', task.due_date
                )
            ) FILTER (WHERE task.id IS NOT NULL), json_array()) AS tasks
        FROM context
        LEFT JOIN ordered_tasks task ON task.context_id = context.id        
        WHERE context.active = 1
        GROUP BY context.id, context.name, context.active
        ORDER BY context.id ASC, task.id ASC
        "#;

pub const LIST_TASKS: &str = r#"
        WITH ordered_tasks AS (
            SELECT * FROM task ORDER BY id ASC
        )
        SELECT 
        context.id,
        context.name,
        context.active,
        COALESCE(
            json_group_array(
                json_object(
                    'id', task.id,
                    'content', task.content,
                    'done', task.done,
                    'creation_date', task.creation_date,
                    'modification_date', task.modification_date,
                    'due_date', task.due_date
                )
            ) FILTER (WHERE task.id IS NOT NULL), json_array()) AS tasks
        FROM context
        LEFT JOIN ordered_tasks task ON task.context_id = context.id
        GROUP BY context.id, context.name, context.active
        ORDER BY context.id ASC, task.id ASC
        "#;
