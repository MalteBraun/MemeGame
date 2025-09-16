-- Your SQL goes here
CREATE TABLE user (
                      id TEXT PRIMARY KEY NOT NULL ,
                      user_name TEXT NOT NULL ,
                      points_game TEXT NOT NULL,
                      points_total TEXT '='
);

INSERT INTO user VALUES ('1', 'user1', '0','0');
INSERT INTO user VALUES ('2', 'user2', 'O', '0');