-- Your SQL goes here


CREATE TABLE `appointment`(
	`date` DATE NOT NULL,
	`user_id` BIGINT NOT NULL,
	`place_id` BIGINT NOT NULL,
	PRIMARY KEY(`user_id`, `place_id`),
	FOREIGN KEY (`user_id`) REFERENCES `user`(`id`),
	FOREIGN KEY (`place_id`) REFERENCES `place`(`id`)
);

