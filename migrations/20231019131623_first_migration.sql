CREATE TABLE
    IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        email VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        password VARCHAR(255) NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    IF NOT EXISTS tasks (
        id SERIAL PRIMARY KEY,
        title VARCHAR(50),
        description TEXT,
        start_at TIMESTAMP,
        end_at TIMESTAMP,
        priority VARCHAR(255),
        user_id UUID,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );