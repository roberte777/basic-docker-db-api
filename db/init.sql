--create a people table and fill it with Game of Thrones characters
create table people (
    id serial primary key,
    name text,
    age integer,
    alive boolean
);
--file it with 5 Game of Thrones characters
INSERT INTO people (name, age, alive)
VALUES ('Jon Snow', 18, true),
       ('Daenerys Targaryen', 18, true),
       ('Tyrion Lannister', 40, true),
       ('Cersei Lannister', 40, true),
       ('Arya Stark', 14, true);
