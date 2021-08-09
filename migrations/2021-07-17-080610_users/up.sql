-- Your SQL goes here

CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `email` varchar(255) NOT NULL,
  `username` varchar(16) DEFAULT NULL,
  `display_name` varchar(48) NOT NULL,
  `photo_url` varchar(255) DEFAULT NULL,
  `created_at` timestamp(6) NOT NULL DEFAULT current_timestamp(6),
  `is_certified` tinyint(4) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `IDX_97672ac88f789774dd47f7c8be` (`email`),
  KEY `IDX_fe0bb3f6520ee0469504521e71` (`username`),
  KEY `IDX_c9b5b525a96ddc2c5647d7f7fa` (`created_at`),
  KEY `IDX_f9bfbc6829d5ebf556c7260fb1` (`is_certified`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4;