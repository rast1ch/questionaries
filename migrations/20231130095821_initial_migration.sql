-- Add migration script here
CREATE TABLE questionaire (
    id serial PRIMARY KEY NOT NULL,
    title varchar(2000) NOT NULL,
    description varchar(2000) NOT NULL,
    published boolean NOT NULL DEFAULT false
);

CREATE TABLE question (
    id serial PRIMARY KEY NOT NULL,
    question varchar(2000) NOT NULL,
    questionaire_id integer NOT NULL,
    FOREIGN KEY (questionaire_id) REFERENCES questionaire (id)
);


CREATE TABLE answer (
    id serial PRIMARY KEY NOT NULL,
    answer varchar(2000) NOT NULL,
    question_id integer NOT NULL,
    FOREIGN KEY (question_id) REFERENCES question (id)
);

CREATE TABLE user_answer (
    id serial PRIMARY KEY NOT NULL,
    answer_id integer NOT NULL,
    email varchar(2000) NOT NULL,
    FOREIGN KEY (answer_id) REFERENCES answer (id),
    UNIQUE (email)
);
