-- Your SQL goes here
CREATE TABLE `likes` (
  `post_id` int(11) NOT NULL,
  `user_id` int(11) NOT NULL,
  PRIMARY KEY (`post_id`,`user_id`),
  KEY `IDX_e2fe567ad8d305fefc918d44f5` (`post_id`),
  KEY `IDX_d5312be6de5784293ac2908978` (`user_id`),
  CONSTRAINT `FK_d5312be6de5784293ac29089784` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION,
  CONSTRAINT `FK_e2fe567ad8d305fefc918d44f50` FOREIGN KEY (`post_id`) REFERENCES `posts` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;