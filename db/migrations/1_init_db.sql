CREATE TABLE IF NOT EXISTS user (
    id INT PRIMARY KEY,
    name VARCHAR(20) not null,
    email VARCHAR(20) not null
);

CREATE TABLE IF NOT EXISTS article (
    id INT PRIMARY KEY,
    description VARCHAR(100) not null, 
    content VARCHAR(500) not null,
    user_id INT not null
);

