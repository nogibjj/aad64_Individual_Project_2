-- Create Iris table
CREATE TABLE IF NOT EXISTS Iris (
    sepal_length REAL,
    sepal_width REAL,
    petal_length REAL,
    petal_width REAL,
    class TEXT
);

-- See 5 rows 
SELECT * FROM Iris LIMIT 5;

-- Insert initial data
INSERT INTO Iris (sepal_length, sepal_width, petal_length, petal_width, class) VALUES (7.5, 7.5, 7.5, 7.5, 'Test');

-- Update Iris table
INSERT INTO Iris (sepal_length, sepal_width, petal_length, petal_width, class) VALUES (7.5, 7.5, 7.5, 7.5, 'Test');
UPDATE Iris SET sepal_length = 7.5 WHERE sepal_width = 7.5;

-- Order Iris table
SELECT * FROM Iris ORDER BY sepal_length DESC;
