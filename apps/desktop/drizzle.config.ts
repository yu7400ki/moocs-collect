import { defineConfig } from "drizzle-kit";
import { sql } from "drizzle-orm";
import {
  index,
  integer,
  sqliteTable,
  text,
  uniqueIndex,
} from "drizzle-orm/sqlite-core";

// courses
export const courses = sqliteTable(
  "courses",
  {
    id: integer().primaryKey({ autoIncrement: true }),
    year: integer().notNull(),
    slug: text().notNull(),
    name: text().notNull().default(""),
    sortIndex: integer("sort_index").notNull(),
    createdAt: integer("created_at", { mode: "timestamp" })
      .notNull()
      .default(sql`(unixepoch())`),
    updatedAt: integer("updated_at", { mode: "timestamp" })
      .notNull()
      .default(sql`(unixepoch())`),
  },
  (table) => [
    index("idx_courses_year").on(table.year),
    uniqueIndex("uq_courses_year_slug").on(table.year, table.slug),
  ],
);

// lectures
export const lectures = sqliteTable(
  "lectures",
  {
    id: integer().primaryKey({ autoIncrement: true }),
    courseId: integer("course_id")
      .notNull()
      .references(() => courses.id, { onDelete: "cascade" }),
    slug: text().notNull(),
    name: text().notNull().default(""),
    sortIndex: integer("sort_index").notNull(),
    createdAt: integer("created_at", { mode: "timestamp" })
      .notNull()
      .default(sql`(unixepoch())`),
    updatedAt: integer("updated_at", { mode: "timestamp" })
      .notNull()
      .default(sql`(unixepoch())`),
  },
  (table) => [
    index("idx_lectures_course").on(table.courseId),
    uniqueIndex("uq_lectures_course_slug").on(table.courseId, table.slug),
  ],
);

// pages
export const pages = sqliteTable(
  "pages",
  {
    id: integer().primaryKey({ autoIncrement: true }),
    lectureId: integer("lecture_id")
      .notNull()
      .references(() => lectures.id, { onDelete: "cascade" }),
    slug: text().notNull(),
    name: text().notNull().default(""),
    sortIndex: integer("sort_index").notNull(),
    key: text().notNull(), // e.g. "2023/course/lecture/page"
    createdAt: integer("created_at", { mode: "timestamp" })
      .notNull()
      .default(sql`(unixepoch())`),
    updatedAt: integer("updated_at", { mode: "timestamp" })
      .notNull()
      .default(sql`(unixepoch())`),
  },
  (table) => [
    index("idx_pages_lecture").on(table.lectureId),
    uniqueIndex("uq_pages_lecture_slug").on(table.lectureId, table.slug),
    uniqueIndex("uq_pages_key").on(table.key),
  ],
);

// slides
export const slides = sqliteTable(
  "slides",
  {
    id: integer().primaryKey({ autoIncrement: true }),
    pageId: integer("page_id")
      .notNull()
      .references(() => pages.id, { onDelete: "cascade" }),
    idx: integer().notNull(), // Slide.index
    url: text().notNull(),
    pdfPath: text("pdf_path"),
    downloadedAt: integer("downloaded_at", { mode: "timestamp" })
      .notNull()
      .default(sql`(unixepoch())`),
  },
  (table) => [
    index("idx_slides_page").on(table.pageId),
    uniqueIndex("uq_slides_page_idx").on(table.pageId, table.idx),
  ],
);

export default defineConfig({
  dialect: "sqlite",
  schema: "./drizzle.config.ts",
  out: "./src-tauri/migrations",
});
