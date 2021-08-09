-- Your SQL goes here
CREATE TABLE `comments` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `content` varchar(1024) NOT NULL,
  `reply_to` int(11) DEFAULT NULL,
  `likes` int(11) NOT NULL DEFAULT 0,
  `level` int(11) NOT NULL DEFAULT 0,
  `has_replies` tinyint(4) NOT NULL DEFAULT 0,
  `deleted` tinyint(4) NOT NULL DEFAULT 0,
  `fk_user_id` int(11) NOT NULL,
  `fk_post_id` int(11) NOT NULL,
  `created_at` timestamp(6) NOT NULL DEFAULT current_timestamp(6),
  `updated_at` datetime(6) NOT NULL DEFAULT current_timestamp(6) ON UPDATE current_timestamp(6),
  PRIMARY KEY (`id`),
  KEY `IDX_9611a099501597c519429f2595` (`created_at`),
  KEY `FK_9e76c01ba499ac561a60d5ebe2c` (`fk_user_id`),
  KEY `FK_ba07ef283dbd783ef1936eec0bc` (`fk_post_id`),
  CONSTRAINT `FK_9e76c01ba499ac561a60d5ebe2c` FOREIGN KEY (`fk_user_id`) REFERENCES `users` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION,
  CONSTRAINT `FK_ba07ef283dbd783ef1936eec0bc` FOREIGN KEY (`fk_post_id`) REFERENCES `posts` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;