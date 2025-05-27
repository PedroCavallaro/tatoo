-- Your SQL goes here

CREATE TABLE `place`(
	`id` BIGINT NOT NULL PRIMARY KEY,
	`name` VARCHAR(255) NOT NULL,
	`picture` VARCHAR(255) NOT NULL,
	`sub` VARCHAR(255) NOT NULL,
	`email` VARCHAR(255) NOT NULL,
	`whatsapp_number` BIGINT,
	`created_at` TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP NOT NULL
);

