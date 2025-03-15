pub const LIST_TASKS_ACTIVE: &str = r#"
        SELECT 
        context.id,
        context.name,
        context.active,
        COALESCE(json_agg(
            json_build_object(
                'id', task.id, 
                'content', task.content, 
                'done', task.done, 
                'creation_date', task.creation_date, 
                'modification_date', task.modification_date) ORDER BY task.id ASC
            ) FILTER (WHERE task.id IS NOT NULL), '[]') AS tasks
        FROM context
        LEFT JOIN task
        ON task.context_id = context.id
        WHERE context.active = true
        GROUP BY context.id
        ORDER BY context.id ASC;
        "#;

pub const LIST_TASKS: &str = r#"
        SELECT 
        context.id,
        context.name,
        context.active,
        COALESCE(json_agg(
            json_build_object(
                'id', task.id, 
                'content', task.content, 
                'done', task.done, 
                'creation_date', task.creation_date, 
                'modification_date', task.modification_date) ORDER BY task.id ASC
            ) FILTER (WHERE task.id IS NOT NULL), '[]') AS tasks
        FROM context
        LEFT JOIN task
        ON task.context_id = context.id
        GROUP BY context.id
        ORDER BY context.id ASC;
        "#;
