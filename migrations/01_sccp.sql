SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for chat_messages
-- ----------------------------
DROP TABLE IF EXISTS `chat_messages`;
CREATE TABLE `chat_messages` (
  `id` int NOT NULL AUTO_INCREMENT,
  `rfq_id` int NOT NULL,
  `user_id` int NOT NULL,
  `user_full_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `company_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `message_text` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `rfq_id` (`rfq_id`),
  KEY `user_id` (`user_id`),
  CONSTRAINT `chat_messages_ibfk_1` FOREIGN KEY (`rfq_id`) REFERENCES `rfqs` (`id`) ON DELETE CASCADE,
  CONSTRAINT `chat_messages_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=16 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ----------------------------
-- Table structure for companies
-- ----------------------------
DROP TABLE IF EXISTS `companies`;
CREATE TABLE `companies` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `company_type` enum('BUYER','SUPPLIER') COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ----------------------------
-- Table structure for purchase_orders
-- ----------------------------
DROP TABLE IF EXISTS `purchase_orders`;
CREATE TABLE `purchase_orders` (
  `id` int NOT NULL AUTO_INCREMENT,
  `quote_id` int NOT NULL,
  `rfq_id` int NOT NULL,
  `buyer_company_id` int NOT NULL,
  `supplier_company_id` int NOT NULL,
  `total_amount` decimal(12,2) NOT NULL,
  `status` enum('PENDING_CONFIRMATION','IN_PRODUCTION','SHIPPED','COMPLETED') COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'PENDING_CONFIRMATION',
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `quote_id` (`quote_id`),
  KEY `rfq_id` (`rfq_id`),
  KEY `buyer_company_id` (`buyer_company_id`),
  KEY `supplier_company_id` (`supplier_company_id`),
  CONSTRAINT `purchase_orders_ibfk_1` FOREIGN KEY (`quote_id`) REFERENCES `quotes` (`id`),
  CONSTRAINT `purchase_orders_ibfk_2` FOREIGN KEY (`rfq_id`) REFERENCES `rfqs` (`id`),
  CONSTRAINT `purchase_orders_ibfk_3` FOREIGN KEY (`buyer_company_id`) REFERENCES `companies` (`id`),
  CONSTRAINT `purchase_orders_ibfk_4` FOREIGN KEY (`supplier_company_id`) REFERENCES `companies` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ----------------------------
-- Table structure for quotes
-- ----------------------------
DROP TABLE IF EXISTS `quotes`;
CREATE TABLE `quotes` (
  `id` int NOT NULL AUTO_INCREMENT,
  `rfq_id` int NOT NULL,
  `supplier_company_id` int NOT NULL,
  `price` decimal(12,2) NOT NULL,
  `lead_time_days` int NOT NULL,
  `notes` text COLLATE utf8mb4_unicode_ci,
  `status` enum('SUBMITTED','ACCEPTED','REJECTED') COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'SUBMITTED',
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `rfq_id` (`rfq_id`),
  KEY `supplier_company_id` (`supplier_company_id`),
  CONSTRAINT `quotes_ibfk_1` FOREIGN KEY (`rfq_id`) REFERENCES `rfqs` (`id`) ON DELETE CASCADE,
  CONSTRAINT `quotes_ibfk_2` FOREIGN KEY (`supplier_company_id`) REFERENCES `companies` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ----------------------------
-- Table structure for rfq_attachments
-- ----------------------------
DROP TABLE IF EXISTS `rfq_attachments`;
CREATE TABLE `rfq_attachments` (
  `id` int NOT NULL AUTO_INCREMENT,
  `rfq_id` int NOT NULL,
  `original_filename` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `stored_path` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `rfq_id` (`rfq_id`),
  CONSTRAINT `rfq_attachments_ibfk_1` FOREIGN KEY (`rfq_id`) REFERENCES `rfqs` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ----------------------------
-- Table structure for rfqs
-- ----------------------------
DROP TABLE IF EXISTS `rfqs`;
CREATE TABLE `rfqs` (
  `id` int NOT NULL AUTO_INCREMENT,
  `buyer_company_id` int NOT NULL,
  `title` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci,
  `quantity` int NOT NULL,
  `status` enum('OPEN','CLOSED','AWARDED') COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'OPEN',
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `buyer_company_id` (`buyer_company_id`),
  CONSTRAINT `rfqs_ibfk_1` FOREIGN KEY (`buyer_company_id`) REFERENCES `companies` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ----------------------------
-- Table structure for users
-- ----------------------------
DROP TABLE IF EXISTS `users`;
CREATE TABLE `users` (
  `id` int NOT NULL AUTO_INCREMENT,
  `company_id` int NOT NULL,
  `email` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `password_hash` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `full_name` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `email` (`email`),
  KEY `company_id` (`company_id`),
  CONSTRAINT `users_ibfk_1` FOREIGN KEY (`company_id`) REFERENCES `companies` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

SET FOREIGN_KEY_CHECKS = 1;
