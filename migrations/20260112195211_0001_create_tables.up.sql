CREATE TABLE Exchanges(
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL
);

CREATE TABLE Symbols(
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    exchange_id INT NOT NULL,
    FOREIGN KEY (exchange_id) REFERENCES Exchanges(id)
);

CREATE TABLE Indicators(
    id SERIAL PRIMARY KEY,
    indicator_type VARCHAR(255) NOT NULL,
    indicator_value INT NOT NULL,
    symbol_id INT NOT NULL,
    FOREIGN KEY (symbol_id) REFERENCES Symbols(id)
);