-- Your SQL goes here
CREATE TABLE `posts` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `title` varchar(255) NOT NULL,
  `author` varchar(255) NOT NULL,
  `type` int(11) NOT NULL,
  `content` varchar(4096) NOT NULL,
  `description` varchar(255) NOT NULL,
  `created_at` timestamp(6) NOT NULL DEFAULT current_timestamp(6),
  `fk_user_id` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `IDX_0f8f2f0c0512fbecbe3034b804` (`created_at`),
  KEY `FK_6b635a273ddcde918c7711e51d1` (`fk_user_id`),
  CONSTRAINT `FK_6b635a273ddcde918c7711e51d1` FOREIGN KEY (`fk_user_id`) REFERENCES `users` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;