CREATE TABLE employees (
       id SERIAL PRIMARY KEY,
       first_name VARCHAR(255) NOT NULL,
       last_name VARCHAR(255) NOT NULL,
       middle_name VARCHAR(255),
       department_id INTEGER REFERENCES departments(id),
       position_id INTEGER REFERENCES positions(id),
       hire_date DATE NOT NULL DEFAULT NOW(),
       termination_date DATE,
       ad_login VARCHAR(255) UNIQUE,
       email VARCHAR(255) UNIQUE,
       status VARCHAR(50) NOT NULL DEFAULT 'работает',
       phone VARCHAR(20)
);