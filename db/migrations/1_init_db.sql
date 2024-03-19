CREATE TABLE IF NOT EXISTS author (
    id INT PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    email VARCHAR(20) NOT NULL
);

CREATE TABLE IF NOT EXISTS article (
    id INT PRIMARY KEY,
    description VARCHAR(100) NOT NULL, 
    content VARCHAR(500) NOT NULL,
    user_id INT not null
);

