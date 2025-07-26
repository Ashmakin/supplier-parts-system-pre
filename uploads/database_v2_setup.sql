-- 清理并创建数据库
DROP DATABASE IF EXISTS `sccp_db`; -- SCCP: Supply Chain Collaboration Platform
CREATE DATABASE `sccp_db` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE `sccp_db`;

-- 公司表
CREATE TABLE `companies` (
                             `id` INT AUTO_INCREMENT PRIMARY KEY,
                             `name` VARCHAR(255) NOT NULL UNIQUE,
                             `company_type` ENUM('BUYER', 'SUPPLIER') NOT NULL,
                             `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB;

-- 用户表
CREATE TABLE `users` (
                         `id` INT AUTO_INCREMENT PRIMARY KEY,
                         `company_id` INT NOT NULL,
                         `email` VARCHAR(255) NOT NULL UNIQUE,
                         `password_hash` VARCHAR(255) NOT NULL,
                         `full_name` VARCHAR(255),
                         `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                         FOREIGN KEY (`company_id`) REFERENCES `companies`(`id`) ON DELETE CASCADE
) ENGINE=InnoDB;

-- 询价单 (RFQ) 表
CREATE TABLE `rfqs` (
                        `id` INT AUTO_INCREMENT PRIMARY KEY,
                        `buyer_company_id` INT NOT NULL,
                        `title` VARCHAR(255) NOT NULL,
                        `description` TEXT,
                        `quantity` INT NOT NULL,
                        `status` ENUM('OPEN', 'CLOSED', 'AWARDED') NOT NULL DEFAULT 'OPEN',
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                        FOREIGN KEY (`buyer_company_id`) REFERENCES `companies`(`id`) ON DELETE CASCADE
) ENGINE=InnoDB;

-- 报价单 (Quote) 表
CREATE TABLE `quotes` (
                          `id` INT AUTO_INCREMENT PRIMARY KEY,
                          `rfq_id` INT NOT NULL,
                          `supplier_company_id` INT NOT NULL,
                          `price` DECIMAL(12, 2) NOT NULL,
                          `lead_time_days` INT NOT NULL,
                          `notes` TEXT,
                          `status` ENUM('SUBMITTED', 'ACCEPTED', 'REJECTED') NOT NULL DEFAULT 'SUBMITTED',
                          `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                          FOREIGN KEY (`rfq_id`) REFERENCES `rfqs`(`id`) ON DELETE CASCADE,
                          FOREIGN KEY (`supplier_company_id`) REFERENCES `companies`(`id`) ON DELETE CASCADE
) ENGINE=InnoDB;

-- 采购订单 (PO) 表
CREATE TABLE `purchase_orders` (
                                   `id` INT AUTO_INCREMENT PRIMARY KEY,
                                   `quote_id` INT NOT NULL UNIQUE, -- 一个报价只能生成一个PO
                                   `rfq_id` INT NOT NULL,
                                   `buyer_company_id` INT NOT NULL,
                                   `supplier_company_id` INT NOT NULL,
                                   `total_amount` DECIMAL(12, 2) NOT NULL,
                                   `status` ENUM('PENDING_CONFIRMATION', 'IN_PRODUCTION', 'SHIPPED', 'COMPLETED') NOT NULL DEFAULT 'PENDING_CONFIRMATION',
                                   `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                   FOREIGN KEY (`quote_id`) REFERENCES `quotes`(`id`),
                                   FOREIGN KEY (`rfq_id`) REFERENCES `rfqs`(`id`),
                                   FOREIGN KEY (`buyer_company_id`) REFERENCES `companies`(`id`),
                                   FOREIGN KEY (`supplier_company_id`) REFERENCES `companies`(`id`)
) ENGINE=InnoDB;

-- 插入一些初始数据方便测试
INSERT INTO `companies` (name, company_type) VALUES
                                                 ('未来科技 (采购方)', 'BUYER'),
                                                 ('精密制造 (供应商)', 'SUPPLIER');

-- 密码是 'password123' (经过哈希处理)
INSERT INTO `users` (company_id, email, password_hash, full_name) VALUES
                                                                      (1, 'buyer@future-tech.com', '$2b$12$ADfV3T7A1QGhoroYJ1yY8u43gGvSg7vT4D.BCM/uVIpYk2i.zYtS.', '采购经理-李'),
                                                                      (2, 'supplier@precision-mfg.com', '$2b$12$ADfV3T7A1QGhoroYJ1yY8u43gGvSg7vT4D.BCM/uVIpYk2i.zYtS.', '销售代表-王');