-- Your SQL goes here
CREATE TABLE `social_accounts` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `provider` varchar(12) NOT NULL,
  `social_id` varchar(255) NOT NULL,
  `user_id` int(11) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `REL_05a0f282d3bed93ca048a7e54d` (`user_id`),
  KEY `IDX_f02f396a5b4446bcc4a055095e` (`provider`,`social_id`),
  CONSTRAINT `FK_05a0f282d3bed93ca048a7e54dd` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4;