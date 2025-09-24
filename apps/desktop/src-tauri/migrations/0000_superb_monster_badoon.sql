CREATE TABLE `courses` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`year` integer NOT NULL,
	`slug` text NOT NULL,
	`name` text DEFAULT '' NOT NULL,
	`sort_index` integer NOT NULL,
	`created_at` integer DEFAULT (unixepoch()) NOT NULL,
	`updated_at` integer DEFAULT (unixepoch()) NOT NULL
);
--> statement-breakpoint
CREATE INDEX `idx_courses_year` ON `courses` (`year`);--> statement-breakpoint
CREATE UNIQUE INDEX `uq_courses_year_slug` ON `courses` (`year`,`slug`);--> statement-breakpoint
CREATE TABLE `lectures` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`course_id` integer NOT NULL,
	`slug` text NOT NULL,
	`name` text DEFAULT '' NOT NULL,
	`sort_index` integer NOT NULL,
	`created_at` integer DEFAULT (unixepoch()) NOT NULL,
	`updated_at` integer DEFAULT (unixepoch()) NOT NULL,
	FOREIGN KEY (`course_id`) REFERENCES `courses`(`id`) ON UPDATE no action ON DELETE cascade
);
--> statement-breakpoint
CREATE INDEX `idx_lectures_course` ON `lectures` (`course_id`);--> statement-breakpoint
CREATE UNIQUE INDEX `uq_lectures_course_slug` ON `lectures` (`course_id`,`slug`);--> statement-breakpoint
CREATE TABLE `pages` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`lecture_id` integer NOT NULL,
	`slug` text NOT NULL,
	`name` text DEFAULT '' NOT NULL,
	`sort_index` integer NOT NULL,
	`key` text NOT NULL,
	`created_at` integer DEFAULT (unixepoch()) NOT NULL,
	`updated_at` integer DEFAULT (unixepoch()) NOT NULL,
	FOREIGN KEY (`lecture_id`) REFERENCES `lectures`(`id`) ON UPDATE no action ON DELETE cascade
);
--> statement-breakpoint
CREATE INDEX `idx_pages_lecture` ON `pages` (`lecture_id`);--> statement-breakpoint
CREATE UNIQUE INDEX `uq_pages_lecture_slug` ON `pages` (`lecture_id`,`slug`);--> statement-breakpoint
CREATE UNIQUE INDEX `uq_pages_key` ON `pages` (`key`);--> statement-breakpoint
CREATE TABLE `slides` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`page_id` integer NOT NULL,
	`idx` integer NOT NULL,
	`url` text NOT NULL,
	`pdf_path` text,
	`downloaded_at` integer DEFAULT (unixepoch()) NOT NULL,
	FOREIGN KEY (`page_id`) REFERENCES `pages`(`id`) ON UPDATE no action ON DELETE cascade
);
--> statement-breakpoint
CREATE INDEX `idx_slides_page` ON `slides` (`page_id`);--> statement-breakpoint
CREATE UNIQUE INDEX `uq_slides_page_idx` ON `slides` (`page_id`,`idx`);