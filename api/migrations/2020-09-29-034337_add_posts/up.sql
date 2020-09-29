-- Your SQL goes here

CREATE TABLE posts
(
  id SERIAL NOT NULL PRIMARY KEY,
  title TEXT NOT NULL,
  author TEXT NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);