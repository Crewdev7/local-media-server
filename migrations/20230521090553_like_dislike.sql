-- Add migration script here

CREATE TABLE likes(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    movie_id INTEGER,
    created TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id,movie_id)
);

-- Create trigger for cascading deletion from users table
CREATE TRIGGER IF NOT EXISTS delete_liked_movies
AFTER DELETE ON users
FOR EACH ROW
BEGIN
    DELETE FROM likes WHERE user_id = OLD.id;
END;

-- Create trigger for cascading deletion from movies table
CREATE TRIGGER IF NOT EXISTS delete_liked_movies_movies
AFTER DELETE ON movies
FOR EACH ROW
BEGIN
    DELETE FROM likes WHERE movie_id = OLD.id;
END;


-- Insert dummy data into movies table
INSERT INTO movies (title, genre, release_year) VALUES
    ('Movie A', 'Action', 2010),
    ('Movie B', 'Drama', 2005),
    ('Movie C', 'Comedy', 2018),
    ('Movie D', 'Thriller', 2012),
    ('Movie E', 'Science Fiction', 2016);

-- Insert dummy data into liked_movies table
INSERT INTO likes (user_id, movie_id) VALUES
    (1, 1),
    (1, 2),
    (2, 3),
    (2, 4),
    (3, 1),
    (3, 3),
    (4, 2),
    (5, 1),
    (5, 4);

